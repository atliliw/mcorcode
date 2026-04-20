//! Chat message history - simple message storage
//!
//! Reference: langchainrust/langchainrust/src/memory/history.rs

use crate::schema::Message;
use serde::{Deserialize, Serialize};

pub struct ChatMessageHistory {
    messages: Vec<Message>,
}

impl ChatMessageHistory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn from_messages(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.messages)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let messages: Vec<Message> = serde_json::from_str(json)?;
        Ok(Self { messages })
    }
}

impl Default for ChatMessageHistory {
    fn default() -> Self {
        Self::new()
    }
}
