//! Hook Actions and Results

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum HookAction {
    AutoApprove,
    AutoDeny { reason: String },
    AskUser,
    ValidateInput { schema: Value },
    RunCommand { cmd: String },
    LogToFile { path: String },
}

#[derive(Debug, Clone)]
pub enum HookResult {
    Continue,
    Approved,
    Denied(String),
    AskUser,
}

impl HookResult {
    pub fn is_denied(&self) -> bool {
        matches!(self, HookResult::Denied(_))
    }

    pub fn is_approved(&self) -> bool {
        matches!(self, HookResult::Approved)
    }

    pub fn should_ask_user(&self) -> bool {
        matches!(self, HookResult::AskUser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_hook_result_should_ask_user() {
        assert!(HookResult::AskUser.should_ask_user());
        assert!(!HookResult::Approved.should_ask_user());
        assert!(!HookResult::Continue.should_ask_user());
    }

    #[test]
    fn test_hook_action_auto_approve() {
        let action = HookAction::AutoApprove;
        // Just test that we can create it
        assert!(matches!(action, HookAction::AutoApprove));
    }

    #[test]
    fn test_hook_action_auto_deny() {
        let action = HookAction::AutoDeny {
            reason: "test".to_string(),
        };
        if let HookAction::AutoDeny { reason } = action {
            assert_eq!(reason, "test");
        }
    }
}
