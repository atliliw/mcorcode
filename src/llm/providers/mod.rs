//! Multi-model Support
//!
//! 借鉴 OpenCode 的多模型管理，支持 75+ providers

pub mod config;
pub mod manager;
pub mod provider;

pub use manager::ModelManager;
pub use provider::{ModelConfig, ProviderConfig, ProviderType};
