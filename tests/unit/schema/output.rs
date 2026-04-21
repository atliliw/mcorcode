//! Output 类型单元测试

use mcorcode::schema::{FinishReason, LlmOutput, TokenUsage, ToolCall};

/// 测试 FinishReason 的 as_str 方法
/// 每种完成原因返回正确的字符串表示
#[test]
fn test_finish_reason_as_str() {
    assert_eq!(FinishReason::Stop.as_str(), "stop");
    assert_eq!(FinishReason::ToolCalls.as_str(), "tool_calls");
    assert_eq!(FinishReason::Length.as_str(), "length");
    assert_eq!(FinishReason::ContentFilter.as_str(), "content_filter");
    assert_eq!(FinishReason::Error.as_str(), "error");
}

/// 测试 FinishReason 的 Display trait 实现
/// to_string 返回值应与 as_str 相同
#[test]
fn test_finish_reason_display() {
    assert_eq!(FinishReason::Stop.to_string(), "stop");
    assert_eq!(FinishReason::ToolCalls.to_string(), "tool_calls");
}

/// 测试 FinishReason 的 is_stop 方法
/// Stop 应返回 true，其他返回 false
#[test]
fn test_finish_reason_is_stop() {
    assert!(FinishReason::Stop.is_stop());
    assert!(!FinishReason::ToolCalls.is_stop());
    assert!(!FinishReason::Length.is_stop());
}

/// 测试 FinishReason 的 needs_tool_execution 方法
/// ToolCalls 应返回 true，其他返回 false
#[test]
fn test_finish_reason_needs_tool_execution() {
    assert!(FinishReason::ToolCalls.needs_tool_execution());
    assert!(!FinishReason::Stop.needs_tool_execution());
    assert!(!FinishReason::Length.needs_tool_execution());
}

/// 测试 TokenUsage 的创建
/// new 应正确计算 total_tokens
#[test]
fn test_token_usage_new() {
    let usage = TokenUsage::new(100, 50);
    assert_eq!(usage.prompt_tokens, 100);
    assert_eq!(usage.completion_tokens, 50);
    assert_eq!(usage.total_tokens, 150);
}

/// 测试 TokenUsage 的 zero 方法
/// zero 应创建所有字段为 0 的实例
#[test]
fn test_token_usage_zero() {
    let usage = TokenUsage::zero();
    assert_eq!(usage.prompt_tokens, 0);
    assert_eq!(usage.completion_tokens, 0);
    assert_eq!(usage.total_tokens, 0);
}

/// 测试 LlmOutput 的文本响应创建
/// text 应创建 Stop 类型的输出
#[test]
fn test_llm_output_text() {
    let output = LlmOutput::text("Hello, world!");
    assert_eq!(output.content, "Hello, world!");
    assert!(output.tool_calls.is_none());
    assert_eq!(output.finish_reason, FinishReason::Stop);
    assert!(output.usage.is_none());
    assert!(output.is_finished());
    assert!(!output.has_tool_calls());
}

/// 测试 LlmOutput 的工具调用创建
/// with_tools 应创建 ToolCalls 类型的输出
#[test]
fn test_llm_output_with_tools() {
    let tool_call = ToolCall::new("call_123", "bash", serde_json::json!({"command": "ls"}));
    let output = LlmOutput::with_tools("Executing", vec![tool_call]);

    assert_eq!(output.content, "Executing");
    assert!(output.tool_calls.is_some());
    assert_eq!(output.finish_reason, FinishReason::ToolCalls);
    assert!(output.has_tool_calls());
    assert!(!output.is_finished());
    assert!(output.finish_reason.needs_tool_execution());
}

/// 测试 LlmOutput 的 with_usage builder 方法
/// with_usage 应设置 token 使用统计
#[test]
fn test_llm_output_with_usage() {
    let output = LlmOutput::text("Test").with_usage(TokenUsage::new(10, 5));
    assert!(output.usage.is_some());
    assert_eq!(output.usage.unwrap().total_tokens, 15);
}

/// 测试 LlmOutput 的 get_tool_calls 方法
/// 无工具调用时应返回空切片
#[test]
fn test_llm_output_get_tool_calls_empty() {
    let output = LlmOutput::text("No tools");
    assert!(output.get_tool_calls().is_empty());
}

/// 测试 LlmOutput 的 get_tool_calls 方法
/// 有工具调用时应返回工具列表
#[test]
fn test_llm_output_get_tool_calls() {
    let tc1 = ToolCall::new("call_1", "bash", serde_json::json!({}));
    let tc2 = ToolCall::new("call_2", "read", serde_json::json!({}));
    let output = LlmOutput::with_tools("Running", vec![tc1.clone(), tc2.clone()]);

    let calls = output.get_tool_calls();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].name, "bash");
    assert_eq!(calls[1].name, "read");
}

/// 测试 FinishReason 的序列化
/// Stop 应序列化为 "stop"
#[test]
fn test_finish_reason_serialization() {
    let reason = FinishReason::Stop;
    let json = serde_json::to_string(&reason).unwrap();
    assert_eq!(json, "\"stop\"");
}

/// 测试 FinishReason 的反序列化
/// "tool_calls" 应反序列化为 ToolCalls
#[test]
fn test_finish_reason_deserialization() {
    let reason: FinishReason = serde_json::from_str("\"tool_calls\"").unwrap();
    assert_eq!(reason, FinishReason::ToolCalls);
}

/// 测试 TokenUsage 的序列化
/// 应正确序列化所有字段
#[test]
fn test_token_usage_serialization() {
    let usage = TokenUsage::new(100, 200);
    let json = serde_json::to_string(&usage).unwrap();
    assert!(json.contains("\"prompt_tokens\":100"));
    assert!(json.contains("\"completion_tokens\":200"));
    assert!(json.contains("\"total_tokens\":300"));
}

/// 测试 LlmOutput 的序列化
/// content 和 finish_reason 应正确序列化
#[test]
fn test_llm_output_serialization() {
    let output = LlmOutput::text("Test response");
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"content\":\"Test response\""));
    assert!(json.contains("\"finish_reason\":\"stop\""));
}
