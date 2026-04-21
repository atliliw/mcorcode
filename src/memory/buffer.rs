//! Conversation buffer memory - stores all messages
//!
//! Reference: langchainrust/langchainrust/src/memory/buffer.rs

use super::base::{BaseMemory, MemoryError};
use crate::schema::Message;

pub struct ConversationBufferMemory {
    messages: Vec<Message>,
}

impl ConversationBufferMemory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Default for ConversationBufferMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseMemory for ConversationBufferMemory {
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn add_user_message(&mut self, content: &str) {
        self.add_message(Message::human(content));
    }

    fn add_ai_message(&mut self, content: &str) {
        self.add_message(Message::ai(content));
    }

    fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    fn clear(&mut self) {
        self.messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_memory_new() {
        let memory = ConversationBufferMemory::new();
        assert!(memory.get_messages().is_empty());
    }

    #[test]
    fn test_buffer_memory_add_user() {
        let mut memory = ConversationBufferMemory::new();
        memory.add_user_message("Hello");
        assert_eq!(memory.get_messages().len(), 1);
        assert_eq!(memory.get_messages()[0].role(), "user");
    }

    #[test]
    fn test_buffer_memory_add_ai() {
        let mut memory = ConversationBufferMemory::new();
        memory.add_ai_message("Hi there!");
        assert_eq!(memory.get_messages().len(), 1);
        assert_eq!(memory.get_messages()[0].role(), "assistant");
    }

    #[test]
    fn test_buffer_memory_add_multiple() {
        let mut memory = ConversationBufferMemory::new();
        memory.add_user_message("Hello");
        memory.add_ai_message("Hi");
        memory.add_user_message("How are you?");
        assert_eq!(memory.get_messages().len(), 3);
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
    fn test_buffer_memory_default() {
        let memory = ConversationBufferMemory::default();
        assert!(memory.get_messages().is_empty());
    }
}
