//! SessionManager 会话管理器单元测试

use mcorcode::session::SessionManager;
use mcorcode::Message;

/// 测试 SessionManager 的创建
/// 新建的管理器应无任何会话
#[test]
fn test_session_manager_new() {
    let manager = SessionManager::new();
    assert!(manager.list_sessions().is_empty());
}

/// 测试 SessionManager 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_session_manager_default() {
    let manager = SessionManager::default();
    assert!(manager.list_sessions().is_empty());
}

/// 测试创建新会话
/// create_session 应返回会话 ID，会话列表应包含该会话
#[test]
fn test_session_manager_create_session() {
    let mut manager = SessionManager::new();
    let session_id = manager.create_session();

    assert!(!session_id.is_empty());
    assert_eq!(manager.list_sessions().len(), 1);

    // 应能获取创建的会话
    let session = manager.get_session(&session_id);
    assert!(session.is_some());
}

/// 测试获取会话
/// get_session 对存在的会话返回 Some，对不存在返回 None
#[test]
fn test_session_manager_get_session() {
    let mut manager = SessionManager::new();
    let session_id = manager.create_session();

    // 存在的会话
    let session = manager.get_session(&session_id);
    assert!(session.is_some());
    assert_eq!(session.unwrap().id, session_id);

    // 不存在的会话
    let nonexistent = manager.get_session("nonexistent-id");
    assert!(nonexistent.is_none());
}

/// 测试添加消息到会话
/// add_message 应向指定会话添加消息，增加消息计数
#[test]
fn test_session_manager_add_message() {
    let mut manager = SessionManager::new();
    let session_id = manager.create_session();

    let message = Message::human("Hello, world!");
    let result = manager.add_message(&session_id, message);

    assert!(result.is_ok());

    // 消息计数应增加
    let session = manager.get_session(&session_id).unwrap();
    assert_eq!(session.message_count, 1);

    // 应能获取消息列表
    let messages = manager.get_messages(&session_id);
    assert!(messages.is_some());
    assert_eq!(messages.unwrap().len(), 1);
}

/// 测试向不存在会话添加消息
/// add_message 向不存在的会话添加消息应不产生错误（但不增加任何计数）
#[test]
fn test_session_manager_add_message_to_nonexistent() {
    let mut manager = SessionManager::new();

    let message = Message::human("Test");
    let result = manager.add_message("nonexistent", message);

    // 应成功（不会向不存在的会话添加）
    assert!(result.is_ok());

    // 仍无会话
    assert!(manager.list_sessions().is_empty());
}

/// 测试删除会话
/// delete_session 后会话应被移除
#[test]
fn test_session_manager_delete_session() {
    let mut manager = SessionManager::new();
    let session_id = manager.create_session();

    assert_eq!(manager.list_sessions().len(), 1);

    manager.delete_session(&session_id);

    assert!(manager.list_sessions().is_empty());
    assert!(manager.get_session(&session_id).is_none());
}

/// 测试清空所有会话
/// clear_all 后所有会话和历史应被移除
#[test]
fn test_session_manager_clear_all() {
    let mut manager = SessionManager::new();

    // 创建多个会话
    let id1 = manager.create_session();
    let id2 = manager.create_session();
    let id3 = manager.create_session();

    assert_eq!(manager.list_sessions().len(), 3);

    manager.clear_all();

    assert!(manager.list_sessions().is_empty());
}

/// 测试多个会话的消息隔离
/// 不同会话的消息应独立存储
#[test]
fn test_session_manager_message_isolation() {
    let mut manager = SessionManager::new();

    let id1 = manager.create_session();
    let id2 = manager.create_session();

    manager
        .add_message(&id1, Message::human("Session 1 message"))
        .unwrap();
    manager
        .add_message(&id2, Message::human("Session 2 message A"))
        .unwrap();
    manager
        .add_message(&id2, Message::human("Session 2 message B"))
        .unwrap();

    // 会话 1 应有 1 条消息
    let messages1 = manager.get_messages(&id1).unwrap();
    assert_eq!(messages1.len(), 1);

    // 会话 2 应有 2 条消息
    let messages2 = manager.get_messages(&id2).unwrap();
    assert_eq!(messages2.len(), 2);

    // 会话状态消息计数应正确
    assert_eq!(manager.get_session(&id1).unwrap().message_count, 1);
    assert_eq!(manager.get_session(&id2).unwrap().message_count, 2);
}
