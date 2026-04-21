//! Unit tests for ConversationBufferWindowMemory

use mcorcode::memory::BaseMemory;
use mcorcode::ConversationBufferWindowMemory;

#[test]
fn test_window_memory_new_with_size() {
    let memory = ConversationBufferWindowMemory::new(5);
    assert_eq!(memory.window_size(), 5);
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_window_memory_default() {
    let memory = ConversationBufferWindowMemory::default();
    assert_eq!(memory.window_size(), 10);
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_window_memory_with_default_window() {
    let memory = ConversationBufferWindowMemory::with_default_window();
    assert_eq!(memory.window_size(), 10);
}

#[test]
fn test_window_memory_trims_excess_messages() {
    let mut memory = ConversationBufferWindowMemory::new(3);

    memory.add_user_message("msg1");
    memory.add_ai_message("msg2");
    memory.add_user_message("msg3");
    assert_eq!(memory.get_messages().len(), 3);

    // Adding more should trim oldest
    memory.add_ai_message("msg4");
    memory.add_user_message("msg5");

    // Should keep only last 3
    assert_eq!(memory.get_messages().len(), 3);

    // Verify oldest messages were removed
    assert_eq!(memory.get_messages()[0].content, "msg3");
    assert_eq!(memory.get_messages()[1].content, "msg4");
    assert_eq!(memory.get_messages()[2].content, "msg5");
}

#[test]
fn test_window_memory_no_trimming_within_limit() {
    let mut memory = ConversationBufferWindowMemory::new(10);

    memory.add_user_message("msg1");
    memory.add_ai_message("msg2");
    memory.add_user_message("msg3");

    // All within limit - no trimming
    assert_eq!(memory.get_messages().len(), 3);
}

#[test]
fn test_window_memory_set_window_size() {
    let mut memory = ConversationBufferWindowMemory::new(10);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");
    memory.add_ai_message("4");

    // Reduce window size - should trim
    memory.set_window_size(2);
    assert_eq!(memory.get_messages().len(), 2);
    assert_eq!(memory.window_size(), 2);
}

#[test]
fn test_window_memory_expand_window_size() {
    let mut memory = ConversationBufferWindowMemory::new(2);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3"); // msg1 should be trimmed

    assert_eq!(memory.get_messages().len(), 2);

    // Expand window - existing messages stay
    memory.set_window_size(10);
    assert_eq!(memory.window_size(), 10);
    assert_eq!(memory.get_messages().len(), 2);
}

#[test]
fn test_window_memory_clear() {
    let mut memory = ConversationBufferWindowMemory::new(5);

    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");

    memory.clear();
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_window_memory_boundary_exact_size() {
    let mut memory = ConversationBufferWindowMemory::new(3);

    // Exactly at limit
    memory.add_user_message("1");
    memory.add_ai_message("2");
    memory.add_user_message("3");

    assert_eq!(memory.get_messages().len(), 3);
    assert_eq!(memory.get_messages()[0].content, "1");
}
