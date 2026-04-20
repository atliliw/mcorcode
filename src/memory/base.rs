//! Base memory trait and error types
//!
//! Reference: langchainrust/langchainrust/src/memory/base.rs

use crate::schema::Message;
use async_trait::async_trait;

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

#[async_trait]
pub trait BaseMemory: Send + Sync {
    fn add_message(&mut self, message: Message);
    fn add_user_message(&mut self, content: &str);
    fn add_ai_message(&mut self, content: &str);
    fn get_messages(&self) -> &[Message];
    fn clear(&mut self);

    fn load(&mut self) -> Result<(), MemoryError> {
        Ok(())
    }

    fn save(&self) -> Result<(), MemoryError> {
        Ok(())
    }
}
