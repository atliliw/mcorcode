//! Conversation buffer window memory - stores last N messages
//!
//! Reference: langchainrust/langchainrust/src/memory/window.rs

use super::base::{BaseMemory, MemoryError};
use crate::schema::Message;

pub struct ConversationBufferWindowMemory {
    messages: Vec<Message>,
    window_size: usize,
}

impl ConversationBufferWindowMemory {
    pub fn new(window_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            window_size,
        }
    }

    pub fn with_default_window() -> Self {
        Self::new(10)
    }

    pub fn window_size(&self) -> usize {
        self.window_size
    }

    pub fn set_window_size(&mut self, size: usize) {
        self.window_size = size;
        self.trim_messages();
    }

    fn trim_messages(&mut self) {
        if self.messages.len() > self.window_size {
            let excess = self.messages.len() - self.window_size;
            self.messages.drain(0..excess);
        }
    }
}

impl Default for ConversationBufferWindowMemory {
    fn default() -> Self {
        Self::with_default_window()
    }
}

impl BaseMemory for ConversationBufferWindowMemory {
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.trim_messages();
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
    fn test_window_memory_new() {
        let memory = ConversationBufferWindowMemory::new(5);
        assert_eq!(memory.window_size(), 5);
        assert!(memory.get_messages().is_empty());
    }

    #[test]
    fn test_window_memory_trimming() {
        let mut memory = ConversationBufferWindowMemory::new(3);
        memory.add_user_message("msg1");
        memory.add_ai_message("msg2");
        memory.add_user_message("msg3");
        memory.add_ai_message("msg4");
        memory.add_user_message("msg5");
        // Should keep only last 3 messages
        assert_eq!(memory.get_messages().len(), 3);
    }

    #[test]
    fn test_window_memory_default() {
        let memory = ConversationBufferWindowMemory::default();
        assert_eq!(memory.window_size(), 10);
    }

    #[test]
    fn test_window_memory_set_size() {
        let mut memory = ConversationBufferWindowMemory::new(10);
        memory.add_user_message("1");
        memory.add_ai_message("2");
        memory.add_user_message("3");
        memory.set_window_size(2);
        assert_eq!(memory.get_messages().len(), 2);
        assert_eq!(memory.window_size(), 2);
    }

    #[test]
    fn test_window_memory_no_trimming_within_limit() {
        let mut memory = ConversationBufferWindowMemory::new(10);
        memory.add_user_message("msg1");
        memory.add_ai_message("msg2");
        memory.add_user_message("msg3");
        assert_eq!(memory.get_messages().len(), 3);
    }
}
