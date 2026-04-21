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

    /// 从 JSON 反序列化
    pub fn from_json(json: &str) -> Result<Self, MemoryError> {
        let messages: Vec<Message> = serde_json::from_str(json)
            .map_err(|e| MemoryError::DeserializationError(e.to_string()))?;
        Ok(Self { messages })
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
        // Buffer memory 默认不裁剪，保留所有消息
        // 但如果超过限制，移除最旧的非系统消息
        while self.token_count() > max_tokens && self.messages.len() > 1 {
            // 找到第一个非系统消息并移除
            let idx = self.messages.iter().position(|m| !m.is_system());
            if let Some(i) = idx {
                self.messages.remove(i);
            } else {
                break;
            }
        }
    }

    fn from_json(json: &str) -> Result<Self, MemoryError> {
        ConversationBufferMemory::from_json(json)
    }
}
