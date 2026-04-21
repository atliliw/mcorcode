//! SessionManager 持久化功能单元测试
//! 测试会话保存、加载和管理

use mcorcode::schema::Message;
use mcorcode::session::SessionManager;
use tempfile::tempdir;

/// 测试 SessionManager 的创建
/// 新建管理器应有默认存储路径
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

/// 测试 SessionManager 的自定义存储路径
/// with_storage_path 应正确设置路径
#[test]
fn test_session_manager_with_storage_path() {
    let temp = tempdir().unwrap();
    let manager = SessionManager::with_storage_path(temp.path());
    assert!(manager.list_sessions().is_empty());
}

/// 测试创建会话
/// create_session 应返回唯一 ID
#[test]
fn test_session_manager_create_session() {
    let mut manager = SessionManager::new();
    let id = manager.create_session();
    assert!(!id.is_empty());
    assert!(manager.session_exists(&id));
}

/// 测试会话存在检查
/// session_exists 应正确判断会话是否存在
#[test]
fn test_session_manager_session_exists() {
    let mut manager = SessionManager::new();
    let id = manager.create_session();
    assert!(manager.session_exists(&id));
    assert!(!manager.session_exists("nonexistent"));
}

/// 测试添加消息
/// add_message 应增加消息计数
#[test]
fn test_session_manager_add_message() {
    let mut manager = SessionManager::new();
    let id = manager.create_session();

    manager.add_message(&id, Message::human("Hello")).unwrap();

    let messages = manager.get_messages(&id).unwrap();
    assert_eq!(messages.len(), 1);
}

/// 测试保存会话到文件
/// save_session 应创建 JSON 文件
#[test]
fn test_session_manager_save_session() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path());
    let id = manager.create_session();

    manager.add_message(&id, Message::human("Test")).unwrap();
    manager.save_session(&id).unwrap();

    assert!(manager.session_file_exists(&id));
}

/// 测试加载会话
/// load_session 应恢复会话状态和历史
#[test]
fn test_session_manager_load_session() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path());
    let id = manager.create_session();

    manager
        .add_message(&id, Message::human("Message 1"))
        .unwrap();
    manager.add_message(&id, Message::ai("Response 1")).unwrap();
    manager.save_session(&id).unwrap();

    manager.delete_session(&id);
    assert!(!manager.session_exists(&id));

    manager.load_session(&id).unwrap();
    assert!(manager.session_exists(&id));

    let messages = manager.get_messages(&id).unwrap();
    assert_eq!(messages.len(), 2);
}

/// 测试列出已保存会话
/// list_saved_sessions 应返回所有已保存会话 ID
#[test]
fn test_session_manager_list_saved_sessions() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path());

    let id1 = manager.create_session();
    let id2 = manager.create_session();

    manager.save_session(&id1).unwrap();
    manager.save_session(&id2).unwrap();

    let saved = manager.list_saved_sessions().unwrap();
    assert_eq!(saved.len(), 2);
    assert!(saved.contains(&id1));
    assert!(saved.contains(&id2));
}

/// 测试删除会话文件
/// delete_session_file 应删除保存的文件
#[test]
fn test_session_manager_delete_session_file() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path());
    let id = manager.create_session();

    manager.save_session(&id).unwrap();
    assert!(manager.session_file_exists(&id));

    manager.delete_session_file(&id).unwrap();
    assert!(!manager.session_file_exists(&id));
}

/// 测试自动保存功能
/// with_auto_save 启用后添加消息应自动保存
#[test]
fn test_session_manager_auto_save() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path()).with_auto_save(true);
    let id = manager.create_session();

    manager
        .add_message(&id, Message::human("Auto save test"))
        .unwrap();

    assert!(manager.session_file_exists(&id));
}

/// 测试保存所有会话
/// save_all 应保存所有活跃会话
#[test]
fn test_session_manager_save_all() {
    let temp = tempdir().unwrap();
    let mut manager = SessionManager::with_storage_path(temp.path());

    let id1 = manager.create_session();
    let id2 = manager.create_session();

    manager.save_all().unwrap();

    assert!(manager.session_file_exists(&id1));
    assert!(manager.session_file_exists(&id2));
}
