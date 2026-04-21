//! HookSystem 钩子系统单元测试

use mcorcode::hooks::{HookAction, HookResult, HookSystem, HookTrigger};

/// 测试 HookSystem 的创建
/// 新建的钩子系统应无任何钩子
#[test]
fn test_hook_system_new() {
    let system = HookSystem::new();

    // 无钩子时执行应返回 Continue
    let result = system.execute_sync(
        HookTrigger::PreToolCall,
        "bash",
        &serde_json::json!({"command": "ls"}),
    );
    assert!(matches!(result, HookResult::Continue));
}

/// 测试 HookSystem 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_hook_system_default() {
    let system = HookSystem::default();

    let result = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result, HookResult::Continue));
}

/// 测试添加 AutoApprove 钩子
/// AutoApprove 钩子匹配时应返回 Approved
#[test]
fn test_hook_system_auto_approve() {
    use mcorcode::hooks::system::Hook;

    let hook = Hook::new(HookTrigger::PreToolCall, HookAction::AutoApprove);

    let system = HookSystem::new().add(hook);

    let result = system.execute_sync(
        HookTrigger::PreToolCall,
        "bash",
        &serde_json::json!({"command": "ls"}),
    );
    assert!(matches!(result, HookResult::Approved));
}

/// 测试添加 AutoDeny 钩子
/// AutoDeny 钩子匹配时应返回 Denied 及原因
#[test]
fn test_hook_system_auto_deny() {
    use mcorcode::hooks::system::Hook;

    let hook = Hook::new(
        HookTrigger::PreToolCall,
        HookAction::AutoDeny {
            reason: "Dangerous command".to_string(),
        },
    );

    let system = HookSystem::new().add(hook);

    let result = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result, HookResult::Denied(_)));
    if let HookResult::Denied(reason) = result {
        assert_eq!(reason, "Dangerous command");
    }
}

/// 测试钩子的工具过滤器
/// for_tool 设置后钩子只匹配特定工具
#[test]
fn test_hook_for_tool_filter() {
    use mcorcode::hooks::system::Hook;

    // 只批准 bash 工具
    let hook = Hook::new(HookTrigger::PreToolCall, HookAction::AutoApprove).for_tool("bash");

    let system = HookSystem::new().add(hook);

    // bash 工具应被批准
    let result_bash = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result_bash, HookResult::Approved));

    // 其他工具应返回 Continue（无匹配钩子）
    let result_other = system.execute_sync(
        HookTrigger::PreToolCall,
        "read_file",
        &serde_json::json!({}),
    );
    assert!(matches!(result_other, HookResult::Continue));
}

/// 测试钩子的通配符匹配
/// "*" 应匹配所有工具
#[test]
fn test_hook_wildcard_match() {
    use mcorcode::hooks::system::Hook;

    // 使用通配符匹配所有工具
    let hook = Hook::new(HookTrigger::PreToolCall, HookAction::AutoApprove).for_tool("*");

    let system = HookSystem::new().add(hook);

    // 所有工具都应被批准
    let result1 = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result1, HookResult::Approved));

    let result2 = system.execute_sync(
        HookTrigger::PreToolCall,
        "read_file",
        &serde_json::json!({}),
    );
    assert!(matches!(result2, HookResult::Approved));
}

/// 测试钩子优先级排序
/// 优先级高的钩子应优先执行
#[test]
fn test_hook_priority() {
    use mcorcode::hooks::system::Hook;

    // 低优先级批准
    let hook_low = Hook::new(HookTrigger::PreToolCall, HookAction::AutoApprove).with_priority(10);

    // 高优先级拒绝
    let hook_high = Hook::new(
        HookTrigger::PreToolCall,
        HookAction::AutoDeny {
            reason: "Blocked by high priority".to_string(),
        },
    )
    .with_priority(1);

    // 添加顺序不重要，按优先级执行
    let system = HookSystem::new().add(hook_low).add(hook_high);

    // 高优先级（1）的拒绝钩子应先执行
    let result = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result, HookResult::Denied(_)));
}

/// 测试 AskUser 钩子动作
/// AskUser 应返回需要用户确认的结果
#[test]
fn test_hook_ask_user() {
    use mcorcode::hooks::system::Hook;

    let hook = Hook::new(HookTrigger::PreToolCall, HookAction::AskUser);

    let system = HookSystem::new().add(hook);

    let result = system.execute_sync(HookTrigger::PreToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result, HookResult::AskUser));
}

/// 测试不匹配的触发器
/// 不同触发器的钩子不应互相影响
#[test]
fn test_hook_wrong_trigger() {
    use mcorcode::hooks::system::Hook;

    // 只响应 PreToolCall
    let hook = Hook::new(HookTrigger::PreToolCall, HookAction::AutoApprove);

    let system = HookSystem::new().add(hook);

    // PostToolCall 应返回 Continue
    let result = system.execute_sync(HookTrigger::PostToolCall, "bash", &serde_json::json!({}));
    assert!(matches!(result, HookResult::Continue));
}
