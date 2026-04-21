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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_mode_as_str() {
        assert_eq!(PermissionMode::Default.as_str(), "default");
        assert_eq!(PermissionMode::AcceptEdits.as_str(), "accept-edits");
        assert_eq!(PermissionMode::AcceptAll.as_str(), "accept-all");
        assert_eq!(PermissionMode::PlanMode.as_str(), "plan");
        assert_eq!(PermissionMode::Sandbox.as_str(), "sandbox");
    }

    #[test]
    fn test_permission_mode_from_str() {
        assert_eq!(
            PermissionMode::from_str("default"),
            Some(PermissionMode::Default)
        );
        assert_eq!(
            PermissionMode::from_str("accept-edits"),
            Some(PermissionMode::AcceptEdits)
        );
        assert_eq!(
            PermissionMode::from_str("accept-all"),
            Some(PermissionMode::AcceptAll)
        );
        assert_eq!(
            PermissionMode::from_str("plan"),
            Some(PermissionMode::PlanMode)
        );
        assert_eq!(
            PermissionMode::from_str("sandbox"),
            Some(PermissionMode::Sandbox)
        );
        assert_eq!(PermissionMode::from_str("invalid"), None);
    }

    #[test]
    fn test_permission_mode_display() {
        assert_eq!(PermissionMode::Default.to_string(), "default");
        assert_eq!(PermissionMode::AcceptAll.to_string(), "accept-all");
    }

    #[test]
    fn test_permission_mode_default() {
        let mode = PermissionMode::default();
        assert_eq!(mode, PermissionMode::Default);
    }

    #[test]
    fn test_permission_mode_equality() {
        assert_eq!(PermissionMode::Default, PermissionMode::Default);
        assert_ne!(PermissionMode::Default, PermissionMode::AcceptAll);
    }
}
