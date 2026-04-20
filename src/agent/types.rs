use crate::schema::Message;
use crate::tools::ToolCall;
use serde::{Deserialize, Serialize};

pub enum AgentAction {
    Tool {
        tool: String,
        tool_input: serde_json::Value,
        log: String,
    },
    Message(Message),
}

pub struct AgentFinish {
    pub return_values: serde_json::Value,
    pub log: String,
}

impl AgentFinish {
    pub fn new(output: impl Into<String>, log: impl Into<String>) -> Self {
        Self {
            return_values: serde_json::json!({"output": output.into()}),
            log: log.into(),
        }
    }

    pub fn output(&self) -> Option<&str> {
        self.return_values.get("output").and_then(|v| v.as_str())
    }
}

pub struct AgentStep {
    pub action: AgentAction,
    pub observation: String,
}

pub enum AgentOutput {
    Action(AgentAction),
    Finish(AgentFinish),
}

pub struct ToolInput {
    pub tool: String,
    pub input: serde_json::Value,
}
