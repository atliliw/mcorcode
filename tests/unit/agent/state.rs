//! Unit tests for McorcodeState and related types

use mcorcode::agent::{McorcodeState, MessageRole, StateMessage, StateStep};
use mcorcode::schema::ToolCall;

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

#[test]
fn test_mcorcode_state_has_tool_calls() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.has_tool_calls());
}

#[test]
fn test_mcorcode_state_reached_max_iterations() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert!(!state.reached_max_iterations());

    state.iteration = 25;
    assert!(state.reached_max_iterations());

    state.iteration = 26;
    assert!(state.reached_max_iterations());
}

#[test]
fn test_mcorcode_state_has_final_output() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.has_final_output());
}

#[test]
fn test_mcorcode_state_has_final_output_true() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.final_output = Some("Done".to_string());
    assert!(state.has_final_output());
}

#[test]
fn test_mcorcode_state_add_system() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_system("You are helpful".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[0].role, MessageRole::System);
    assert_eq!(state.messages[1].role, MessageRole::Human);
}

#[test]
fn test_mcorcode_state_add_human() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_human("How are you?".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].role, MessageRole::Human);
}

#[test]
fn test_mcorcode_state_add_ai() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_ai("I'm fine".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].role, MessageRole::AI);
}

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

#[test]
fn test_mcorcode_state_add_tool_result() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.add_tool_result("call_123".to_string(), "Output".to_string());
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].tool_call_id, Some("call_123".to_string()));
}

#[test]
fn test_mcorcode_state_increment_iteration() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert_eq!(state.iteration, 0);

    state.increment_iteration();
    assert_eq!(state.iteration, 1);

    state.increment_iteration();
    assert_eq!(state.iteration, 2);
}

#[test]
fn test_mcorcode_state_set_error() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.set_error("Something went wrong".to_string());

    assert!(state.error.is_some());
    assert_eq!(state.error, Some("Something went wrong".to_string()));
    assert!(!state.should_continue);
}

#[test]
fn test_mcorcode_state_finish() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.finish("Done".to_string());

    assert_eq!(state.final_output, Some("Done".to_string()));
    assert!(!state.should_continue);
}

#[test]
fn test_mcorcode_state_message_count() {
    let mut state = McorcodeState::new("Hello".to_string());
    assert_eq!(state.message_count(), 1);

    state.add_ai("Hi".to_string());
    assert_eq!(state.message_count(), 2);

    state.add_human("How are you?".to_string());
    assert_eq!(state.message_count(), 3);
}

#[test]
fn test_state_message_system() {
    let msg = StateMessage::system("System message".to_string());
    assert_eq!(msg.role, MessageRole::System);
    assert_eq!(msg.content, "System message");
    assert!(msg.tool_calls.is_none());
    assert!(msg.tool_call_id.is_none());
}

#[test]
fn test_state_message_human() {
    let msg = StateMessage::human("User message".to_string());
    assert_eq!(msg.role, MessageRole::Human);
    assert_eq!(msg.content, "User message");
}

#[test]
fn test_state_message_ai() {
    let msg = StateMessage::ai("AI response".to_string());
    assert_eq!(msg.role, MessageRole::AI);
    assert_eq!(msg.content, "AI response");
    assert!(msg.tool_calls.is_none());
}

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

#[test]
fn test_state_message_tool() {
    let msg = StateMessage::tool("call_123".to_string(), "Tool result".to_string());
    assert_eq!(msg.role, MessageRole::Tool);
    assert_eq!(msg.tool_call_id, Some("call_123".to_string()));
    assert_eq!(msg.content, "Tool result");
}

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

#[test]
fn test_state_should_compact() {
    let state = McorcodeState::new("Hello".to_string());
    assert!(!state.should_compact());
}

#[test]
fn test_state_should_compact_true() {
    let mut state = McorcodeState::new("Hello".to_string());
    state.needs_compact = true;
    assert!(state.should_compact());
}

#[test]
fn test_message_role_equality() {
    assert_eq!(MessageRole::System, MessageRole::System);
    assert_eq!(MessageRole::Human, MessageRole::Human);
    assert_ne!(MessageRole::AI, MessageRole::Human);
    assert_ne!(MessageRole::Tool, MessageRole::System);
}
