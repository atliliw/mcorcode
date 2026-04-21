//! Hook Triggers

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookTrigger {
    PreToolCall,
    PostToolCall,
    PreSession,
    PostSession,
    OnCompact,
    OnError,
}

impl HookTrigger {
    pub fn as_str(&self) -> &'static str {
        match self {
            HookTrigger::PreToolCall => "pre_tool_call",
            HookTrigger::PostToolCall => "post_tool_call",
            HookTrigger::PreSession => "pre_session",
            HookTrigger::PostSession => "post_session",
            HookTrigger::OnCompact => "on_compact",
            HookTrigger::OnError => "on_error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_ne!(HookTrigger::PreToolCall, HookTrigger::PostToolCall);
    }
}
