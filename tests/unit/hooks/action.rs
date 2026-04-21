//! HookAction 和 HookResult 枚举单元测试

use mcorcode::{HookAction, HookResult};

/// 测试 HookResult 的 is_denied 方法
/// 只有 Denied 变体应返回 true
#[test]
fn test_hook_result_is_denied() {
    assert!(HookResult::Denied("reason".to_string()).is_denied());
    assert!(!HookResult::Approved.is_denied());
    assert!(!HookResult::Continue.is_denied());
    assert!(!HookResult::AskUser.is_denied());
}

/// 测试 HookResult 的 is_approved 方法
/// 只有 Approved 变体应返回 true
#[test]
fn test_hook_result_is_approved() {
    assert!(HookResult::Approved.is_approved());
    assert!(!HookResult::Continue.is_approved());
    assert!(!HookResult::Denied("reason".to_string()).is_approved());
    assert!(!HookResult::AskUser.is_approved());
}

/// 测试 HookResult 的 should_ask_user 方法
/// 只有 AskUser 变体应返回 true
#[test]
fn test_hook_result_should_ask_user() {
    assert!(HookResult::AskUser.should_ask_user());
    assert!(!HookResult::Approved.should_ask_user());
    assert!(!HookResult::Continue.should_ask_user());
    assert!(!HookResult::Denied("reason".to_string()).should_ask_user());
}

/// 测试 HookResult::Continue 的行为
/// Continue 不应匹配任何特殊结果检查
#[test]
fn test_hook_result_continue() {
    let result = HookResult::Continue;
    assert!(!result.is_denied());
    assert!(!result.is_approved());
    assert!(!result.should_ask_user());
}

/// 测试 AutoApprove 动作的创建
/// AutoApprove 变体应可构造
#[test]
fn test_hook_action_auto_approve() {
    let action = HookAction::AutoApprove;
    assert!(matches!(action, HookAction::AutoApprove));
}

/// 测试带原因的 AutoDeny 动作创建
/// AutoDeny 应存储提供的原因字符串
#[test]
fn test_hook_action_auto_deny() {
    let action = HookAction::AutoDeny {
        reason: "test reason".to_string(),
    };
    if let HookAction::AutoDeny { reason } = action {
        assert_eq!(reason, "test reason");
    } else {
        panic!("预期 AutoDeny 变体");
    }
}

/// 测试 AskUser 动作的创建
/// AskUser 变体应可构造
#[test]
fn test_hook_action_ask_user() {
    let action = HookAction::AskUser;
    assert!(matches!(action, HookAction::AskUser));
}

/// 测试带 JSON schema 的 ValidateInput 动作
/// ValidateInput 应存储验证 schema
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

/// 测试带命令字符串的 RunCommand 动作
/// RunCommand 应存储要执行的命令
#[test]
fn test_hook_action_run_command() {
    let action = HookAction::RunCommand {
        cmd: "ls -la".to_string(),
    };
    if let HookAction::RunCommand { cmd } = action {
        assert_eq!(cmd, "ls -la");
    }
}

/// 测试带文件路径的 LogToFile 动作
/// LogToFile 应存储目标日志文件路径
#[test]
fn test_hook_action_log_to_file() {
    let action = HookAction::LogToFile {
        path: "/tmp/log.txt".to_string(),
    };
    if let HookAction::LogToFile { path } = action {
        assert_eq!(path, "/tmp/log.txt");
    }
}
