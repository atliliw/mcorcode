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
