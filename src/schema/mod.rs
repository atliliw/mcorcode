//! Core schema types for mcorcode
//!
//! Provides foundational types used across all modules.

pub mod document;
pub mod message;
pub mod output;

pub use document::Document;
pub use message::{Message, MessageType, ToolCall};
pub use output::{FinishReason, LlmOutput, TokenUsage};
