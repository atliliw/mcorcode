//! Permission Modes
//!
//! 借鉴 Claude Code 的 5 种权限模式

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionMode {
    Default,
    AcceptEdits,
    AcceptAll,
    PlanMode,
    Sandbox,
}

impl PermissionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionMode::Default => "default",
            PermissionMode::AcceptEdits => "accept-edits",
            PermissionMode::AcceptAll => "accept-all",
            PermissionMode::PlanMode => "plan",
            PermissionMode::Sandbox => "sandbox",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(PermissionMode::Default),
            "accept-edits" => Some(PermissionMode::AcceptEdits),
            "accept-all" => Some(PermissionMode::AcceptAll),
            "plan" => Some(PermissionMode::PlanMode),
            "sandbox" => Some(PermissionMode::Sandbox),
            _ => None,
        }
    }
}

impl std::fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for PermissionMode {
    fn default() -> Self {
        PermissionMode::Default
    }
}
