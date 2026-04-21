//! OpenAI API Adapter

use async_trait::async_trait;
use futures::stream::{StreamExt, BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::llm::types::{BaseChatModel, LlmError, ToolDefinition};
use crate::schema::{Message, LlmOutput, FinishReason, TokenUsage, ToolCall};

/// OpenAI API 配置
#[derive(Debug, Clone)]
pub struct OpenAIClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
}

impl Default for OpenAIClientConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(4096),
        }
    }
}

impl OpenAIClientConfig {
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

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = Some(max);
        self
    }
}

/// OpenAI Chat 模型适配器
pub struct OpenAIClient {
    client: Client,
    config: OpenAIClientConfig,
    tools: Vec<ToolDefinition>,
}

impl OpenAIClient {
    pub fn new(config: OpenAIClientConfig) -> Self {
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

    fn convert_messages(messages: Vec<Message>) -> Vec<InternalMessage> {
        messages.into_iter().map(|m| {
            let role = m.role();
            InternalMessage {
                role: role.to_string(),
                content: Some(m.content),
                tool_calls: m.tool_calls.clone().map(|tc| tc.into_iter().map(|t| InternalToolCall {
                    id: t.id,
                    r#type: "function".to_string(),
                    function: InternalFunctionCall {
                        name: t.name,
                        arguments: t.arguments.to_string(),
                    },
                }).collect()),
                tool_call_id: m.tool_call_id.clone(),
            }
        }).collect()
    }

    fn parse_response(response: InternalResponse) -> Result<LlmOutput, LlmError> {
        let choice = response.choices.into_iter().next()
            .ok_or_else(|| LlmError::InvalidResponse("No choices in response".to_string()))?;

        let finish_reason = match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("length") => FinishReason::Length,
            Some("content_filter") => FinishReason::ContentFilter,
            _ => FinishReason::Error,
        };

        let tool_calls = choice.message.tool_calls.map(|tc| {
            tc.into_iter().map(|t| {
                let args: serde_json::Value = serde_json::from_str(&t.function.arguments)
                    .unwrap_or_else(|_| serde_json::json!({}));
                ToolCall::new(t.id, t.function.name, args)
            }).collect()
        });

        let usage = response.usage.map(|u| TokenUsage::new(u.prompt_tokens, u.completion_tokens));

        Ok(LlmOutput {
            content: choice.message.content.unwrap_or_default(),
            tool_calls,
            finish_reason,
            usage,
        })
    }
}

#[async_trait]
impl BaseChatModel for OpenAIClient {
    async fn chat(&self, messages: Vec<Message>) -> Result<LlmOutput, LlmError> {
        let openai_messages = Self::convert_messages(messages);

        let tools_json: Option<Vec<serde_json::Value>> = if self.tools.is_empty() {
            None
        } else {
            Some(self.tools.iter().map(|t| t.to_openai_format()).collect())
        };

        let request = InternalRequest {
            model: self.config.model.clone(),
            messages: openai_messages,
            tools: tools_json,
            tool_choice: if self.tools.is_empty() { None } else { Some("auto".to_string()) },
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            stream: Some(false),
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
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
        let openai_messages = Self::convert_messages(messages);

        let request = InternalRequest {
            model: self.config.model.clone(),
            messages: openai_messages,
            tools: None,
            tool_choice: None,
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
            stream: Some(true),
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
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
                        if data == "[DONE]" {
                            return None;
                        }
                        if let Ok(parsed) = serde_json::from_str::<InternalStreamResponse>(data) {
                            if let Some(choice) = parsed.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    return Some(content.clone());
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
    messages: Vec<InternalMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct InternalMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<InternalToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct InternalToolCall {
    id: String,
    #[serde(rename = "type")]
    r#type: String,
    function: InternalFunctionCall,
}

#[derive(Serialize, Deserialize)]
struct InternalFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Deserialize)]
struct InternalResponse {
    choices: Vec<InternalChoice>,
    #[serde(default)]
    usage: Option<InternalUsage>,
}

#[derive(Deserialize)]
struct InternalChoice {
    message: InternalMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct InternalUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

#[derive(Deserialize)]
struct InternalStreamResponse {
    choices: Vec<InternalStreamChoice>,
}

#[derive(Deserialize)]
struct InternalStreamChoice {
    delta: InternalStreamDelta,
}

#[derive(Deserialize)]
struct InternalStreamDelta {
    #[serde(default)]
    content: Option<String>,
}