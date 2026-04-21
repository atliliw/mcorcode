use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;
use std::pin::Pin;
use std::future::Future;

use langchainrust::langgraph::node::{AsyncFn, NodeResult};
use langchainrust::langgraph::errors::GraphError;
use langchainrust::langgraph::state::StateUpdate;
use langchainrust::core::tools::BaseTool;
use langchainrust::core::language_models::{BaseChatModel, LLMResult};
use langchainrust::language_models::openai::OpenAIError;
use langchainrust::RunnableConfig;
use langchainrust::schema::Message;

use crate::agent::graph::state::{McorcodeState, StateStep, StateMessage, MessageRole};
use crate::permission::{PermissionChecker, PermissionAction};

pub struct LlmNode {
    llm: Arc<dyn BaseChatModel<Error = OpenAIError>>,
}

impl LlmNode {
    pub fn new(llm: Arc<dyn BaseChatModel<Error = OpenAIError>>) -> Self {
        Self { llm }
    }
}

impl AsyncFn<McorcodeState> for LlmNode {
    fn call(&self, state: &McorcodeState) -> Pin<Box<dyn Future<Output = NodeResult<McorcodeState>> + Send>> {
        let llm = self.llm.clone();
        let state = state.clone();
        
        Box::pin(async move {
            let messages = state.to_llm_messages();
            
            let response = llm.chat(messages, None).await
                .map_err(|e| GraphError::ExecutionError(format!("LLM error: {}", e)))?;
            
            let mut new_state = state.clone();
            new_state.iteration += 1;
            
            if let Some(tool_calls) = &response.tool_calls {
                if !tool_calls.is_empty() {
                    new_state.tool_calls_queue = tool_calls.iter().map(|tc| crate::schema::ToolCall {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                        arguments: serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::Value::Null),
                    }).collect();
                    new_state.messages.push(StateMessage::ai_with_tools(response.content.clone(), new_state.tool_calls_queue.clone()));
                } else {
                    new_state.final_output = Some(response.content.clone());
                    new_state.should_continue = false;
                    new_state.messages.push(StateMessage::ai(response.content.clone()));
                }
            } else {
                new_state.final_output = Some(response.content.clone());
                new_state.should_continue = false;
                new_state.messages.push(StateMessage::ai(response.content.clone()));
            }
            
            Ok(StateUpdate::full(new_state))
        })
    }
}

pub struct ToolsNode {
    tools: HashMap<String, Arc<dyn BaseTool>>,
    permission_checker: Arc<PermissionChecker>,
}

impl ToolsNode {
    pub fn new(
        tools: HashMap<String, Arc<dyn BaseTool>>,
        permission_checker: Arc<PermissionChecker>,
    ) -> Self {
        Self { tools, permission_checker }
    }
}

impl AsyncFn<McorcodeState> for ToolsNode {
    fn call(&self, state: &McorcodeState) -> Pin<Box<dyn Future<Output = NodeResult<McorcodeState>> + Send>> {
        let tools = self.tools.clone();
        let permission_checker = self.permission_checker.clone();
        let state = state.clone();
        
        Box::pin(async move {
            let mut new_state = state.clone();
            
            for tool_call in &state.tool_calls_queue {
                let tool_name = &tool_call.name;
                let input = &tool_call.arguments;
                
                let perm_result = permission_checker.check(tool_name, input);
                if perm_result.action == PermissionAction::Deny {
                    new_state.messages.push(StateMessage::tool(
                        tool_call.id.clone(),
                        perm_result.reason.clone().unwrap_or_else(|| "Permission denied".to_string())
                    ));
                    continue;
                }
                
                let tool = tools.get(tool_name);
                if tool.is_none() {
                    new_state.messages.push(StateMessage::tool(
                        tool_call.id.clone(),
                        format!("Tool '{}' not found", tool_name)
                    ));
                    continue;
                }
                
                let start = Instant::now();
                let input_str = serde_json::to_string(input).unwrap_or_default();
                let result = tool.unwrap().run(input_str).await;
                let duration_ms = start.elapsed().as_millis() as u64;
                
                match result {
                    Ok(output) => {
                        let step = StateStep::new(tool_name.clone(), input.clone(), output.clone(), duration_ms);
                        new_state.add_step(step);
                        new_state.messages.push(StateMessage::tool(tool_call.id.clone(), output));
                    }
                    Err(e) => {
                        new_state.messages.push(StateMessage::tool(tool_call.id.clone(), format!("Error: {}", e)));
                    }
                }
            }
            
            new_state.tool_calls_queue.clear();
            
            Ok(StateUpdate::full(new_state))
        })
    }
}

pub struct CompactNode {
    max_messages: usize,
}

impl CompactNode {
    pub fn new(max_messages: usize) -> Self {
        Self { max_messages }
    }
}

impl AsyncFn<McorcodeState> for CompactNode {
    fn call(&self, state: &McorcodeState) -> Pin<Box<dyn Future<Output = NodeResult<McorcodeState>> + Send>> {
        let max_messages = self.max_messages;
        let state = state.clone();
        
        Box::pin(async move {
            let mut new_state = state.clone();
            
            let system_msgs: Vec<_> = new_state.messages.iter()
                .filter(|m| m.role == MessageRole::System)
                .cloned()
                .collect();
            
            let recent_msgs: Vec<_> = new_state.messages.iter()
                .rev()
                .take(max_messages)
                .cloned()
                .collect();
            
            new_state.messages = system_msgs;
            new_state.messages.extend(recent_msgs.into_iter().rev());
            new_state.needs_compact = false;
            
            Ok(StateUpdate::full(new_state))
        })
    }
}