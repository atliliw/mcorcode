use crate::agent::types::AgentAction;
use crate::callbacks::{CallbackHandler, RunTree};
use async_trait::async_trait;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct FileHandler {
    file_path: PathBuf,
}

impl FileHandler {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: path.into(),
        }
    }

    fn write(&self, content: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = file.write_all(content.as_bytes());
            let _ = file.write_all(b"\n");
        }
    }
}

#[async_trait]
impl CallbackHandler for FileHandler {
    fn on_llm_start(&self, run: &RunTree) {
        self.write(&format!("[{}] LLM Start: {}", run.start_time, run.name));
    }

    fn on_llm_new_token(&self, run: &RunTree, token: &str) {
        self.write(&format!("[{}] Token: {}", run.start_time, token));
    }

    fn on_llm_end(&self, run: &RunTree, response: &str) {
        self.write(&format!(
            "[{}] LLM End: {} - {}",
            run.start_time, run.name, response
        ));
    }

    fn on_llm_error(&self, run: &RunTree, error: &str) {
        self.write(&format!(
            "[{}] LLM Error: {} - {}",
            run.start_time, run.name, error
        ));
    }

    fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &Value) {
        self.write(&format!(
            "[{}] Tool Start: {} - input: {}",
            run.start_time, tool_name, input
        ));
    }

    fn on_tool_end(&self, run: &RunTree, result: &str) {
        self.write(&format!(
            "[{}] Tool End: result: {}",
            run.start_time, result
        ));
    }

    fn on_tool_error(&self, run: &RunTree, error: &str) {
        self.write(&format!("[{}] Tool Error: {}", run.start_time, error));
    }

    fn on_agent_start(&self, run: &RunTree, input: &str) {
        self.write(&format!("[{}] Agent Start: {}", run.start_time, input));
    }

    fn on_agent_action(&self, run: &RunTree, action: &AgentAction) {
        match action {
            AgentAction::Tool { tool, .. } => {
                self.write(&format!("[{}] Agent Action: {}", run.start_time, tool));
            }
            AgentAction::Message(msg) => {
                self.write(&format!(
                    "[{}] Agent Message: {}",
                    run.start_time, msg.content
                ));
            }
        }
    }

    fn on_agent_finish(&self, run: &RunTree, output: &str) {
        self.write(&format!("[{}] Agent Finish: {}", run.start_time, output));
    }

    fn on_chain_start(&self, run: &RunTree) {
        self.write(&format!("[{}] Chain Start: {}", run.start_time, run.name));
    }

    fn on_chain_end(&self, run: &RunTree, output: &str) {
        self.write(&format!(
            "[{}] Chain End: {} - {}",
            run.start_time, run.name, output
        ));
    }
}
