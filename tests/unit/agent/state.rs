//! McorcodeState 和相关类型单元测试

use mcorcode::agent::{McorcodeState, MessageRole, StateMessage, StateStep};
use mcorcode::schema::ToolCall;

/// 测试 McorcodeState 的创建
/// 新状态应有唯一 session_id、一条消息、初始化计数器
#[test]
fn test_mcorcode_state_new() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.session_id.is_empty());
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.iteration, 0);
    assert_eq!(state.max_iterations, 25);
    assert!(state.should_continue);
    assert!(state.tool_calls_queue.is_empty());
    assert!(state.final_output.is_none());
    assert!(state.error.is_none());
}

/// 测试 McorcodeState 的恢复功能
/// 可从已有 session_id 和消息列表恢复状态
#[test]
fn test_mcorcode_state_resume() {
    let session_id = "test-session-123".to_string();
    let messages = vec![
        StateMessage::human("Hello".to_string()),
        StateMessage::ai("Hi".to_string()),
    ];
    let state = McorcodeState::resume(session_id.clone(), messages);
    assert_eq!(state.session_id, session_id);
    assert_eq!(state.messages.len(), 2);
}

/// 测试 has_tool_calls 方法
/// 新状态无工具调用队列时应返回 false
#[test]
fn test_mcorcode_state_has_tool_calls() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.has_tool_calls());
}

/// 测试 reached_max_iterations 方法
/// 迭代次数达到最大值时应返回 true
#[test]
fn test_mcorcode_state_reached_max_iterations() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert!(!state.reached_max_iterations());

    state.iteration = 25;
    assert!(state.reached_max_iterations());

    state.iteration = 26;
    assert!(state.reached_max_iterations());
}

/// 测试 has_final_output 方法（无输出）
/// 无最终输出时应返回 false
#[test]
fn test_mcorcode_state_has_final_output() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.has_final_output());
}

/// 测试 has_final_output 方法（有输出）
/// 有最终输出时应返回 true
#[test]
fn test_mcorcode_state_has_final_output_true() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.final_output = Some("Done".to_string());
    assert!(state.has_final_output());
}

/// 测试 add_system 方法
/// 系统消息应插入到消息列表开头
#[test]
fn test_mcorcode_state_add_system() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_system("You are helpful".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[0].role, MessageRole::System);
    assert_eq!(state.messages[1].role, MessageRole::Human);
}

/// 测试 add_human 方法
/// 用户消息应追加到消息列表末尾
#[test]
fn test_mcorcode_state_add_human() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_human("How are you?".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].role, MessageRole::Human);
}

/// 测试 add_ai 方法
/// AI 消息应追加到消息列表末尾
#[test]
fn test_mcorcode_state_add_ai() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_ai("I'm fine".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].role, MessageRole::AI);
}

/// 测试 add_ai_with_tools 方法
/// 带工具调用的 AI 消息应包含 tool_calls 字段
#[test]
fn test_mcorcode_state_add_ai_with_tools() {
    let mut state = McorcodeState::new("Hello".to_string());
    let tool_calls = vec![ToolCall::new(
        "call_1",
        "bash",
        serde_json::json!({"cmd": "ls"}),
    )];
    state.add_ai_with_tools("Running command".to_string(), tool_calls);
    assert_eq!(state.messages.len(), 2);
    assert!(state.messages[1].tool_calls.is_some());
}

/// 测试 add_tool_result 方法
/// 工具结果消息应包含 tool_call_id 字段
#[test]
fn test_mcorcode_state_add_tool_result() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_tool_result("call_123".to_string(), "Output".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].tool_call_id, Some("call_123".to_string()));
}

/// 测试 increment_iteration 方法
/// 每次调用迭代计数应增加 1
#[test]
fn test_mcorcode_state_increment_iteration() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert_eq!(state.iteration, 0);

    state.increment_iteration();
    assert_eq!(state.iteration, 1);

    state.increment_iteration();
    assert_eq!(state.iteration, 2);
}

/// 测试 set_error 方法
/// 设置错误后 should_continue 应变为 false
#[test]
fn test_mcorcode_state_set_error() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.set_error("Something went wrong".to_string());

    assert!(state.error.is_some());
    assert_eq!(state.error, Some("Something went wrong".to_string()));
    assert!(!state.should_continue);
}

/// 测试 finish 方法
/// 完成后 final_output 应设置，should_continue 变为 false
#[test]
fn test_mcorcode_state_finish() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.finish("Done".to_string());

    assert_eq!(state.final_output, Some("Done".to_string()));
    assert!(!state.should_continue);
}

/// 测试 message_count 方法
/// 应正确返回当前消息数量
#[test]
fn test_mcorcode_state_message_count() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert_eq!(state.message_count(), 1);

    state.add_ai("Hi".to_string());
    assert_eq!(state.message_count(), 2);

    state.add_human("How are you?".to_string());
    assert_eq!(state.message_count(), 3);
}

/// 测试 StateMessage::system 创建
/// 系统消息角色应为 System
#[test]
fn test_state_message_system() {
    let msg = StateMessage::system("System message".to_string());
    assert_eq!(msg.role, MessageRole::System);
    assert_eq!(msg.content, "System message");
    assert!(msg.tool_calls.is_none());
    assert!(msg.tool_call_id.is_none());
}

/// 测试 StateMessage::human 创建
/// 用户消息角色应为 Human
#[test]
fn test_state_message_human() {
    let msg = StateMessage::human("User message".to_string());
    assert_eq!(msg.role, MessageRole::Human);
    assert_eq!(msg.content, "User message");
}

/// 测试 StateMessage::ai 创建
/// AI 消息角色应为 AI，无工具调用
#[test]
fn test_state_message_ai() {
    let msg = StateMessage::ai("AI response".to_string());
    assert_eq!(msg.role, MessageRole::AI);
    assert_eq!(msg.content, "AI response");
    assert!(msg.tool_calls.is_none());
}

/// 测试 StateMessage::ai_with_tools 创建
/// 带工具调用的 AI 消息应有 tool_calls 字段
#[test]
fn test_state_message_ai_with_tools() {
    let tool_calls = vec![ToolCall::new(
        "call_1",
        "read",
        serde_json::json!({"path": "test.txt"}),
    )];
    let msg = StateMessage::ai_with_tools("Reading file".to_string(), tool_calls.clone());
    assert_eq!(msg.role, MessageRole::AI);
    assert!(msg.tool_calls.is_some());
    assert_eq!(msg.tool_calls.unwrap().len(), 1);
}

/// 测试 StateMessage::tool 创建
/// 工具消息角色应为 Tool，有 tool_call_id
#[test]
fn test_state_message_tool() {
    let msg = StateMessage::tool("call_123".to_string(), "Tool result".to_string());
    assert_eq!(msg.role, MessageRole::Tool);
    assert_eq!(msg.tool_call_id, Some("call_123".to_string()));
    assert_eq!(msg.content, "Tool result");
}

/// 测试 StateStep::new 创建成功步骤
/// 成功步骤的 success 应为 true
#[test]
fn test_state_step_new() {
    let step = StateStep::new(
        "bash".to_string(),
        serde_json::json!({"cmd": "ls"}),
        "output".to_string(),
        100,
    );
    assert_eq!(step.tool, "bash");
    assert!(step.success);
    assert_eq!(step.duration_ms, 100);
    assert!(!step.id.is_empty());
}

/// 测试 StateStep::failed 创建失败步骤
/// 失败步骤的 success 应为 false
#[test]
fn test_state_step_failed() {
    let step = StateStep::failed(
        "bash".to_string(),
        serde_json::json!({"cmd": "invalid"}),
        "Error: command failed".to_string(),
    );
    assert_eq!(step.tool, "bash");
    assert!(!step.success);
    assert_eq!(step.observation, "Error: command failed");
    assert_eq!(step.duration_ms, 0);
}

/// 测试 should_compact 方法（不需要压缩）
/// 新状态不需要压缩时应返回 false
#[test]
fn test_state_should_compact() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.should_compact());
}

/// 测试 should_compact 方法（需要压缩）
/// needs_compact 为 true 时应返回 true
#[test]
fn test_state_should_compact_true() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.needs_compact = true;
    assert!(state.should_compact());
}

/// 测试 MessageRole 的相等性比较
/// 相同角色应相等，不同角色不应相等
#[test]
fn test_message_role_equality() {
    assert_eq!(MessageRole::System, MessageRole::System);
    assert_eq!(MessageRole::Human, MessageRole::Human);
    assert_ne!(MessageRole::AI, MessageRole::Human);
    assert_ne!(MessageRole::Tool, MessageRole::System);
}
