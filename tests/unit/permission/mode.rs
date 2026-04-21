//! PermissionMode 枚举单元测试

use mcorcode::PermissionMode;

/// 测试所有权限模式的 as_str 方法
/// 每种模式应返回正确的字符串表示
#[test]
fn test_permission_mode_as_str() {
    assert_eq!(PermissionMode::Default.as_str(), "default");
    assert_eq!(PermissionMode::AcceptEdits.as_str(), "accept-edits");
    assert_eq!(PermissionMode::AcceptAll.as_str(), "accept-all");
    assert_eq!(PermissionMode::PlanMode.as_str(), "plan");
    assert_eq!(PermissionMode::Sandbox.as_str(), "sandbox");
}

/// 测试 from_str 解析有效模式字符串
/// 所有有效模式字符串应解析为正确的 PermissionMode
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

/// 测试 from_str 解析无效字符串
/// 无效字符串应返回 None
#[test]
fn test_permission_mode_from_str_invalid() {
    assert_eq!(PermissionMode::from_str("invalid"), None);
    assert_eq!(PermissionMode::from_str(""), None);
    assert_eq!(PermissionMode::from_str("ACCEPT-ALL"), None);
}

/// 测试 Display trait 实现
/// to_string 应与 as_str 输出一致
#[test]
fn test_permission_mode_display() {
    assert_eq!(PermissionMode::Default.to_string(), "default");
    assert_eq!(PermissionMode::AcceptEdits.to_string(), "accept-edits");
    assert_eq!(PermissionMode::AcceptAll.to_string(), "accept-all");
    assert_eq!(PermissionMode::PlanMode.to_string(), "plan");
    assert_eq!(PermissionMode::Sandbox.to_string(), "sandbox");
}

/// 测试 Default trait 实现
/// 默认模式应为 PermissionMode::Default
#[test]
fn test_permission_mode_default() {
    let mode = PermissionMode::default();
    assert_eq!(mode, PermissionMode::Default);
}

/// 测试 PartialEq 实现
/// 相同模式应相等，不同模式不应相等
#[test]
fn test_permission_mode_equality() {
    assert_eq!(PermissionMode::Default, PermissionMode::Default);
    assert_eq!(PermissionMode::AcceptAll, PermissionMode::AcceptAll);
    assert_ne!(PermissionMode::Default, PermissionMode::AcceptAll);
    assert_ne!(PermissionMode::PlanMode, PermissionMode::Sandbox);
}

/// 测试 Clone trait 实现
/// 克隆的模式应与原模式相等
#[test]
fn test_permission_mode_clone() {
    let mode = PermissionMode::AcceptEdits;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

/// 测试 Copy trait 实现
/// 复制的模式应与原模式相等
#[test]
fn test_permission_mode_copy() {
    let mode = PermissionMode::PlanMode;
    let copied = mode;
    assert_eq!(mode, copied);
}
