//! 公共配置模块
//!
//! 集中管理 API 配置和常量

use std::env;

/// OpenAI API 配置
pub struct OpenAIConfig {
    /// API Key - 从环境变量 OPENAI_API_KEY 读取，或使用默认值
    pub api_key: String,
    /// Base URL - API 代理地址
    pub base_url: String,
    /// 默认聊天模型
    pub default_model: String,
    /// Embedding 模型
    pub embedding_model: String,
}

impl OpenAIConfig {
    /// 从环境变量或默认值创建配置
    pub fn from_env_or_default() -> Self {
        Self {
            api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
                "sk-l0YYMX65mCYRlTJYH0ptf4BFpqJwm8Xo9Z5IMqSZD0yOafl6".to_string()
            }),
            base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai-proxy.org/v1".to_string()),
            default_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
            embedding_model: env::var("OPENAI_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "text-embedding-ada-002".to_string()),
        }
    }

    /// 使用默认硬编码配置（用于测试）
    pub fn default_test() -> Self {
        Self {
            api_key: "sk-l0YYMX65mCYRlTJYH0ptf4BFpqJwm8Xo9Z5IMqSZD0yOafl6".to_string(),
            base_url: "https://api.openai-proxy.org/v1".to_string(),
            default_model: "gpt-3.5-turbo".to_string(),
            embedding_model: "text-embedding-ada-002".to_string(),
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self::from_env_or_default()
    }
}

/// Anthropic API 配置
pub struct AnthropicConfig {
    /// API Key - 从环境变量 ANTHROPIC_API_KEY 读取
    pub api_key: String,
    /// Base URL - API 地址
    pub base_url: String,
    /// 默认模型
    pub default_model: String,
}

impl AnthropicConfig {
    pub fn from_env_or_default() -> Self {
        Self {
            api_key: env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
            base_url: env::var("ANTHROPIC_BASE_URL")
                .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string()),
            default_model: env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string()),
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self::from_env_or_default()
    }
}

/// 全局应用配置
pub struct AppConfig {
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
    /// 最大 token 数
    pub max_tokens: usize,
    /// 温度参数
    pub temperature: f32,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            openai: OpenAIConfig::default(),
            anthropic: AnthropicConfig::default(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }

    /// 用于测试的配置
    pub fn test_config() -> Self {
        Self {
            openai: OpenAIConfig::default_test(),
            anthropic: AnthropicConfig::default(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 常量定义
pub mod constants {
    /// 默认最大迭代次数
    pub const MAX_ITERATIONS: usize = 25;

    /// 默认窗口大小
    pub const DEFAULT_WINDOW_SIZE: usize = 10;

    /// Token 估算比例 (约 4 字符 = 1 token)
    pub const TOKEN_RATIO: usize = 4;

    /// 默认超时时间 (秒)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 60;

    /// 流式响应超时 (秒)
    pub const STREAM_TIMEOUT_SECS: u64 = 120;
}
