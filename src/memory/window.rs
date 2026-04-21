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

    /// 从 JSON 反序列化
    pub fn from_json_with_window(json: &str, window_size: usize) -> Result<Self, MemoryError> {
        let messages: Vec<Message> = serde_json::from_str(json)
            .map_err(|e| MemoryError::DeserializationError(e.to_string()))?;
        Ok(Self {
            messages,
            window_size,
        })
    }

    fn trim_messages(&mut self) {
        if self.messages.len() > self.window_size {
            // 保留系统消息
            let system_msgs: Vec<Message> = self
                .messages
                .iter()
                .filter(|m| m.is_system())
                .cloned()
                .collect();

            let non_system_count = self.messages.len() - system_msgs.len();
            let excess = non_system_count.saturating_sub(self.window_size);

            if excess > 0 {
                // 移除最旧的非系统消息
                let mut new_messages = system_msgs;
                let non_system: Vec<Message> = self
                    .messages
                    .iter()
                    .filter(|m| !m.is_system())
                    .skip(excess)
                    .cloned()
                    .collect();
                new_messages.extend(non_system);
                self.messages = new_messages;
            }
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

    fn add_tool_result(&mut self, tool_call_id: &str, result: &str) {
        self.add_message(Message::tool(tool_call_id, result));
    }

    fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    fn clear(&mut self) {
        self.messages.clear();
    }

    fn trim_to_token_limit(&mut self, max_tokens: usize) {
        while self.token_count() > max_tokens && self.messages.len() > 1 {
            // 移除最旧的非系统消息
            let idx = self.messages.iter().position(|m| !m.is_system());
            if let Some(i) = idx {
                self.messages.remove(i);
            } else {
                break;
            }
        }
    }

    fn from_json(json: &str) -> Result<Self, MemoryError> {
        ConversationBufferWindowMemory::from_json_with_window(json, 10)
    }
}
