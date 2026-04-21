//! 带工具的 Agent 案例

use mcorcode::common::AppConfig;
use mcorcode::llm::{OpenAIClient, OpenAIClientConfig};
use mcorcode::llm::types::{BaseChatModel, ToolDefinition};
use mcorcode::schema::Message;

#[tokio::main]
async fn main() {
    println!("=== 工具调用案例 ===\n");
    
    let config = AppConfig::test_config();
    
    // 定义一个天气查询工具
    let weather_tool = ToolDefinition::new(
        "get_weather",
        "获取指定城市的当前天气信息"
    )
    .with_parameters(serde_json::json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "城市名称，如 '北京'、'上海'"
            }
        },
        "required": ["city"]
    }));
    
    // 定义一个计算器工具
    let calculator_tool = ToolDefinition::new(
        "calculate",
        "执行数学计算"
    )
    .with_parameters(serde_json::json!({
        "type": "object",
        "properties": {
            "expression": {
                "type": "string",
                "description": "数学表达式，如 '2 + 3 * 4'"
            }
        },
        "required": ["expression"]
    }));
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key)
        .with_base_url(config.openai.base_url)
        .with_model(config.openai.default_model)
        .with_max_tokens(300);
    
    // 创建带工具的客户端
    let client = OpenAIClient::new(client_config)
        .with_tools(vec![weather_tool, calculator_tool]);
    
    // 让 LLM 决定使用什么工具
    let messages = vec![
        Message::human("北京今天的天气怎么样？"),
    ];
    
    println!("用户: 北京今天的天气怎么样？\n");
    
    match client.chat(messages).await {
        Ok(output) => {
            println!("助手回复: {}\n", output.content);
            
            // 检查是否有工具调用
            if output.has_tool_calls() {
                println!("LLM 决定调用以下工具:");
                for tool_call in output.get_tool_calls() {
                    println!("  - 工具: {}", tool_call.name);
                    println!("    参数: {}", tool_call.arguments);
                    
                    // 这里应该执行实际工具并返回结果
                    // 模拟工具执行
                    if tool_call.name == "get_weather" {
                        println!("    模拟结果: 北京今天晴，温度 25°C");
                    }
                }
            }
        }
        Err(e) => {
            println!("错误: {:?}", e);
        }
    }
    
    // 另一个例子：数学计算
    println!("\n--- 第二个例子 ---\n");
    
    let messages2 = vec![
        Message::human("帮我计算 (15 + 7) * 3 的结果"),
    ];
    
    println!("用户: 帮我计算 (15 + 7) * 3 的结果\n");
    
    match client.chat(messages2).await {
        Ok(output) => {
            println!("助手回复: {}\n", output.content);
            
            if output.has_tool_calls() {
                for tool_call in output.get_tool_calls() {
                    println!("  - 工具: {}", tool_call.name);
                    println!("    参数: {}", tool_call.arguments);
                    
                    if tool_call.name == "calculate" {
                        println!("    结果: {}", 66); // (15+7)*3 = 66
                    }
                }
            }
        }
        Err(e) => {
            println!("错误: {:?}", e);
        }
    }
}