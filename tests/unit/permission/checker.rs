//! PermissionChecker 权限检查器单元测试

use mcorcode::permission::{PermissionAction, PermissionChecker, PermissionMode, PermissionResult};

/// 测试 PermissionResult 的 allow 创建
/// allow() 应创建一个允许的结果
#[test]
fn test_permission_result_allow() {
    let result = PermissionResult::allow();
    assert!(result.is_allowed());
    assert!(!result.is_denied());
    assert!(matches!(result.action, PermissionAction::Allow));
    assert!(result.reason.is_none());
}

/// 测试 PermissionResult 的 deny 创建
/// deny() 应创建一个拒绝的结果，带原因
#[test]
fn test_permission_result_deny() {
    let result = PermissionResult::deny("Dangerous command");
    assert!(result.is_denied());
    assert!(!result.is_allowed());
    assert!(matches!(result.action, PermissionAction::Deny));
    assert_eq!(result.reason, Some("Dangerous command".to_string()));
}

/// 测试 PermissionResult 的 ask_user 创建
/// ask_user() 应创建一个需要用户确认的结果
#[test]
fn test_permission_result_ask_user() {
    let result = PermissionResult::ask_user();
    assert!(!result.is_allowed());
    assert!(!result.is_denied());
    assert!(matches!(result.action, PermissionAction::AskUser));
}

/// 测试 PermissionResult 的 sandbox 创建
/// sandbox() 应创建一个沙盒执行的结果
#[test]
fn test_permission_result_sandbox() {
    let result = PermissionResult::sandbox();
    assert!(matches!(result.action, PermissionAction::Sandbox));
}

/// 测试 PermissionChecker 在 Default 模式下的行为
/// Default 模式应询问用户确认
#[test]
fn test_permission_checker_default_mode() {
    let checker = PermissionChecker::new(PermissionMode::Default);

    let result = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(matches!(result.action, PermissionAction::AskUser));
}

/// 测试 PermissionChecker 在 AcceptAll 模式下的行为
/// AcceptAll 模式应自动批准所有工具
#[test]
fn test_permission_checker_accept_all_mode() {
    let checker = PermissionChecker::new(PermissionMode::AcceptAll);

    // bash 命令应被批准
    let result_bash = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(result_bash.is_allowed());

    // 文件工具应被批准
    let result_file = checker.check("read", &serde_json::json!({"path": "/tmp/test"}));
    assert!(result_file.is_allowed());
}

/// 测试 PermissionChecker 在 PlanMode 模式下的行为
/// PlanMode 模式应拒绝所有工具
#[test]
fn test_permission_checker_plan_mode() {
    let checker = PermissionChecker::new(PermissionMode::PlanMode);

    // bash 命令应被拒绝
    let result_bash = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(result_bash.is_denied());

    // 文件工具应被拒绝
    let result_file = checker.check("read", &serde_json::json!({"path": "/tmp/test"}));
    assert!(result_file.is_denied());
}

/// 测试 PermissionChecker 在 AcceptEdits 模式下的行为
/// AcceptEdits 模式应批准文件工具，询问 shell 工具
#[test]
fn test_permission_checker_accept_edits_mode() {
    let checker = PermissionChecker::new(PermissionMode::AcceptEdits);

    // 文件工具应被批准
    let result_file = checker.check("read", &serde_json::json!({"path": "/tmp/test"}));
    assert!(result_file.is_allowed());

    // shell 工具应询问用户
    let result_bash = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(matches!(result_bash.action, PermissionAction::AskUser));
}

/// 测试 PermissionChecker 在 Sandbox 模式下的行为
/// Sandbox 模式对文件工具应返回沙盒执行，对 bash 应询问用户
#[test]
fn test_permission_checker_sandbox_mode() {
    let checker = PermissionChecker::new(PermissionMode::Sandbox);

    // 文件工具应返回沙盒执行
    let result_file = checker.check("read", &serde_json::json!({"path": "/tmp/test"}));
    assert!(matches!(result_file.action, PermissionAction::Sandbox));

    // bash 工具在 Sandbox 模式下会询问用户（check_bash 有特殊逻辑）
    let result_bash = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(matches!(result_bash.action, PermissionAction::AskUser));
}

/// 测试 PermissionChecker 检测危险命令
/// 危险命令应被拒绝，无论何种模式
#[test]
fn test_permission_checker_dangerous_command() {
    let checker = PermissionChecker::new(PermissionMode::Default);

    // rm -rf 是危险命令
    let result = checker.check("bash", &serde_json::json!({"command": "rm -rf /tmp/test"}));
    assert!(result.is_denied());
    assert!(result.reason.unwrap().contains("Dangerous command"));
}

/// 测试 PermissionChecker 的危险命令列表
/// 包含 sudo、chmod 777、mkfs 等
#[test]
fn test_permission_checker_multiple_dangerous_commands() {
    let checker = PermissionChecker::new(PermissionMode::AcceptAll);

    // sudo 是危险命令
    let sudo_result = checker.check("bash", &serde_json::json!({"command": "sudo apt install"}));
    assert!(sudo_result.is_denied());

    // chmod 777 是危险命令
    let chmod_result = checker.check("bash", &serde_json::json!({"command": "chmod 777 /tmp"}));
    assert!(chmod_result.is_denied());
}

/// 测试 PermissionChecker 的 allowed_paths 设置
/// allowed_paths 应限制可访问的路径
#[test]
fn test_permission_checker_allowed_paths() {
    let checker = PermissionChecker::new(PermissionMode::Default)
        .with_allowed_paths(vec!["/safe/path".to_string()]);

    // 允许路径内的文件
    let result_allowed = checker.check("read", &serde_json::json!({"path": "/safe/path/test.txt"}));
    // Default 模式下仍需询问，但不会被路径拒绝

    // 允许路径外的文件应被拒绝
    let result_denied = checker.check(
        "read",
        &serde_json::json!({"path": "/dangerous/path/test.txt"}),
    );
    assert!(result_denied.is_denied());
    assert!(result_denied.reason.unwrap().contains("Path not allowed"));
}

/// 测试 PermissionChecker 的 set_mode 方法
/// set_mode 应能动态切换权限模式
#[test]
fn test_permission_checker_set_mode() {
    let mut checker = PermissionChecker::new(PermissionMode::Default);

    // 初始为 Default 模式
    assert_eq!(checker.mode(), PermissionMode::Default);

    // 切换到 AcceptAll
    checker.set_mode(PermissionMode::AcceptAll);
    assert_eq!(checker.mode(), PermissionMode::AcceptAll);

    // 现在应批准命令
    let result = checker.check("bash", &serde_json::json!({"command": "ls"}));
    assert!(result.is_allowed());
}

/// 测试 PermissionChecker 的 mode 方法
/// mode() 应返回当前权限模式
#[test]
fn test_permission_checker_mode() {
    let checker = PermissionChecker::new(PermissionMode::AcceptEdits);
    assert_eq!(checker.mode(), PermissionMode::AcceptEdits);
}
