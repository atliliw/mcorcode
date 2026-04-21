//! AgentExecutor 单元测试
//! 测试 Agent 执行器的配置和错误处理

use mcorcode::agent::{AgentAction, AgentError, AgentFinish, AgentStep};

/// 测试 AgentError 的 LlmError 显示格式
/// LlmError 应正确格式化为 "LLM error: {msg}"
#[test]
fn test_agent_error_llm_error_display() {
    let error = AgentError::LlmError("API timeout".to_string());
    assert_eq!(error.to_string(), "LLM error: API timeout");
}

/// 测试 AgentError 的 ToolExecutionError 显示格式
/// ToolExecutionError 应包含工具名和错误消息
#[test]
fn test_agent_error_tool_execution_display() {
    let error = AgentError::ToolExecutionError {
        tool: "bash".to_string(),
        error: "command failed".to_string(),
    };
    assert_eq!(error.to_string(), "Tool 'bash' error: command failed");
}

/// 测试 AgentError 的 MaxIterationsReached 显示格式
/// MaxIterationsReached 应显示固定消息
#[test]
fn test_agent_error_max_iterations_display() {
    let error = AgentError::MaxIterationsReached;
    assert_eq!(error.to_string(), "Max iterations reached");
}

/// 测试 AgentError 的 TimeoutReached 显示格式
/// TimeoutReached 应显示 "Timeout"
#[test]
fn test_agent_error_timeout_display() {
    let error = AgentError::TimeoutReached;
    assert_eq!(error.to_string(), "Timeout");
}

/// 测试 AgentError 的 OutputParsingError 显示格式
/// OutputParsingError 应显示解析错误消息
#[test]
fn test_agent_error_output_parsing_display() {
    let error = AgentError::OutputParsingError("Invalid JSON".to_string());
    assert_eq!(error.to_string(), "Output parsing error: Invalid JSON");
}

/// 测试 AgentError 的 InvalidToolCall 显示格式
/// InvalidToolCall 应显示无效工具调用消息
#[test]
fn test_agent_error_invalid_tool_call_display() {
    let error = AgentError::InvalidToolCall("Tool not found".to_string());
    assert_eq!(error.to_string(), "Invalid tool call: Tool not found");
}

/// 测试 AgentFinish 的创建
/// new() 应正确设置 return_values 和 log
#[test]
fn test_agent_finish_new() {
    let finish = AgentFinish::new("Task completed", "All done");
    assert_eq!(finish.output(), Some("Task completed"));
    assert_eq!(finish.log, "All done");
}

/// 测试 AgentFinish 的 output 方法
/// output() 应从 return_values 中提取 output 字段
#[test]
fn test_agent_finish_output_extraction() {
    let finish = AgentFinish::new("Result", "Log message");
    assert_eq!(finish.output(), Some("Result"));
}

/// 测试 AgentFinish 的 output 方法（无 output 字段）
/// return_values 无 output 时应返回 None
#[test]
fn test_agent_finish_output_none() {
    let finish = AgentFinish {
        return_values: serde_json::json!({"other": "value"}),
        log: "test".to_string(),
    };
    assert!(finish.output().is_none());
}

/// 测试 AgentAction::Tool 的创建
/// Tool action 应包含工具名、输入和日志
#[test]
fn test_agent_action_tool_creation() {
    let action = AgentAction::Tool {
        tool: "read".to_string(),
        tool_input: serde_json::json!({"path": "/test"}),
        log: "Reading file".to_string(),
    };

    match action {
        AgentAction::Tool {
            tool,
            tool_input,
            log,
        } => {
            assert_eq!(tool, "read");
            assert_eq!(tool_input["path"], "/test");
            assert_eq!(log, "Reading file");
        }
        AgentAction::Message(_) => panic!("Expected Tool action"),
    }
}

/// 测试 AgentStep 的创建
/// AgentStep 应包含 action 和 observation
#[test]
fn test_agent_step_creation() {
    let action = AgentAction::Tool {
        tool: "bash".to_string(),
        tool_input: serde_json::json!({"cmd": "ls"}),
        log: "".to_string(),
    };
    let step = AgentStep {
        action,
        observation: "file1.txt\nfile2.txt".to_string(),
    };

    assert_eq!(step.observation, "file1.txt\nfile2.txt");
}

/// 测试 AgentError 实现 Error trait
/// AgentError 应可作为 std::error::Error 使用
#[test]
fn test_agent_error_is_std_error() {
    let error = AgentError::MaxIterationsReached;
    let _: &dyn std::error::Error = &error;
}
