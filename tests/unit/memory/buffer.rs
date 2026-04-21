//! ConversationBufferMemory 单元测试

use mcorcode::memory::BaseMemory;
use mcorcode::ConversationBufferMemory;

/// 测试 ConversationBufferMemory 的创建
/// 新建的内存应有空的消息列表
#[test]
fn test_buffer_memory_new() {
    let memory = ConversationBufferMemory::new();
    assert!(memory.get_messages().is_empty());
}

/// 测试 ConversationBufferMemory 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_buffer_memory_default() {
    let memory = ConversationBufferMemory::default();
    assert!(memory.get_messages().is_empty());
}

/// 测试向内存添加用户消息
/// 消息计数应增加，角色应为 "user"
#[test]
fn test_buffer_memory_add_user_message() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    assert_eq!(memory.get_messages().len(), 1);
    assert_eq!(memory.get_messages()[0].content, "Hello");
    assert_eq!(memory.get_messages()[0].role(), "user");
}

/// 测试向内存添加 AI 消息
/// 消息计数应增加，角色应为 "assistant"
#[test]
fn test_buffer_memory_add_ai_message() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_ai_message("Hi there!");
    assert_eq!(memory.get_messages().len(), 1);
    assert_eq!(memory.get_messages()[0].content, "Hi there!");
    assert_eq!(memory.get_messages()[0].role(), "assistant");
}

/// 测试向内存添加多条消息
/// 所有消息应按顺序保留
#[test]
fn test_buffer_memory_add_multiple_messages() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    memory.add_ai_message("Hi");
    memory.add_user_message("How are you?");
    assert_eq!(memory.get_messages().len(), 3);
}

/// 测试对话流程模拟
/// 验证消息以正确顺序存储（用户、助手交替）
#[test]
fn test_buffer_memory_conversation_flow() {
    let mut memory = ConversationBufferMemory::new();

    memory.add_user_message("What is Rust?");
    memory.add_ai_message("Rust is a systems programming language.");
    memory.add_user_message("Tell me more.");
    memory.add_ai_message("It focuses on safety and performance.");

    assert_eq!(memory.get_messages().len(), 4);

    assert_eq!(memory.get_messages()[0].role(), "user");
    assert_eq!(memory.get_messages()[1].role(), "assistant");
    assert_eq!(memory.get_messages()[2].role(), "user");
    assert_eq!(memory.get_messages()[3].role(), "assistant");
}

/// 测试内存清空功能
/// clear() 后所有消息应被移除
#[test]
fn test_buffer_memory_clear() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    memory.add_ai_message("Hi");
    assert_eq!(memory.get_messages().len(), 2);

    memory.clear();
    assert!(memory.get_messages().is_empty());
}

/// 测试 buffer 内存保留所有消息
/// 与 window 内存不同，buffer 不限制消息数量
#[test]
fn test_buffer_memory_preserves_all_messages() {
    let mut memory = ConversationBufferMemory::new();

    for i in 0..100 {
        memory.add_user_message(&format!("Message {}", i));
    }

    assert_eq!(memory.get_messages().len(), 100);
}
