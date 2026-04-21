//! HookTrigger 枚举单元测试

use mcorcode::HookTrigger;

/// 测试所有触发器类型的 as_str 方法
/// 每种触发器应返回正确的字符串标识
#[test]
fn test_hook_trigger_as_str() {
    assert_eq!(HookTrigger::PreToolCall.as_str(), "pre_tool_call");
    assert_eq!(HookTrigger::PostToolCall.as_str(), "post_tool_call");
    assert_eq!(HookTrigger::PreSession.as_str(), "pre_session");
    assert_eq!(HookTrigger::PostSession.as_str(), "post_session");
    assert_eq!(HookTrigger::OnCompact.as_str(), "on_compact");
    assert_eq!(HookTrigger::OnError.as_str(), "on_error");
}

/// 测试触发器的 PartialEq 实现
/// 相同触发器相等，不同触发器不相等
#[test]
fn test_hook_trigger_equality() {
    assert_eq!(HookTrigger::PreToolCall, HookTrigger::PreToolCall);
    assert_eq!(HookTrigger::PostSession, HookTrigger::PostSession);
    assert_ne!(HookTrigger::PreToolCall, HookTrigger::PostToolCall);
    assert_ne!(HookTrigger::PreSession, HookTrigger::PostSession);
}

/// 测试 Clone trait 实现
/// 克隆的触发器应与原触发器完全相同
#[test]
fn test_hook_trigger_clone() {
    let trigger = HookTrigger::OnCompact;
    let cloned = trigger.clone();
    assert_eq!(trigger, cloned);
}

/// 测试 Copy trait 实现
/// 复制的触发器应与原触发器相等
#[test]
fn test_hook_trigger_copy() {
    let trigger = HookTrigger::OnError;
    let copied = trigger;
    assert_eq!(trigger, copied);
}

/// 测试所有触发器类型的字符串表示唯一
/// 每种触发器应产生不同的 as_str 输出
#[test]
fn test_all_trigger_types() {
    let triggers = [
        HookTrigger::PreToolCall,
        HookTrigger::PostToolCall,
        HookTrigger::PreSession,
        HookTrigger::PostSession,
        HookTrigger::OnCompact,
        HookTrigger::OnError,
    ];

    let strings: Vec<&str> = triggers.iter().map(|t| t.as_str()).collect();
    for i in 0..strings.len() {
        for j in (i + 1)..strings.len() {
            assert_ne!(strings[i], strings[j], "触发器字符串重复");
        }
    }
}
