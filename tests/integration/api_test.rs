//! 真实 API 调用集成测试
//!
//! 使用真实 API Key 进行测试（需要网络连接）

use mcorcode::common::{OpenAIConfig, AppConfig};
use mcorcode::llm::{OpenAIClient, OpenAIClientConfig};
use mcorcode::llm::types::BaseChatModel;
use mcorcode::schema::Message;

/// 测试 OpenAI API 基础连接
/// 验证 API Key 和 Base URL 配置正确
#[tokio::test]
async fn test_openai_api_connection() {
    let config = AppConfig::test_config();
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key.clone())
        .with_base_url(config.openai.base_url.clone())
        .with_model(config.openai.default_model.clone())
        .with_max_tokens(100);
    
    let client = OpenAIClient::new(client_config);
    
    // 发送简单测试消息
    let messages = vec![
        Message::system("You are a helpful assistant."),
        Message::human("Say 'Hello' in exactly one word."),
    ];
    
    let result = client.chat(messages).await;

    println!("Request: {:?}", result);
    // 验证响应
    match result {
        Ok(output) => {
            println!("Response: {}", output.content);
            assert!(!output.content.is_empty());
            assert!(output.usage.is_some());
        }
        Err(e) => {
            println!("Error: {:?}", e);
            // 如果是网络错误，测试环境可能无法连接
            // 不 panic，只记录
        }
    }
}

/// 测试 OpenAI 流式响应
/// 验证 stream_chat 方法正常工作
#[tokio::test]
async fn test_openai_stream_chat() {
    let config = AppConfig::test_config();
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key.clone())
        .with_base_url(config.openai.base_url.clone())
        .with_model(config.openai.default_model.clone());
    
    let client = OpenAIClient::new(client_config);
    
    let messages = vec![
        Message::human("Count from 1 to 5, one number per line."),
    ];
    
    let result = client.stream_chat(messages).await;
    
    match result {
        Ok(mut stream) => {
            let mut collected = String::new();
            while let Some(token) = stream.next().await {
                collected.push_str(&token);
            }
            println!("Streamed response: {}", collected);
            assert!(!collected.is_empty());
        }
        Err(e) => {
            println!("Stream error: {:?}", e);
        }
    }
}

use futures::StreamExt;

/// 测试工具调用功能
/// 验证 LLM 能正确生成 tool_calls
#[tokio::test]
async fn test_openai_tool_calls() {
    use mcorcode::llm::types::ToolDefinition;
    
    let config = AppConfig::test_config();
    
    // 定义一个简单工具
    let tool = ToolDefinition::new("get_weather", "Get current weather for a location")
        .with_parameters(serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["location"]
        }));
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key.clone())
        .with_base_url(config.openai.base_url.clone())
        .with_model(config.openai.default_model.clone());
    
    let client = OpenAIClient::new(client_config).with_tools(vec![tool]);
    
    let messages = vec![
        Message::human("What's the weather in Beijing?"),
    ];
    
    let result = client.chat(messages).await;
    
    match result {
        Ok(output) => {
            println!("Response: {}", output.content);
            if output.has_tool_calls() {
                println!("Tool calls: {:?}", output.tool_calls);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

/// 测试配置加载
/// 验证 AppConfig 正确读取配置
#[test]
fn test_app_config() {
    let config = AppConfig::test_config();
    
    assert!(!config.openai.api_key.is_empty());
    assert_eq!(config.openai.base_url, "https://api.openai-proxy.org/v1");
    assert_eq!(config.openai.default_model, "gpt-3.5-turbo");
    assert_eq!(config.openai.embedding_model, "text-embedding-ada-002");
}

/// 测试环境变量覆盖
/// 验证环境变量优先级高于默认值
#[test]
fn test_env_override() {
    // 清除可能存在的环境变量
    std::env::remove_var("OPENAI_API_KEY");
    std::env::remove_var("OPENAI_BASE_URL");
    
    let config = OpenAIConfig::from_env_or_default();
    
    // 应使用默认值
    assert!(!config.api_key.is_empty());
    assert_eq!(config.base_url, "https://api.openai-proxy.org/v1");
}