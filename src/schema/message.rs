//! Message types for LLM communication
//!
//! Reference: langchainrust/langchainrust/src/schema/message.rs

use serde::{Deserialize, Serialize};

/// Message type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    System,
    Human,
    AI,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_types() {
        let system = Message::system("You are a helpful assistant.");
        assert_eq!(system.r#type, MessageType::System);
        assert_eq!(system.role(), "system");

        let human = Message::human("Hello!");
        assert_eq!(human.r#type, MessageType::Human);
        assert_eq!(human.role(), "user");

        let ai = Message::ai("Hi there!");
        assert_eq!(ai.r#type, MessageType::AI);
        assert_eq!(ai.role(), "assistant");
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::human("Test message");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"Test message\""));
    }

    #[test]
    fn test_tool_call() {
        let tc = ToolCall::new(
            "call_123",
            "read_file",
            serde_json::json!({"path": "test.txt"}),
        );
        assert_eq!(tc.id, "call_123");
        assert_eq!(tc.name, "read_file");
    }
}
