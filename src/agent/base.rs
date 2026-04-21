//! Agent 基础 Trait 定义
//!
//! 定义 Agent 的核心接口，包括规划、执行和系统提示词

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::tools::Tool;
use super::types::{AgentOutput, AgentError};

/// Agent 基础 trait
/// 
/// 所有 Agent 实现都需要提供规划能力和系统提示词
#[async_trait]
pub trait BaseAgent: Send + Sync {
    /// 规划下一步行动
    /// 
    /// 根据中间步骤和输入，决定下一步是执行工具还是完成任务
    async fn plan(
        &self,
        intermediate_steps: &[super::types::AgentStep],
        inputs: &HashMap<String, String>,
    ) -> Result<AgentOutput, AgentError>;
    
    /// 获取 Agent 的系统提示词
    fn system_prompt(&self) -> String;
    
    /// 获取可用工具列表
    fn get_tools(&self) -> &[Arc<dyn Tool>];
    
    /// 获取 Agent 名称
    fn name(&self) -> &str;
}