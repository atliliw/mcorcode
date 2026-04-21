//! 流式聊天案例

use mcorcode::common::AppConfig;
use mcorcode::llm::{OpenAIClient, OpenAIClientConfig};
use mcorcode::llm::types::BaseChatModel;
use mcorcode::schema::Message;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("=== 流式聊天案例 ===\n");
    
    let config = AppConfig::test_config();
    
    let client_config = OpenAIClientConfig::new(config.openai.api_key)
        .with_base_url(config.openai.base_url)
        .with_model(config.openai.default_model);
    
    let client = OpenAIClient::new(client_config);
    
    let messages = vec![
        Message::system("你是一个创意写作助手。"),
        Message::human("请写一首关于编程的短诗，不超过4行。"),
    ];
    
    println!("用户: 请写一首关于编程的短诗，不超过4行。\n");
    println!("助手: ");
    
    // 流式输出
    match client.stream_chat(messages).await {
        Ok(mut stream) => {
            while let Some(token) = stream.next().await {
                print!("{}", token);
                // 立即刷新输出
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
            println!("\n");
        }
        Err(e) => {
            println!("错误: {:?}", e);
        }
    }
}