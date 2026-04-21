use crate::schema::Message;

#[derive(Debug)]
pub enum AgentError {
    LlmError(String),
    ToolExecutionError { tool: String, error: String },
    MaxIterationsReached,
    TimeoutReached,
    OutputParsingError(String),
    InvalidToolCall(String),
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::LlmError(msg) => write!(f, "LLM error: {}", msg),
            AgentError::ToolExecutionError { tool, error } => {
                write!(f, "Tool '{}' error: {}", tool, error)
            }
            AgentError::MaxIterationsReached => write!(f, "Max iterations reached"),
            AgentError::TimeoutReached => write!(f, "Timeout"),
            AgentError::OutputParsingError(msg) => write!(f, "Output parsing error: {}", msg),
            AgentError::InvalidToolCall(msg) => write!(f, "Invalid tool call: {}", msg),
        }
    }
}

impl std::error::Error for AgentError {}

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
