//! Core schema types for mcorcode
//!
//! Provides foundational types used across all modules.

pub mod message;
pub mod document;

pub use message::{Message, MessageType};
pub use document::Document;