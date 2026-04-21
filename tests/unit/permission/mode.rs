//! Unit tests for PermissionMode

use mcorcode::PermissionMode;

#[test]
fn test_permission_mode_as_str() {
    assert_eq!(PermissionMode::Default.as_str(), "default");
    assert_eq!(PermissionMode::AcceptEdits.as_str(), "accept-edits");
    assert_eq!(PermissionMode::AcceptAll.as_str(), "accept-all");
    assert_eq!(PermissionMode::PlanMode.as_str(), "plan");
    assert_eq!(PermissionMode::Sandbox.as_str(), "sandbox");
}

#[test]
fn test_permission_mode_from_str_valid() {
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
}

#[test]
fn test_permission_mode_from_str_invalid() {
    assert_eq!(PermissionMode::from_str("invalid"), None);
    assert_eq!(PermissionMode::from_str(""), None);
    assert_eq!(PermissionMode::from_str("ACCEPT-ALL"), None); // case sensitive
}

#[test]
fn test_permission_mode_display() {
    assert_eq!(PermissionMode::Default.to_string(), "default");
    assert_eq!(PermissionMode::AcceptEdits.to_string(), "accept-edits");
    assert_eq!(PermissionMode::AcceptAll.to_string(), "accept-all");
    assert_eq!(PermissionMode::PlanMode.to_string(), "plan");
    assert_eq!(PermissionMode::Sandbox.to_string(), "sandbox");
}

#[test]
fn test_permission_mode_default() {
    let mode = PermissionMode::default();
    assert_eq!(mode, PermissionMode::Default);
}

#[test]
fn test_permission_mode_equality() {
    assert_eq!(PermissionMode::Default, PermissionMode::Default);
    assert_eq!(PermissionMode::AcceptAll, PermissionMode::AcceptAll);
    assert_ne!(PermissionMode::Default, PermissionMode::AcceptAll);
    assert_ne!(PermissionMode::PlanMode, PermissionMode::Sandbox);
}

#[test]
fn test_permission_mode_clone() {
    let mode = PermissionMode::AcceptEdits;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[test]
fn test_permission_mode_copy() {
    let mode = PermissionMode::PlanMode;
    let copied = mode;
    assert_eq!(mode, copied);
}
