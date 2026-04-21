//! FunctionCallingAgent 单元测试
//! 测试 Function Calling Agent 的创建和配置

use mcorcode::agent::{FunctionCallingAgent, BaseAgent};
use std::sync::Arc;
use mcorcode::tools::ReadTool;

/// 测试 FunctionCallingAgent 的创建
/// 新建 agent 应有正确的名称和默认系统提示词
#[test]
fn test_function_calling_agent_new() {
    let agent = FunctionCallingAgent::new("test_agent", vec![]);
    assert_eq!(agent.name(), "test_agent");
    assert!(!agent.system_prompt().is_empty());
}

/// 测试 FunctionCallingAgent 的自定义系统提示词
/// with_system_prompt 应替换默认提示词
#[test]
fn test_function_calling_agent_custom_prompt() {
    let agent = FunctionCallingAgent::new("test", vec![])
        .with_system_prompt("Custom prompt");
    assert_eq!(agent.system_prompt(), "Custom prompt");
}

/// 测试 FunctionCallingAgent 的工具列表
/// get_tools 应返回所有已添加的工具
#[test]
fn test_function_calling_agent_tools() {
    let tool = Arc::new(ReadTool::new("."));
    let agent = FunctionCallingAgent::new("test", vec![tool.clone()]);
    
    let tools = agent.get_tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name(), "read_file");
}

/// 测试 FunctionCallingAgent 的 plan 方法（无中间步骤）
/// 第一次迭代应返回 Finish
#[tokio::test]
async fn test_function_calling_agent_plan_first_iteration() {
    let agent = FunctionCallingAgent::new("test", vec![]);
    let inputs = std::collections::HashMap::from([
        ("input".to_string(), "Hello".to_string()),
    ]);
    
    let result: Result<mcorcode::agent::AgentOutput, mcorcode::agent::AgentError> = agent.plan(&[], &inputs).await;
    assert!(result.is_ok());
    
    match result.unwrap() {
        mcorcode::agent::AgentOutput::Finish(finish) => {
            assert_eq!(finish.output(), Some("Hello"));
        }
        mcorcode::agent::AgentOutput::Action(_) => panic!("Expected Finish"),
    }
}

/// 测试 FunctionCallingAgent 空工具列表
/// 无工具时 get_tools 应返回空数组
#[test]
fn test_function_calling_agent_empty_tools() {
    let agent = FunctionCallingAgent::new("empty", vec![]);
    assert!(agent.get_tools().is_empty());
}

/// 测试 FunctionCallingAgent 的默认系统提示词内容
/// 默认提示词应包含工具使用说明
#[test]
fn test_function_calling_agent_default_prompt_content() {
    let agent = FunctionCallingAgent::new("test", vec![]);
    let prompt = agent.system_prompt();
    assert!(prompt.contains("tools"));
    assert!(prompt.contains("AI assistant"));
}