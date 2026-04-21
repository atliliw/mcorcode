//! Anthropic Claude API Adapter

use async_trait::async_trait;
use futures::stream::{StreamExt, BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::llm::types::{BaseChatModel, LlmError, ToolDefinition};
use crate::schema::{Message, LlmOutput, FinishReason, TokenUsage, ToolCall, MessageType};

/// Anthropic 配置
#[derive(Debug, Clone)]
pub struct AnthropicClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: usize,
}

impl Default for AnthropicClientConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
        }
    }
}

impl AnthropicClientConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = max;
        self
    }
}

/// Anthropic Claude 模型适配器
pub struct AnthropicClient {
    client: Client,
    config: AnthropicClientConfig,
    tools: Vec<ToolDefinition>,
}

impl AnthropicClient {
    pub fn new(config: AnthropicClientConfig) -> Self {
        Self {
            client: Client::new(),
            config,
            tools: Vec::new(),
        }
    }

    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = tools;
        self
    }

    fn convert_messages(messages: Vec<Message>) -> (Option<String>, Vec<InternalMessage>) {
        let system_message = messages.iter()
            .find(|m| m.is_system())
            .map(|m| m.content.clone());

        let other_messages: Vec<InternalMessage> = messages.iter()
            .filter(|m| !m.is_system())
            .map(|m| {
                let role = match m.r#type {
                    MessageType::Human => "user",
                    MessageType::AI => "assistant",
                    MessageType::Tool => "user",
                    _ => "user",
                };

                if m.r#type == MessageType::Tool {
                    InternalMessage {
                        role: role.to_string(),
                        content: InternalContent::ToolResult(
                            m.tool_call_id.clone().unwrap_or_default(),
                            m.content.clone(),
                        ),
                    }
                } else if m.has_tool_calls() {
                    InternalMessage {
                        role: role.to_string(),
                        content: InternalContent::ToolUse(
                            m.content.clone(),
                            m.tool_calls.clone().unwrap_or_default(),
                        ),
                    }
                } else {
                    InternalMessage {
                        role: role.to_string(),
                        content: InternalContent::Text(m.content.clone()),
                    }
                }
            })
            .collect();

        (system_message, other_messages)
    }

    fn parse_response(response: InternalResponse) -> Result<LlmOutput, LlmError> {
        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") => FinishReason::Stop,
            Some("tool_use") => FinishReason::ToolCalls,
            Some("max_tokens") => FinishReason::Length,
            _ => FinishReason::Stop,
        };

        let mut content_text = String::new();
        let mut tool_calls: Vec<ToolCall> = Vec::new();

        for block in response.content {
            match block.r#type.as_str() {
                "text" => {
                    if let Some(text) = block.text {
                        content_text.push_str(&text);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name), Some(input)) = (block.id.clone(), block.name.clone(), block.input.clone()) {
                        tool_calls.push(ToolCall::new(id, name, input));
                    }
                }
                _ => {}
            }
        }

        let usage = TokenUsage::new(response.usage.input_tokens, response.usage.output_tokens);

        Ok(LlmOutput {
            content: content_text,
            tool_calls: if tool_calls.is_empty() { None } else { Some(tool_calls) },
            finish_reason,
            usage: Some(usage),
        })
    }
}

#[async_trait]
impl BaseChatModel for AnthropicClient {
    async fn chat(&self, messages: Vec<Message>) -> Result<LlmOutput, LlmError> {
        let (system, anthropic_messages) = Self::convert_messages(messages);

        let tools_json: Option<Vec<serde_json::Value>> = if self.tools.is_empty() {
            None
        } else {
            Some(self.tools.iter().map(|t| t.to_anthropic_format()).collect())
        };

        let request = InternalRequest {
            model: self.config.model.clone(),
            max_tokens: self.config.max_tokens,
            system,
            messages: anthropic_messages,
            tools: tools_json,
        };

        let response = self.client
            .post(format!("{}/messages", self.config.base_url))
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError { code: status, message: error_text });
        }

        let response_body: InternalResponse = response
            .json()
            .await
            .map_err(|e| LlmError::InvalidResponse(e.to_string()))?;

        Self::parse_response(response_body)
    }

    async fn stream_chat(&self, messages: Vec<Message>) -> Result<BoxStream<'static, String>, LlmError> {
        let (system, anthropic_messages) = Self::convert_messages(messages);

        let request = InternalStreamRequest {
            model: self.config.model.clone(),
            max_tokens: self.config.max_tokens,
            system,
            messages: anthropic_messages,
            stream: true,
        };

        let response = self.client
            .post(format!("{}/messages", self.config.base_url))
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            return Err(LlmError::ApiError { code: status, message: "Stream request failed".to_string() });
        }

        let stream = response.bytes_stream()
            .filter_map(|chunk| async move {
                let bytes = chunk.ok()?;
                let text = String::from_utf8_lossy(&bytes);
                
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if let Ok(parsed) = serde_json::from_str::<InternalStreamEvent>(data) {
                            if parsed.type_field == "content_block_delta" {
                                if let Some(delta) = parsed.delta {
                                    if delta.type_field == "text_delta" {
                                        if let Some(text) = delta.text {
                                            return Some(text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None
            })
            .boxed();

        Ok(stream)
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }

    fn get_tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.clone()
    }

    fn set_tools(&mut self, tools: Vec<ToolDefinition>) {
        self.tools = tools;
    }
}

#[derive(Serialize)]
struct InternalRequest {
    model: String,
    max_tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<InternalMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize)]
struct InternalStreamRequest {
    model: String,
    max_tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<InternalMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct InternalMessage {
    role: String,
    content: InternalContent,
}

enum InternalContent {
    Text(String),
    ToolUse(String, Vec<ToolCall>),
    ToolResult(String, String),
}

impl Serialize for InternalContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        match self {
            InternalContent::Text(text) => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", text)?;
            }
            InternalContent::ToolUse(content, tool_calls) => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", content)?;
                if let Some(first) = tool_calls.first() {
                    map.serialize_entry("tool_use_id", &first.id)?;
                }
            }
            InternalContent::ToolResult(tool_use_id, content) => {
                map.serialize_entry("type", "tool_result")?;
                map.serialize_entry("tool_use_id", tool_use_id)?;
                map.serialize_entry("content", content)?;
            }
        }
        map.end()
    }
}

#[derive(Deserialize)]
struct InternalResponse {
    content: Vec<InternalContentBlock>,
    #[serde(default)]
    stop_reason: Option<String>,
    usage: InternalUsage,
}

#[derive(Deserialize)]
struct InternalContentBlock {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    input: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct InternalUsage {
    input_tokens: usize,
    output_tokens: usize,
}

#[derive(Deserialize)]
struct InternalStreamEvent {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(default)]
    delta: Option<InternalDelta>,
}

#[derive(Deserialize)]
struct InternalDelta {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(default)]
    text: Option<String>,
}