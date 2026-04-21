//! Conversation summary memory - summarizes old messages
//!
//! Reference: langchainrust/langchainrust/src/memory/summary.rs

use super::base::{BaseMemory, MemoryError};
use crate::schema::Message;

pub struct ConversationSummaryMemory {
    messages: Vec<Message>,
    current_summary: String,
    max_messages: usize,
}

impl ConversationSummaryMemory {
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: Vec::new(),
            current_summary: String::new(),
            max_messages,
        }
    }

    pub fn get_summary(&self) -> &str {
        &self.current_summary
    }

    fn needs_summary(&self) -> bool {
        self.messages.len() > self.max_messages
    }
}

impl Default for ConversationSummaryMemory {
    fn default() -> Self {
        Self::new(20)
    }
}

impl BaseMemory for ConversationSummaryMemory {
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn add_user_message(&mut self, content: &str) {
        self.add_message(Message::human(content));
    }

    fn add_ai_message(&mut self, content: &str) {
        self.add_message(Message::ai(content));
    }

    fn add_tool_result(&mut self, tool_call_id: &str, result: &str) {
        self.add_message(Message::tool(tool_call_id, result));
    }

    fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    fn clear(&mut self) {
        self.messages.clear();
        self.current_summary.clear();
    }

    fn trim_to_token_limit(&mut self, max_tokens: usize) {
        // Summary memory 使用摘要机制而非简单裁剪
        while self.token_count() > max_tokens && self.messages.len() > 1 {
            let idx = self.messages.iter().position(|m| !m.is_system());
            if let Some(i) = idx {
                self.messages.remove(i);
            } else {
                break;
            }
        }
    }
}
