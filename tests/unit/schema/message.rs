//! Message 类型单元测试

use mcorcode::{Message, MessageType, ToolCall};

/// 测试所有消息类型的创建和角色字符串
/// 包括 system、human、ai、tool 四种类型
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

/// 测试用户消息的 JSON 序列化
/// 验证 role 字段序列化为 "user"，content 字段正确保留
#[test]
fn test_message_serialization() {
    let msg = Message::human("Test message");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"user\""));
    assert!(json.contains("\"content\":\"Test message\""));
}

/// 测试系统消息的 JSON 序列化
/// 验证 role 字段序列化为 "system"
#[test]
fn test_message_system_serialization() {
    let msg = Message::system("You are helpful");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"system\""));
}

/// 测试 AI 消息的 JSON 序列化
/// 验证 role 字段序列化为 "assistant"
#[test]
fn test_message_ai_serialization() {
    let msg = Message::ai("Response text");
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"assistant\""));
}

/// 测试 ToolCall 结构体的创建
/// 验证 id、name、arguments 字段正确设置
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

/// 测试 ToolCall 的 JSON 序列化
/// 验证 id 和 name 字段正确序列化
#[test]
fn test_tool_call_serialization() {
    let tc = ToolCall::new("call_abc", "bash", serde_json::json!({"command": "ls -la"}));
    let json = serde_json::to_string(&tc).unwrap();
    assert!(json.contains("\"id\":\"call_abc\""));
    assert!(json.contains("\"name\":\"bash\""));
}

/// 测试 Message 的 has_tool_calls 方法
/// 普通 AI 消息返回 false，带工具调用的 AI 消息返回 true
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

/// 测试 MessageType 的 as_str 方法
/// 每种消息类型返回正确的字符串表示
#[test]
fn test_message_type_as_str() {
    assert_eq!(MessageType::System.as_str(), "system");
    assert_eq!(MessageType::Human.as_str(), "user");
    assert_eq!(MessageType::AI.as_str(), "assistant");
    assert_eq!(MessageType::Tool.as_str(), "tool");
}

/// 测试 MessageType 的 Display trait 实现
/// to_string 返回值应与 as_str 相同
#[test]
fn test_message_type_display() {
    assert_eq!(MessageType::System.to_string(), "system");
    assert_eq!(MessageType::Human.to_string(), "user");
    assert_eq!(MessageType::AI.to_string(), "assistant");
    assert_eq!(MessageType::Tool.to_string(), "tool");
}
