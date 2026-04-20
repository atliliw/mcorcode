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
