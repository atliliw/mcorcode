//! Agent 执行器
//!
//! 实现 Agent 的执行循环，处理 LLM 调用和工具执行

use std::sync::Arc;
use std::time::Duration;

use tokio::time::timeout;

use crate::llm::BaseChatModel;
use crate::memory::BaseMemory;
use crate::schema::{FinishReason, ToolCall};
use crate::tools::Tool;
use crate::callbacks::CallbackManager;
use crate::callbacks::run_tree::{RunTree, RunType};

use super::base::BaseAgent;
use super::types::{AgentError, AgentAction, AgentStep};

const DEFAULT_MAX_ITERATIONS: usize = 10;
const DEFAULT_TIMEOUT_SECS: u64 = 300;

pub struct AgentExecutor {
    agent: Arc<dyn BaseAgent>,
    llm: Box<dyn BaseChatModel>,
    memory: Box<dyn BaseMemory>,
    callbacks: Option<CallbackManager>,
    max_iterations: usize,
    timeout_duration: Duration,
}

impl AgentExecutor {
    pub fn new(agent: Arc<dyn BaseAgent>, llm: Box<dyn BaseChatModel>) -> Self {
        Self {
            agent,
            llm,
            memory: Box::new(crate::memory::ConversationBufferMemory::new()),
            callbacks: None,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            timeout_duration: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn with_memory(mut self, memory: Box<dyn BaseMemory>) -> Self {
        self.memory = memory;
        self
    }

    pub fn with_callbacks(mut self, callbacks: CallbackManager) -> Self {
        self.callbacks = Some(callbacks);
        self
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_duration = Duration::from_secs(secs);
        self
    }

    /// 执行 Agent 循环
    pub async fn run(&mut self, input: &str) -> Result<String, AgentError> {
        self.memory.add_user_message(input);
        
        let run_tree = RunTree::new(RunType::Agent, "agent_run");

        if let Some(cb) = &self.callbacks {
            cb.on_chain_start(&run_tree);
        }

        let result = timeout(self.timeout_duration, self.run_loop(&run_tree))
            .await
            .map_err(|_| AgentError::TimeoutReached)?;

        if let Some(cb) = &self.callbacks {
            if let Ok(output) = &result {
                cb.on_chain_end(&run_tree, output);
            }
        }

        result
    }

    async fn run_loop(&mut self, run_tree: &RunTree) -> Result<String, AgentError> {
        let mut intermediate_steps: Vec<AgentStep> = Vec::new();

        for iteration in 0..self.max_iterations {
            let messages = self.memory.get_messages().to_vec();
            
            if let Some(cb) = &self.callbacks {
                cb.on_llm_start(run_tree);
            }

            let output = self.llm.chat(messages).await
                .map_err(|e| AgentError::LlmError(e.to_string()))?;

            if let Some(cb) = &self.callbacks {
                cb.on_llm_end(run_tree, &output.content);
            }

            match output.finish_reason {
                FinishReason::Stop => {
                    self.memory.add_ai_message(&output.content);
                    return Ok(output.content);
                }
                FinishReason::ToolCalls => {
                    let tool_calls = output.tool_calls.unwrap_or_default();
                    
                    for tool_call in tool_calls {
                        let result = self.execute_tool(&tool_call, run_tree).await?;
                        
                        let step = AgentStep {
                            action: AgentAction::Tool {
                                tool: tool_call.name.clone(),
                                tool_input: tool_call.arguments.clone(),
                                log: String::new(),
                            },
                            observation: result.clone(),
                        };
                        intermediate_steps.push(step);
                        
                        self.memory.add_tool_result(&tool_call.id, &result);
                    }
                }
                FinishReason::Length => {
                    return Err(AgentError::OutputParsingError("Response truncated due to length limit".to_string()));
                }
                FinishReason::ContentFilter => {
                    return Err(AgentError::LlmError("Content filtered by API".to_string()));
                }
                FinishReason::Error => {
                    return Err(AgentError::LlmError("API returned error".to_string()));
                }
            }
        }

        Err(AgentError::MaxIterationsReached)
    }

    async fn execute_tool(&self, tool_call: &ToolCall, run_tree: &RunTree) -> Result<String, AgentError> {
        let tools = self.agent.get_tools();
        let tool = tools.iter()
            .find(|t| t.name() == tool_call.name)
            .ok_or_else(|| AgentError::InvalidToolCall(format!("Tool '{}' not found", tool_call.name)))?;

        if let Some(cb) = &self.callbacks {
            cb.on_tool_start(run_tree, tool.name(), &tool_call.arguments);
        }

        let result = tool.execute(tool_call.arguments.clone())
            .await
            .map_err(|e| AgentError::ToolExecutionError {
                tool: tool_call.name.clone(),
                error: e.to_string(),
            })?;

        if let Some(cb) = &self.callbacks {
            cb.on_tool_end(run_tree, &result);
        }

        Ok(result)
    }

    pub fn get_memory(&self) -> &dyn BaseMemory {
        self.memory.as_ref()
    }

    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }
}