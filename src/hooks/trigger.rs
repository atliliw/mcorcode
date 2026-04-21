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
