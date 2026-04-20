use super::run_tree::RunTree;
use async_trait::async_trait;

#[async_trait]
pub trait CallbackHandler: Send + Sync {
    fn on_llm_start(&self, run: &RunTree);
    fn on_llm_end(&self, run: &RunTree, response: &str);
    fn on_llm_error(&self, run: &RunTree, error: &str);

    fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &str);
    fn on_tool_end(&self, run: &RunTree, result: &str);
    fn on_tool_error(&self, run: &RunTree, error: &str);

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

    pub fn on_llm_end(&self, run: &RunTree, response: &str) {
        for handler in &self.handlers {
            handler.on_llm_end(run, response);
        }
    }

    pub fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &str) {
        for handler in &self.handlers {
            handler.on_tool_start(run, tool_name, input);
        }
    }

    pub fn on_tool_end(&self, run: &RunTree, result: &str) {
        for handler in &self.handlers {
            handler.on_tool_end(run, result);
        }
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}
