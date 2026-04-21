//! LLM Output types
//!
//! Defines output structures from LLM API responses

use crate::schema::ToolCall;
use serde::{Deserialize, Serialize};

/// LLM 输出完成原因
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinishReason {
    /// 正常结束，LLM 完成响应
    #[serde(rename = "stop")]
    Stop,
    /// LLM 返回工具调用
    #[serde(rename = "tool_calls")]
    ToolCalls,
    /// 达到 max_tokens 限制
    #[serde(rename = "length")]
    Length,
    /// 内容过滤触发
    #[serde(rename = "content_filter")]
    ContentFilter,
    /// 发生错误
    #[serde(rename = "error")]
    Error,
}

impl FinishReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            FinishReason::Stop => "stop",
            FinishReason::ToolCalls => "tool_calls",
            FinishReason::Length => "length",
            FinishReason::ContentFilter => "content_filter",
            FinishReason::Error => "error",
        }
    }

    /// 是否为正常结束
    pub fn is_stop(&self) -> bool {
        matches!(self, FinishReason::Stop)
    }

    /// 是否需要执行工具
    pub fn needs_tool_execution(&self) -> bool {
        matches!(self, FinishReason::ToolCalls)
    }
}

impl std::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Token 使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// 输入 token 数
    pub prompt_tokens: usize,
    /// 输出 token 数
    pub completion_tokens: usize,
    /// 总 token 数
    pub total_tokens: usize,
}

impl TokenUsage {
    pub fn new(prompt: usize, completion: usize) -> Self {
        Self {
            prompt_tokens: prompt,
            completion_tokens: completion,
            total_tokens: prompt + completion,
        }
    }

    pub fn zero() -> Self {
        Self {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        }
    }
}

/// LLM API 响应输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmOutput {
    /// 响应内容文本
    pub content: String,
    /// 工具调用列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// 完成原因
    pub finish_reason: FinishReason,
    /// Token 使用统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

impl LlmOutput {
    /// 创建文本响应输出
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tool_calls: None,
            finish_reason: FinishReason::Stop,
            usage: None,
        }
    }

    /// 创建带工具调用的输出
    pub fn with_tools(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            content: content.into(),
            tool_calls: Some(tool_calls),
            finish_reason: FinishReason::ToolCalls,
            usage: None,
        }
    }

    /// 设置 token 使用统计
    pub fn with_usage(mut self, usage: TokenUsage) -> Self {
        self.usage = Some(usage);
        self
    }

    /// 是否有工具调用
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls.as_ref().map_or(false, |tc| !tc.is_empty())
    }

    /// 是否为正常结束
    pub fn is_finished(&self) -> bool {
        self.finish_reason.is_stop()
    }

    /// 获取工具调用列表
    pub fn get_tool_calls(&self) -> &[ToolCall] {
        self.tool_calls
            .as_ref()
            .map(|tc| tc.as_slice())
            .unwrap_or(&[])
    }
}
