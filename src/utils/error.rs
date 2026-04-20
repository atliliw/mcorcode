use thiserror::Error;

#[derive(Error, Debug)]
pub enum McorCodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("LLM API error: {0}")]
    LlmApi(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Session error: {0}")]
    Session(String),
}
