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
