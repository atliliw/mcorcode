//! Unit tests for HookAction and HookResult

use mcorcode::{HookAction, HookResult};

#[test]
fn test_hook_result_is_denied() {
    assert!(HookResult::Denied("reason".to_string()).is_denied());
    assert!(!HookResult::Approved.is_denied());
    assert!(!HookResult::Continue.is_denied());
    assert!(!HookResult::AskUser.is_denied());
}

#[test]
fn test_hook_result_is_approved() {
    assert!(HookResult::Approved.is_approved());
    assert!(!HookResult::Continue.is_approved());
    assert!(!HookResult::Denied("reason".to_string()).is_approved());
    assert!(!HookResult::AskUser.is_approved());
}

#[test]
fn test_hook_result_should_ask_user() {
    assert!(HookResult::AskUser.should_ask_user());
    assert!(!HookResult::Approved.should_ask_user());
    assert!(!HookResult::Continue.should_ask_user());
    assert!(!HookResult::Denied("reason".to_string()).should_ask_user());
}

#[test]
fn test_hook_result_continue() {
    let result = HookResult::Continue;
    assert!(!result.is_denied());
    assert!(!result.is_approved());
    assert!(!result.should_ask_user());
}

#[test]
fn test_hook_action_auto_approve() {
    let action = HookAction::AutoApprove;
    assert!(matches!(action, HookAction::AutoApprove));
}

#[test]
fn test_hook_action_auto_deny() {
    let action = HookAction::AutoDeny {
        reason: "test reason".to_string(),
    };
    if let HookAction::AutoDeny { reason } = action {
        assert_eq!(reason, "test reason");
    } else {
        panic!("Expected AutoDeny variant");
    }
}

#[test]
fn test_hook_action_ask_user() {
    let action = HookAction::AskUser;
    assert!(matches!(action, HookAction::AskUser));
}

#[test]
fn test_hook_action_validate_input() {
    let schema = serde_json::json!({"type": "object"});
    let action = HookAction::ValidateInput {
        schema: schema.clone(),
    };
    if let HookAction::ValidateInput { schema: s } = action {
        assert_eq!(s, schema);
    }
}

#[test]
fn test_hook_action_run_command() {
    let action = HookAction::RunCommand {
        cmd: "ls -la".to_string(),
    };
    if let HookAction::RunCommand { cmd } = action {
        assert_eq!(cmd, "ls -la");
    }
}

#[test]
fn test_hook_action_log_to_file() {
    let action = HookAction::LogToFile {
        path: "/tmp/log.txt".to_string(),
    };
    if let HookAction::LogToFile { path } = action {
        assert_eq!(path, "/tmp/log.txt");
    }
}
