//! 最简单的聊天案例

use mcorcode::common::AppConfig;
use mcorcode::llm::{OpenAIClient, OpenAIClientConfig};
use mcorcode::llm::types::BaseChatModel;
use mcorcode::schema::Message;

#[tokio::main]
async fn main() {
    println!("=== 简单聊天案例 ===\n");
    
    // 加载配置（使用真实 API Key）
    let config = AppConfig::test_config();
    
    // 创建 OpenAI 客户端
    let client_config = OpenAIClientConfig::new(config.openai.api_key)
        .with_base_url(config.openai.base_url)
        .with_model(config.openai.default_model)
        .with_max_tokens(500);
    
    let client = OpenAIClient::new(client_config);
    
    // 构建消息
    let messages = vec![
        Message::system("你是一个有帮助的助手，用中文回答问题。"),
        Message::human("你好，请介绍一下 Rust 语言的特点。"),
    ];
    
    println!("用户: 你好，请介绍一下 Rust 语言的特点。\n");
    
    // 调用 API
    match client.chat(messages).await {
        Ok(output) => {
            println!("助手: {}\n", output.content);
            
            if let Some(usage) = output.usage {
                println!("Token 使用: 输入 {} / 输出 {} / 总计 {}", 
                    usage.prompt_tokens, 
                    usage.completion_tokens,
                    usage.total_tokens
                );
            }
        }
        Err(e) => {
            println!("错误: {:?}", e);
        }
    }
}