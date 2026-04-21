//! SessionState 会话状态单元测试

use mcorcode::session::SessionState;

/// 测试 SessionState 的创建
/// 新建的会话应有唯一 ID、创建时间与更新时间相同、消息计数为 0、状态为活跃
#[test]
fn test_session_state_new() {
    let state = SessionState::new();
    assert!(!state.id.is_empty());
    assert_eq!(state.created_at, state.updated_at);
    assert_eq!(state.message_count, 0);
    assert!(state.active);
}

/// 测试 SessionState 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_session_state_default() {
    let state = SessionState::default();
    assert!(!state.id.is_empty());
    assert_eq!(state.message_count, 0);
    assert!(state.active);
}

/// 测试带指定 ID 的 SessionState 创建
/// with_id 应创建具有指定 ID 的会话状态
#[test]
fn test_session_state_with_id() {
    let custom_id = "my-custom-session-id";
    let state = SessionState::with_id(custom_id);
    assert_eq!(state.id, custom_id);
}

/// 测试 touch 方法更新时间
/// touch() 后 updated_at 应更新为当前时间
#[test]
fn test_session_state_touch() {
    let mut state = SessionState::new();
    let original_updated = state.updated_at;

    // 等待一小段时间
    std::thread::sleep(std::time::Duration::from_millis(10));

    state.touch();

    // updated_at 应比原来晚
    assert!(state.updated_at > original_updated);
}

/// 测试 increment_messages 方法
/// increment_messages() 应增加消息计数并更新时间
#[test]
fn test_session_state_increment_messages() {
    let mut state = SessionState::new();
    let original_updated = state.updated_at;

    std::thread::sleep(std::time::Duration::from_millis(10));

    state.increment_messages();

    assert_eq!(state.message_count, 1);
    assert!(state.updated_at > original_updated);

    // 再次增加
    state.increment_messages();
    assert_eq!(state.message_count, 2);
}

/// 测试 deactivate 方法
/// deactivate() 后 active 应变为 false，updated_at 应更新
#[test]
fn test_session_state_deactivate() {
    let mut state = SessionState::new();
    assert!(state.active);

    state.deactivate();

    assert!(!state.active);
}

/// 测试 UUID 格式的 ID
/// SessionState ID 应是有效的 UUID v4 格式
#[test]
fn test_session_state_uuid_format() {
    let state = SessionState::new();

    // UUID v4 格式验证
    let parts: Vec<&str> = state.id.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
}
