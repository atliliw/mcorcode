use crate::callbacks::{CallbackHandler, RunTree};
use async_trait::async_trait;
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

    fn on_tool_start(&self, run: &RunTree, tool_name: &str, input: &str) {
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
