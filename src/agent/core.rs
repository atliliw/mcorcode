use crate::llm::{LlmClient, Message};
use crate::tools::{Tool, ToolRegistry};
use crate::context::ContextManager;
use std::sync::Arc;
use anyhow::Result;

pub struct Agent {
    llm_client: LlmClient,
    tool_registry: ToolRegistry,
    context_manager: ContextManager,
    max_iterations: usize,
}

impl Agent {
    pub fn new(llm_client: LlmClient, tool_registry: ToolRegistry) -> Self {
        Self {
            llm_client,
            tool_registry,
            context_manager: ContextManager::new(),
            max_iterations: 50,
        }
    }

    pub async fn run(&mut self, prompt: &str) -> Result<String> {
        self.context_manager.add_user_message(prompt);

        for iteration in 0..self.max_iterations {
            let messages = self.context_manager.get_messages();
            let response = self.llm_client.chat(messages).await?;

            if let Some(tool_calls) = &response.tool_calls {
                if tool_calls.is_empty() {
                    return Ok(response.content);
                }

                self.context_manager.add_assistant_message(&response);

                for tool_call in tool_calls {
                    let result = self.execute_tool(tool_call).await?;
                    self.context_manager.add_tool_result(&tool_call.id, &result);
                }
            } else {
                return Ok(response.content);
            }
        }

        Err(anyhow::anyhow!("Max iterations reached"))
    }

    async fn execute_tool(&self, tool_call: &crate::llm::ToolCall) -> Result<String> {
        if let Some(tool) = self.tool_registry.get(&tool_call.name) {
            tool.execute(tool_call.arguments.clone()).await
        } else {
            Ok(format!("Unknown tool: {}", tool_call.name))
        }
    }

    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tool_registry.register(tool);
    }
}