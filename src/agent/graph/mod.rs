//! Mcorcode Agent Graph
//!
//! 使用 LangGraph 替代 Claude Code 的 QueryEngine，实现核心 Agent 执行循环。
//!
//! ## 架构对比
//!
//! | Claude Code QueryEngine | LangGraph |
//! |-------------------------|-----------|
//! | while (stop_reason !== 'end') | Conditional edges |
//! | yield tokens | StreamEvent |
//! | execute_tool() | tools_node |
//! | messages.push(tool_result) | AppendMessagesReducer |
//! | 状态持久化 (手动) | Checkpointer (自动) |
//! | 中断恢复 (手动) | interrupt_before/after |

use std::sync::Arc;
use std::collections::HashMap;

use langchainrust::langgraph::{
    StateGraph, CompiledGraph, START, END,
    StateSchema, StateUpdate, Reducer, AppendMessagesReducer,
    Checkpointer, FileCheckpointer, StreamEvent,
};
use langchainrust::core::tools::{BaseTool, ToolError, ToolDefinition, to_tool_definition};
use langchainrust::core::language_models::BaseChatModel;
use langchainrust::schema::Message;

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::schema::{MessageType, ToolCall};
use crate::permission::{PermissionChecker, PermissionMode, PermissionResult, PermissionAction};
use crate::hooks::{HookSystem, HookTrigger};

pub mod state;
pub mod builder;
pub mod nodes;
pub mod router;

pub use state::{McorcodeState, StateMessage, StateStep, MessageRole};
pub use builder::AgentGraphBuilder;