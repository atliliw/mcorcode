//! LLM Base Types

use async_trait::async_trait;
use serde_json::Value;

use crate::schema::{Message, ToolCall};

#[async_trait]
pub trait BaseChatModel: Send + Sync {
    async fn chat(&self, messages: Vec<Message>, config: Option<Value>) -> Result<LlmOutput, LlmError>;
    fn model_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct LlmOutput {
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: FinishReason,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishReason {
    Stop,
    ToolCalls,
    Length,
    ContentFilter,
    Error,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug)]
pub enum LlmError {
    NetworkError(String),
    ApiError { code: u16, message: String },
    TimeoutError,
    RateLimitError { retry_after: Option<u64> },
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