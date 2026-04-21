//! 完整 Agent 流程案例

use mcorcode::common::AppConfig;
use mcorcode::llm::{OpenAIClient, OpenAIClientConfig};
use mcorcode::llm::types::{BaseChatModel, ToolDefinition};
use mcorcode::memory::{ConversationBufferMemory, BaseMemory};
use mcorcode::schema::{Message, ToolCall};

#[tokio::main]
async fn main() {
    println!("=== 完整 Agent 流程案例 ===\n");
    
    let config = AppConfig::test_config();
    
    // 定义工具
    let tools = vec![
        ToolDefinition::new("read_file", "读取文件内容")
            .with_parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "文件路径" }
                },
                "required": ["path"]
            })),
        ToolDefinition::new("write_file", "写入文件内容")
            .with_parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path", "content"]
            })),
        ToolDefinition::new("bash", "执行 shell 命令")
            .with_parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" }
                },
                "required": ["command"]
            })),
    ];
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key)
        .with_base_url(config.openai.base_url)
        .with_model(config.openai.default_model)
        .with_max_tokens(1000);
    
    let client = OpenAIClient::new(client_config)
        .with_tools(tools);
    
    // 创建记忆
    let mut memory = ConversationBufferMemory::new();
    memory.add_message(Message::system(
        "你是一个代码助手。你有以下工具可用：\n\
        - read_file: 读取文件\n\
        - write_file: 写入文件\n\
        - bash: 执行命令\n\
        根据用户请求选择合适的工具。"
    ));
    
    // 用户任务
    let task = "请读取 Cargo.toml 文件，然后告诉我项目名称是什么";
    
    memory.add_user_message(task);
    println!("任务: {}\n", task);
    
    // 第一轮：LLM 决定调用工具
    println!("--- 第 1 轮：LLM 分析 ---\n");
    
    let messages = memory.get_messages().to_vec();
    
    match client.chat(messages).await {
        Ok(output) => {
            println!("LLM: {}", output.content);
            
            if output.has_tool_calls() {
                println!("\n工具调用:");
                for tc in output.get_tool_calls() {
                    println!("  → {}({})", tc.name, tc.arguments);
                    
                    // 执行工具（模拟）
                    let result = execute_tool_mock(&tc);
                    println!("  ← 结果: {}\n", result.chars().take(100).collect::<String>());
                    
                    // 将结果加入记忆
                    memory.add_tool_result(&tc.id, &result);
                }
            }
        }
        Err(e) => println!("错误: {:?}", e),
    }
    
    // 第二轮：LLM 分析结果
    println!("--- 第 2 轮：分析结果 ---\n");
    
    let messages2 = memory.get_messages().to_vec();
    
    match client.chat(messages2).await {
        Ok(output) => {
            println!("LLM 最终回复: {}\n", output.content);
            
            if let Some(usage) = output.usage {
                println!("总 Token: {}", usage.total_tokens);
            }
        }
        Err(e) => println!("错误: {:?}", e),
    }
    
    // 显示完整对话历史
    println!("--- 对话历史 ---\n");
    for (i, msg) in memory.get_messages().iter().enumerate() {
        let content_preview: String = msg.content.chars().take(50).collect();
        println!("{}. [{}] {}", i + 1, msg.role(), content_preview);
    }
}

/// 模拟工具执行
fn execute_tool_mock(tool_call: &ToolCall) -> String {
    match tool_call.name.as_str() {
        "read_file" => {
            let path = tool_call.arguments.get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            
            if path.contains("Cargo.toml") {
                "[package]\nname = \"mcorcode\"\nversion = \"0.1.0\"\nedition = \"2021\"".to_string()
            } else {
                format!("文件 {} 不存在", path)
            }
        }
        "write_file" => "文件已写入成功".to_string(),
        "bash" => {
            let cmd = tool_call.arguments.get("command")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            format!("命令 '{}' 执行完成", cmd)
        }
        _ => "未知工具".to_string(),
    }
}