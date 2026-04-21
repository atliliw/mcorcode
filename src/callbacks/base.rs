use super::run_tree::RunTree;
use crate::agent::types::AgentAction;
use crate::schema::LlmOutput;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait CallbackHandler: Send + Sync {
    fn on_llm_start(&self, run: &RunTree);
    fn on_llm_new_token(&self, run: &RunTree, token: &str);
    fn on_llm_end(&self, run: &RunTree, response: &str);
    fn on_llm_error(&self, run: &RunTree, error: &str);

    fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &Value);
    fn on_tool_end(&self, run: &RunTree, result: &str);
    fn on_tool_error(&self, run: &RunTree, error: &str);

    fn on_agent_start(&self, run: &RunTree, input: &str);
    fn on_agent_action(&self, run: &RunTree, action: &AgentAction);
    fn on_agent_finish(&self, run: &RunTree, output: &str);

    fn on_chain_start(&self, run: &RunTree);
    fn on_chain_end(&self, run: &RunTree, output: &str);
}

pub struct CallbackManager {
    handlers: Vec<Box<dyn CallbackHandler>>,
}

impl CallbackManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn CallbackHandler>) {
        self.handlers.push(handler);
    }

    pub fn on_llm_start(&self, run: &RunTree) {
        for handler in &self.handlers {
            handler.on_llm_start(run);
        }
    }

    pub fn on_llm_new_token(&self, run: &RunTree, token: &str) {
        for handler in &self.handlers {
            handler.on_llm_new_token(run, token);
        }
    }

    pub fn on_llm_end(&self, run: &RunTree, response: &str) {
        for handler in &self.handlers {
            handler.on_llm_end(run, response);
        }
    }

    pub fn on_llm_error(&self, run: &RunTree, error: &str) {
        for handler in &self.handlers {
            handler.on_llm_error(run, error);
        }
    }

    pub fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &Value) {
        for handler in &self.handlers {
            handler.on_tool_start(run, tool_name, input);
        }
    }

    pub fn on_tool_end(&self, run: &RunTree, result: &str) {
        for handler in &self.handlers {
            handler.on_tool_end(run, result);
        }
    }

    pub fn on_tool_error(&self, run: &RunTree, error: &str) {
        for handler in &self.handlers {
            handler.on_tool_error(run, error);
        }
    }

    pub fn on_agent_start(&self, run: &RunTree, input: &str) {
        for handler in &self.handlers {
            handler.on_agent_start(run, input);
        }
    }

    pub fn on_agent_action(&self, run: &RunTree, action: &AgentAction) {
        for handler in &self.handlers {
            handler.on_agent_action(run, action);
        }
    }

    pub fn on_agent_finish(&self, run: &RunTree, output: &str) {
        for handler in &self.handlers {
            handler.on_agent_finish(run, output);
        }
    }

    pub fn on_chain_start(&self, run: &RunTree) {
        for handler in &self.handlers {
            handler.on_chain_start(run);
        }
    }

    pub fn on_chain_end(&self, run: &RunTree, output: &str) {
        for handler in &self.handlers {
            handler.on_chain_end(run, output);
        }
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}
