//! Unit tests for HookTrigger

use mcorcode::HookTrigger;

#[test]
fn test_hook_trigger_as_str() {
    assert_eq!(HookTrigger::PreToolCall.as_str(), "pre_tool_call");
    assert_eq!(HookTrigger::PostToolCall.as_str(), "post_tool_call");
    assert_eq!(HookTrigger::PreSession.as_str(), "pre_session");
    assert_eq!(HookTrigger::PostSession.as_str(), "post_session");
    assert_eq!(HookTrigger::OnCompact.as_str(), "on_compact");
    assert_eq!(HookTrigger::OnError.as_str(), "on_error");
}

#[test]
fn test_hook_trigger_equality() {
    assert_eq!(HookTrigger::PreToolCall, HookTrigger::PreToolCall);
    assert_eq!(HookTrigger::PostSession, HookTrigger::PostSession);
    assert_ne!(HookTrigger::PreToolCall, HookTrigger::PostToolCall);
    assert_ne!(HookTrigger::PreSession, HookTrigger::PostSession);
}

#[test]
fn test_hook_trigger_clone() {
    let trigger = HookTrigger::OnCompact;
    let cloned = trigger.clone();
    assert_eq!(trigger, cloned);
}

#[test]
fn test_hook_trigger_copy() {
    let trigger = HookTrigger::OnError;
    let copied = trigger;
    assert_eq!(trigger, copied);
}

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

    // Verify all have unique string representations
    let strings: Vec<&str> = triggers.iter().map(|t| t.as_str()).collect();
    for i in 0..strings.len() {
        for j in (i + 1)..strings.len() {
            assert_ne!(strings[i], strings[j], "Duplicate trigger strings");
        }
    }
}
