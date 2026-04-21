//! Multi-model Support
//!
//! 借鉴 OpenCode 的多模型管理，支持 75+ providers

pub mod provider;
pub mod manager;
pub mod config;

pub use provider::{ProviderType, ProviderConfig};
pub use manager::ModelManager;
pub use config::ModelConfig;