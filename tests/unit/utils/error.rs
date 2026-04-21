//! McorCodeError 错误类型单元测试

use mcorcode::utils::McorCodeError;
use std::io;

/// 测试 IO 错误转换
/// McorCodeError::Io 应能从 std::io::Error 自动转换
#[test]
fn test_error_from_io() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error: McorCodeError = io_error.into();

    assert!(matches!(error, McorCodeError::Io(_)));
}

/// 测试错误消息显示
/// 各错误类型应有正确的 Display 消息
#[test]
fn test_error_display_io() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "test file");
    let error = McorCodeError::Io(io_error);

    let message = error.to_string();
    assert!(message.contains("IO error"));
}

/// 测试 LlmApi 错误消息
/// McorCodeError::LlmApi 应显示正确的错误消息
#[test]
fn test_error_display_llm_api() {
    let error = McorCodeError::LlmApi("API request failed".to_string());

    let message = error.to_string();
    assert!(message.contains("LLM API error"));
    assert!(message.contains("API request failed"));
}

/// 测试 ToolExecution 错误消息
/// McorCodeError::ToolExecution 应显示正确的错误消息
#[test]
fn test_error_display_tool_execution() {
    let error = McorCodeError::ToolExecution("bash command failed".to_string());

    let message = error.to_string();
    assert!(message.contains("Tool execution error"));
    assert!(message.contains("bash command failed"));
}

/// 测试 Config 错误消息
/// McorCodeError::Config 应显示正确的错误消息
#[test]
fn test_error_display_config() {
    let error = McorCodeError::Config("missing config file".to_string());

    let message = error.to_string();
    assert!(message.contains("Configuration error"));
    assert!(message.contains("missing config file"));
}

/// 测试 Session 错误消息
/// McorCodeError::Session 应显示正确的错误消息
#[test]
fn test_error_display_session() {
    let error = McorCodeError::Session("session expired".to_string());

    let message = error.to_string();
    assert!(message.contains("Session error"));
    assert!(message.contains("session expired"));
}

/// 测试 Debug trait 实现
/// McorCodeError 应实现 Debug trait
#[test]
fn test_error_debug() {
    let error = McorCodeError::LlmApi("test".to_string());

    // Debug 输出应包含类型信息
    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("LlmApi"));
}
