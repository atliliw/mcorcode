//! Streaming 类型定义
//!
//! 提供 LLM 流式响应的类型支持

use crate::schema::FinishReason;
use serde::{Deserialize, Serialize};

pub struct StreamingChunk {
    pub content: String,
    pub tool_calls: Option<Vec<PartialToolCall>>,
    pub finish_reason: Option<FinishReason>,
}

impl StreamingChunk {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tool_calls: None,
            finish_reason: None,
        }
    }

    pub fn with_tool_calls(mut self, tool_calls: Vec<PartialToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }

    pub fn with_finish_reason(mut self, reason: FinishReason) -> Self {
        self.finish_reason = Some(reason);
        self
    }

    pub fn is_final(&self) -> bool {
        self.finish_reason.is_some()
    }

    pub fn has_content(&self) -> bool {
        !self.content.is_empty()
    }

    pub fn accumulate(&mut self, other: &StreamingChunk) {
        self.content.push_str(&other.content);

        if let Some(other_tools) = &other.tool_calls {
            match &mut self.tool_calls {
                Some(my_tools) => {
                    for other_tool in other_tools {
                        if let Some(my_tool) =
                            my_tools.iter_mut().find(|t| t.index == other_tool.index)
                        {
                            my_tool.accumulate(other_tool);
                        } else {
                            my_tools.push(other_tool.clone());
                        }
                    }
                }
                None => {
                    self.tool_calls = Some(other_tools.clone());
                }
            }
        }

        if other.finish_reason.is_some() {
            self.finish_reason = other.finish_reason.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialToolCall {
    pub index: usize,
    pub id: Option<String>,
    pub name: Option<String>,
    pub arguments: String,
}

impl PartialToolCall {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            id: None,
            name: None,
            arguments: String::new(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn append_arguments(&mut self, args: &str) {
        self.arguments.push_str(args);
    }

    pub fn accumulate(&mut self, other: &PartialToolCall) {
        if other.id.is_some() {
            self.id = other.id.clone();
        }
        if other.name.is_some() {
            self.name = other.name.clone();
        }
        self.arguments.push_str(&other.arguments);
    }

    pub fn is_complete(&self) -> bool {
        self.id.is_some() && self.name.is_some() && !self.arguments.is_empty()
    }

    pub fn try_parse_arguments(&self) -> Option<serde_json::Value> {
        if self.arguments.is_empty() {
            return None;
        }
        serde_json::from_str(&self.arguments).ok()
    }
}

pub struct StreamingState {
    chunks: Vec<StreamingChunk>,
    accumulated: StreamingChunk,
}

impl StreamingState {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            accumulated: StreamingChunk::new(String::new()),
        }
    }

    pub fn push(&mut self, chunk: StreamingChunk) {
        self.accumulated.accumulate(&chunk);
        self.chunks.push(chunk);
    }

    pub fn finalize(&self) -> StreamingResult {
        let tool_calls = self.accumulated.tool_calls.clone().and_then(|partial| {
            let complete: Vec<_> = partial
                .into_iter()
                .filter(|t| t.is_complete())
                .filter_map(|t| {
                    let args = t.try_parse_arguments()?;
                    Some(crate::schema::ToolCall::new(
                        t.id.unwrap_or_default(),
                        t.name.unwrap_or_default(),
                        args,
                    ))
                })
                .collect();
            if complete.is_empty() {
                None
            } else {
                Some(complete)
            }
        });

        StreamingResult {
            content: self.accumulated.content.clone(),
            tool_calls,
            finish_reason: self.accumulated.finish_reason.clone(),
        }
    }

    pub fn is_streaming(&self) -> bool {
        self.accumulated.finish_reason.is_none()
    }
}

impl Default for StreamingState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct StreamingResult {
    pub content: String,
    pub tool_calls: Option<Vec<crate::schema::ToolCall>>,
    pub finish_reason: Option<FinishReason>,
}
