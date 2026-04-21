//! LLM Base Types
//!
//! 提供 BaseChatModel trait 和 LLM 相关错误类型

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde_json::Value;

use crate::schema::{Message, LlmOutput, ToolCall};

/// LLM 聊天模型基础 trait
#[async_trait]
pub trait BaseChatModel: Send + Sync {
    /// 同步聊天调用
    async fn chat(&self, messages: Vec<Message>) -> Result<LlmOutput, LlmError>;
    
    /// 流式聊天调用（返回 token 流）
    async fn stream_chat(&self, messages: Vec<Message>) -> Result<BoxStream<'static, String>, LlmError>;
    
    /// 获取模型名称
    fn model_name(&self) -> &str;
    
    /// 获取可用工具定义
    fn get_tool_definitions(&self) -> Vec<ToolDefinition>;
    
    /// 设置工具定义
    fn set_tools(&mut self, tools: Vec<ToolDefinition>);
}

/// LLM 错误类型
#[derive(Debug)]
pub enum LlmError {
    /// 网络错误
    NetworkError(String),
    /// API 错误（带状态码和消息）
    ApiError { code: u16, message: String },
    /// 超时错误
    TimeoutError,
    /// 速率限制错误（带重试等待时间）
    RateLimitError { retry_after: Option<u64> },
    /// 响应格式错误
    InvalidResponse(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            LlmError::ApiError { code, message } => write!(f, "API error {}: {}", code, message),
            LlmError::TimeoutError => write!(f, "Timeout"),
            LlmError::RateLimitError { retry_after } => write!(f, "Rate limited, retry after: {:?}", retry_after),
            LlmError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl std::error::Error for LlmError {}

/// 工具定义（用于传递给 LLM API）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolDefinition {
    /// 工具类型（通常为 "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// 函数定义
    pub function: FunctionDefinition,
}

impl ToolDefinition {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: name.into(),
                description: description.into(),
                parameters: serde_json::json!({"type": "object"}),
            },
        }
    }
    
    pub fn with_parameters(mut self, params: serde_json::Value) -> Self {
        self.function.parameters = params;
        self
    }
    
    /// 转换为 OpenAI tools 格式
    pub fn to_openai_format(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    /// 转换为 Anthropic tools 格式
    pub fn to_anthropic_format(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.function.name,
            "description": self.function.description,
            "input_schema": self.function.parameters
        })
    }
}

/// 函数定义
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionDefinition {
    /// 函数名称
    pub name: String,
    /// 函数描述
    pub description: String,
    /// 参数 JSON Schema
    pub parameters: serde_json::Value,
}