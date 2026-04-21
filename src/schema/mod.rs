//! Core schema types for mcorcode
//!
//! Provides foundational types used across all modules.

pub mod document;
pub mod message;

pub use document::Document;
pub use message::{Message, MessageType, ToolCall};
