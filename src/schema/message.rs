//! Message types for LLM communication
//!
//! Reference: langchainrust/langchainrust/src/schema/message.rs

use serde::{Deserialize, Serialize};

/// Message type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    Human,
    #[serde(rename = "assistant")]
    AI,
    #[serde(rename = "tool")]
    Tool,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::System => "system",
            MessageType::Human => "user",
            MessageType::AI => "assistant",
            MessageType::Tool => "tool",
        }
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Message structure for LLM communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "role")]
    pub r#type: MessageType,

    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            r#type: MessageType::System,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }
    }

    pub fn human(content: impl Into<String>) -> Self {
        Self {
            r#type: MessageType::Human,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }
    }

    pub fn ai(content: impl Into<String>) -> Self {
        Self {
            r#type: MessageType::AI,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }
    }

    pub fn ai_with_tool_calls(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            r#type: MessageType::AI,
            content: content.into(),
            tool_calls: Some(tool_calls),
            tool_call_id: None,
            name: None,
        }
    }

    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            r#type: MessageType::Tool,
            content: content.into(),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
            name: None,
        }
    }

    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls.as_ref().map_or(false, |tc| !tc.is_empty())
    }

    pub fn role(&self) -> &'static str {
        self.r#type.as_str()
    }

    /// Convert to langchainrust Message type
    pub fn to_langchain(&self) -> langchainrust::schema::Message {
        use langchainrust::core::tools::ToolCall as LcToolCall;
        use langchainrust::schema::MessageType as LcMessageType;

        match self.r#type {
            MessageType::System => langchainrust::schema::Message::system(&self.content),
            MessageType::Human => langchainrust::schema::Message::human(&self.content),
            MessageType::AI => {
                if let Some(tool_calls) = &self.tool_calls {
                    let lc_tool_calls: Vec<LcToolCall> = tool_calls
                        .iter()
                        .map(|tc| LcToolCall::new(&tc.id, &tc.name, tc.arguments.to_string()))
                        .collect();
                    langchainrust::schema::Message::ai_with_tool_calls(&self.content, lc_tool_calls)
                } else {
                    langchainrust::schema::Message::ai(&self.content)
                }
            }
            MessageType::Tool => langchainrust::schema::Message::tool(
                self.tool_call_id.clone().unwrap_or_default(),
                &self.content,
            ),
        }
    }
}

/// Tool call structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique tool call ID
    pub id: String,

    /// Tool name
    pub name: String,

    /// Tool arguments (JSON)
    pub arguments: serde_json::Value,
}

impl ToolCall {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        arguments: serde_json::Value,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            arguments,
        }
    }
}
