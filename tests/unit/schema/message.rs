//! Unit tests for Message types

use mcorcode::{Message, MessageType, ToolCall};

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

    let tool = Message::tool("call_123", "Result");
    assert_eq!(tool.r#type, MessageType::Tool);
    assert_eq!(tool.role(), "tool");
}

#[test]
fn test_message_serialization() {
    let msg = Message::human("Test message");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"user\""));
    assert!(json.contains("\"content\":\"Test message\""));
}

#[test]
fn test_message_system_serialization() {
    let msg = Message::system("You are helpful");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"system\""));
}

#[test]
fn test_message_ai_serialization() {
    let msg = Message::ai("Response text");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"assistant\""));
}

#[test]
fn test_tool_call_creation() {
    let tc = ToolCall::new(
        "call_123",
        "read_file",
        serde_json::json!({"path": "test.txt"}),
    );
    assert_eq!(tc.id, "call_123");
    assert_eq!(tc.name, "read_file");
    assert!(tc.arguments.is_object());
}

#[test]
fn test_tool_call_serialization() {
    let tc = ToolCall::new("call_abc", "bash", serde_json::json!({"command": "ls -la"}));
    let json = serde_json::to_string(&tc).unwrap();
    assert!(json.contains("\"id\":\"call_abc\""));
    assert!(json.contains("\"name\":\"bash\""));
}

#[test]
fn test_message_has_tool_calls() {
    let msg = Message::ai("Hello");
    assert!(!msg.has_tool_calls());

    let msg_with_tools = Message::ai_with_tool_calls(
        "Executing",
        vec![ToolCall::new("call_1", "bash", serde_json::json!({}))],
    );
    assert!(msg_with_tools.has_tool_calls());
}

#[test]
fn test_message_type_as_str() {
    assert_eq!(MessageType::System.as_str(), "system");
    assert_eq!(MessageType::Human.as_str(), "user");
    assert_eq!(MessageType::AI.as_str(), "assistant");
    assert_eq!(MessageType::Tool.as_str(), "tool");
}

#[test]
fn test_message_type_display() {
    assert_eq!(MessageType::System.to_string(), "system");
    assert_eq!(MessageType::Human.to_string(), "user");
    assert_eq!(MessageType::AI.to_string(), "assistant");
    assert_eq!(MessageType::Tool.to_string(), "tool");
}
