//! Base memory trait and error types
//!
//! Reference: langchainrust/langchainrust/src/memory/base.rs

use crate::schema::Message;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
pub enum MemoryError {
    SerializationError(String),
    DeserializationError(String),
    StorageError(String),
    Other(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            MemoryError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            MemoryError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            MemoryError::Other(msg) => write!(f, "Memory error: {}", msg),
        }
    }
}

impl std::error::Error for MemoryError {}

/// 内存基础 trait
#[async_trait]
pub trait BaseMemory: Send + Sync {
    /// 添加消息
    fn add_message(&mut self, message: Message);

    /// 添加用户消息
    fn add_user_message(&mut self, content: &str);

    /// 添加 AI 消息
    fn add_ai_message(&mut self, content: &str);

    /// 添加工具结果消息
    fn add_tool_result(&mut self, tool_call_id: &str, result: &str);

    /// 获取所有消息
    fn get_messages(&self) -> &[Message];

    /// 清空内存
    fn clear(&mut self);

    /// 计算 token 数量（简单估算）
    fn token_count(&self) -> usize {
        self.get_messages().iter().map(|m| m.token_estimate()).sum()
    }

    /// 裁剪到 token 限制
    fn trim_to_token_limit(&mut self, max_tokens: usize);

    /// 获取最近 N 条消息
    fn get_last_n(&self, n: usize) -> &[Message] {
        let messages = self.get_messages();
        let start = messages.len().saturating_sub(n);
        &messages[start..]
    }

    /// 序列化为 JSON
    fn to_json(&self) -> Result<String, MemoryError> {
        serde_json::to_string(self.get_messages())
            .map_err(|e| MemoryError::SerializationError(e.to_string()))
    }

    /// 从 JSON 反序列化（默认实现返回错误）
    fn from_json(json: &str) -> Result<Self, MemoryError>
    where
        Self: Sized,
    {
        Err(MemoryError::DeserializationError(
            "from_json not implemented".to_string(),
        ))
    }

    /// 加载内存
    fn load(&mut self) -> Result<(), MemoryError> {
        Ok(())
    }

    /// 保存内存
    fn save(&self) -> Result<(), MemoryError> {
        Ok(())
    }
}
