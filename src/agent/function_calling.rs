//! Function Calling Agent 实现
//!
//! 基于 LLM Function Calling 机制的 Agent 实现

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::tools::Tool;

use super::base::BaseAgent;
use super::types::{AgentOutput, AgentError, AgentAction, AgentStep, AgentFinish};

const DEFAULT_SYSTEM_PROMPT: &str = "You are an AI assistant with access to tools for file operations, code editing, and shell commands. Use these tools to help the user accomplish their tasks efficiently.

When using tools:
1. Explain what you're doing before taking action
2. Use the appropriate tool for each task
3. Handle errors gracefully and report them clearly";

pub struct FunctionCallingAgent {
    name: String,
    tools: Vec<Arc<dyn Tool>>,
    system_prompt: String,
}

impl FunctionCallingAgent {
    pub fn new(name: impl Into<String>, tools: Vec<Arc<dyn Tool>>) -> Self {
        Self {
            name: name.into(),
            tools,
            system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
        }
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    pub fn add_tool(mut self, tool: Arc<dyn Tool>) -> Self {
        self.tools.push(tool);
        self
    }
}

#[async_trait]
impl BaseAgent for FunctionCallingAgent {
    async fn plan(
        &self,
        intermediate_steps: &[AgentStep],
        inputs: &HashMap<String, String>,
    ) -> Result<AgentOutput, AgentError> {
        let input = inputs.get("input")
            .or_else(|| inputs.get("question"))
            .ok_or_else(|| AgentError::OutputParsingError("No input provided".to_string()))?;

        if intermediate_steps.is_empty() {
            return Ok(AgentOutput::Finish(AgentFinish::new(
                input,
                "First iteration - waiting for LLM response"
            )));
        }

        let last_step = intermediate_steps.last().unwrap();
        match &last_step.action {
            AgentAction::Tool { tool, tool_input, .. } => {
                Ok(AgentOutput::Finish(AgentFinish::new(
                    format!("Tool '{}' executed with result: {}", tool, last_step.observation),
                    "Tool execution completed"
                )))
            }
            AgentAction::Message(msg) => {
                Ok(AgentOutput::Finish(AgentFinish::new(
                    msg.content.clone(),
                    "Direct message response"
                )))
            }
        }
    }

    fn system_prompt(&self) -> String {
        self.system_prompt.clone()
    }

    fn get_tools(&self) -> &[Arc<dyn Tool>] {
        &self.tools
    }

    fn name(&self) -> &str {
        &self.name
    }
}