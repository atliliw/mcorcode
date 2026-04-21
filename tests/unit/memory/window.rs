//! ConversationBufferWindowMemory 单元测试

use mcorcode::memory::BaseMemory;
use mcorcode::ConversationBufferWindowMemory;

/// 测试带自定义窗口大小的内存创建
/// 窗口大小应正确设置，消息列表应为空
#[test]
fn test_window_memory_new_with_size() {
    let memory = ConversationBufferWindowMemory::new(5);
    assert_eq!(memory.window_size(), 5);
    assert!(memory.get_messages().is_empty());
}

/// 测试 window 内存的 default 实现
/// 默认窗口大小应为 10
#[test]
fn test_window_memory_default() {
    let memory = ConversationBufferWindowMemory::default();
    assert_eq!(memory.window_size(), 10);
    assert!(memory.get_messages().is_empty());
}

/// 测试 with_default_window() 构造函数
/// 应创建窗口大小为 10 的内存
#[test]
fn test_window_memory_with_default_window() {
    let memory = ConversationBufferWindowMemory::with_default_window();
    assert_eq!(memory.window_size(), 10);
}

/// 测试超出窗口大小时的自动裁剪
/// 超出限制时最旧的消息应被移除
#[test]
fn test_window_memory_trims_excess_messages() {
    let mut memory = ConversationBufferWindowMemory::new(3);

    memory.add_user_message("msg1");
    memory.add_ai_message("msg2");
    memory.add_user_message("msg3");
    assert_eq!(memory.get_messages().len(), 3);

    memory.add_ai_message("msg4");
    memory.add_user_message("msg5");

    assert_eq!(memory.get_messages().len(), 3);

    assert_eq!(memory.get_messages()[0].content, "msg3");
    assert_eq!(memory.get_messages()[1].content, "msg4");
    assert_eq!(memory.get_messages()[2].content, "msg5");
}

/// 测试在窗口限制内时不裁剪
/// 消息数量 <= 窗口大小时全部保留
#[test]
fn test_window_memory_no_trimming_within_limit() {
    let mut memory = ConversationBufferWindowMemory::new(10);

    memory.add_user_message("msg1");
    memory.add_ai_message("msg2");
    memory.add_user_message("msg3");

    assert_eq!(memory.get_messages().len(), 3);
}

/// 测试动态缩小窗口大小
/// 已有消息应裁剪到新的较小大小
#[test]
fn test_window_memory_set_window_size() {
    let mut memory = ConversationBufferWindowMemory::new(10);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");
    memory.add_ai_message("4");

    memory.set_window_size(2);
    assert_eq!(memory.get_messages().len(), 2);
    assert_eq!(memory.window_size(), 2);
}

/// 测试在已有消息后扩大窗口
/// 更大的窗口不应裁剪已有消息
#[test]
fn test_window_memory_expand_window_size() {
    let mut memory = ConversationBufferWindowMemory::new(2);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");

    assert_eq!(memory.get_messages().len(), 2);

    memory.set_window_size(10);
    assert_eq!(memory.window_size(), 10);
    assert_eq!(memory.get_messages().len(), 2);
}

/// 测试 window 内存的清空功能
/// 无论窗口大小如何，所有消息都应被移除
#[test]
fn test_window_memory_clear() {
    let mut memory = ConversationBufferWindowMemory::new(5);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");

    memory.clear();
    assert!(memory.get_messages().is_empty());
}

/// 测试恰好达到窗口大小的边界情况
/// 消息数量等于窗口大小时不应触发裁剪
#[test]
fn test_window_memory_boundary_exact_size() {
    let mut memory = ConversationBufferWindowMemory::new(3);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");

    assert_eq!(memory.get_messages().len(), 3);
    assert_eq!(memory.get_messages()[0].content, "1");
}
