//! Unit tests for ConversationBufferMemory

use mcorcode::memory::BaseMemory;
use mcorcode::ConversationBufferMemory;

#[test]
fn test_buffer_memory_new() {
    let memory = ConversationBufferMemory::new();
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_buffer_memory_default() {
    let memory = ConversationBufferMemory::default();
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_buffer_memory_add_user_message() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    assert_eq!(memory.get_messages().len(), 1);
    assert_eq!(memory.get_messages()[0].content, "Hello");
    assert_eq!(memory.get_messages()[0].role(), "user");
}

#[test]
fn test_buffer_memory_add_ai_message() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_ai_message("Hi there!");
    assert_eq!(memory.get_messages().len(), 1);
    assert_eq!(memory.get_messages()[0].content, "Hi there!");
    assert_eq!(memory.get_messages()[0].role(), "assistant");
}

#[test]
fn test_buffer_memory_add_multiple_messages() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    memory.add_ai_message("Hi");
    memory.add_user_message("How are you?");
    assert_eq!(memory.get_messages().len(), 3);
}

#[test]
fn test_buffer_memory_conversation_flow() {
    let mut memory = ConversationBufferMemory::new();

    // Simulate a conversation
    memory.add_user_message("What is Rust?");
    memory.add_ai_message("Rust is a systems programming language.");
    memory.add_user_message("Tell me more.");
    memory.add_ai_message("It focuses on safety and performance.");

    assert_eq!(memory.get_messages().len(), 4);

    // Check order
    assert_eq!(memory.get_messages()[0].role(), "user");
    assert_eq!(memory.get_messages()[1].role(), "assistant");
    assert_eq!(memory.get_messages()[2].role(), "user");
    assert_eq!(memory.get_messages()[3].role(), "assistant");
}

#[test]
fn test_buffer_memory_clear() {
    let mut memory = ConversationBufferMemory::new();
    memory.add_user_message("Hello");
    memory.add_ai_message("Hi");
    assert_eq!(memory.get_messages().len(), 2);

    memory.clear();
    assert!(memory.get_messages().is_empty());
}

#[test]
fn test_buffer_memory_preserves_all_messages() {
    let mut memory = ConversationBufferMemory::new();

    // Add many messages - buffer should keep all
    for i in 0..100 {
        memory.add_user_message(&format!("Message {}", i));
    }

    assert_eq!(memory.get_messages().len(), 100);
}
