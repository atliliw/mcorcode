use crate::callbacks::{CallbackHandler, RunTree};
use async_trait::async_trait;

pub struct StdOutHandler {
    verbose: bool,
}

impl StdOutHandler {
    pub fn new() -> Self {
        Self { verbose: true }
    }

    pub fn quiet() -> Self {
        Self { verbose: false }
    }
}

impl Default for StdOutHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CallbackHandler for StdOutHandler {
    fn on_llm_start(&self, run: &RunTree) {
        if self.verbose {
            println!("[LLM] Starting: {}", run.name);
        }
    }

    fn on_llm_end(&self, run: &RunTree, response: &str) {
        if self.verbose {
            println!("[LLM] End: {}", run.name);
            if let Some(ms) = run.duration_ms() {
                println!("  Duration: {}ms", ms);
            }
        }
    }

    fn on_llm_error(&self, run: &RunTree, error: &str) {
        println!("[LLM] Error in {}: {}", run.name, error);
    }

    fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &str) {
        if self.verbose {
            println!("[Tool] {} starting", tool_name);
        }
    }

    fn on_tool_end(&self, run: &RunTree, result: &str) {
        if self.verbose {
            println!("[Tool] Completed");
        }
    }

    fn on_tool_error(&self, run: &RunTree, error: &str) {
        println!("[Tool] Error: {}", error);
    }

    fn on_chain_start(&self, run: &RunTree) {
        if self.verbose {
            println!("[Chain] Starting: {}", run.name);
        }
    }

    fn on_chain_end(&self, run: &RunTree, output: &str) {
        if self.verbose {
            println!("[Chain] End: {}", run.name);
        }
    }
}
