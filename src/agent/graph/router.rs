use crate::agent::graph::state::McorcodeState;
use async_trait::async_trait;
use langchainrust::langgraph::edge::ConditionalEdge;
use langchainrust::langgraph::errors::GraphError;

pub struct AgentRouter;

#[async_trait::async_trait]
impl ConditionalEdge<McorcodeState> for AgentRouter {
    async fn route(&self, state: &McorcodeState) -> Result<String, GraphError> {
        if state.has_tool_calls() {
            return Ok("tools".to_string());
        }
        
        if state.has_final_output() {
            return Ok("end".to_string());
        }
        
        if state.should_compact() {
            return Ok("compact".to_string());
        }
        
        if state.reached_max_iterations() {
            return Ok("end".to_string());
        }
        
        Ok("end".to_string())
    }
}

pub struct ToolsRouter;

#[async_trait::async_trait]
impl ConditionalEdge<McorcodeState> for ToolsRouter {
    async fn route(&self, state: &McorcodeState) -> Result<String, GraphError> {
        if state.reached_max_iterations() {
            return Ok("end".to_string());
        }
        Ok("llm".to_string())
    }
}

pub struct CompactRouter;

#[async_trait::async_trait]
impl ConditionalEdge<McorcodeState> for CompactRouter {
    async fn route(&self, _state: &McorcodeState) -> Result<String, GraphError> {
        Ok("llm".to_string())
    }
}