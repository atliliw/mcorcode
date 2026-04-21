//! Memory 使用案例

use mcorcode::memory::{BaseMemory, ConversationBufferMemory, ConversationBufferWindowMemory};
use mcorcode::schema::Message;

fn main() {
    println!("=== Memory 使用案例 ===\n");

    // 1. BufferMemory - 保存所有消息
    println!("--- ConversationBufferMemory ---\n");

    let mut buffer_memory = ConversationBufferMemory::new();

    // 添加对话历史
    buffer_memory.add_user_message("你好！");
    buffer_memory.add_ai_message("你好！有什么我可以帮助你的吗？");
    buffer_memory.add_user_message("请介绍一下 Rust。");
    buffer_memory.add_ai_message("Rust 是一门系统编程语言...");
    buffer_memory.add_tool_result("tool_123", "文件内容已读取");

    println!("对话历史:");
    for msg in buffer_memory.get_messages() {
        println!(
            "  [{}] {}",
            msg.role(),
            msg.content.chars().take(30).collect::<String>()
        );
    }

    println!("\nToken 估算: {}", buffer_memory.token_count());

    // JSON 序列化
    let json = buffer_memory.to_json().unwrap();
    println!("序列化长度: {} bytes\n", json.len());

    // 2. WindowMemory - 只保留最近 N 条
    println!("--- ConversationBufferWindowMemory ---\n");

    let mut window_memory = ConversationBufferWindowMemory::new(3);

    // 添加更多消息
    window_memory.add_user_message("消息 1");
    window_memory.add_ai_message("回复 1");
    window_memory.add_user_message("消息 2");
    window_memory.add_ai_message("回复 2");
    window_memory.add_user_message("消息 3");
    window_memory.add_ai_message("回复 3");
    window_memory.add_user_message("消息 4"); // 这条会触发裁剪
    window_memory.add_ai_message("回复 4");

    println!("窗口大小: {}", window_memory.window_size());
    println!("当前保留 {} 条消息:", window_memory.get_messages().len());

    for msg in window_memory.get_messages() {
        println!("  [{}] {}", msg.role(), msg.content);
    }

    // 获取最近 2 条
    println!("\n最近 2 条:");
    for msg in window_memory.get_last_n(2) {
        println!("  [{}] {}", msg.role(), msg.content);
    }

    // 3. Token 限制裁剪
    println!("\n--- Token 限制裁剪 ---\n");

    let mut memory = ConversationBufferMemory::new();

    // 添加长消息
    memory.add_message(Message::system("系统提示：你是助手"));
    memory.add_user_message("这是一条很长的用户消息，包含很多内容...");
    memory.add_ai_message("这是一条很长的回复，也包含很多内容...");
    memory.add_user_message("又一条长消息");

    println!(
        "裁剪前: {} 条消息, {} tokens",
        memory.get_messages().len(),
        memory.token_count()
    );

    // 裁剪到 50 tokens（模拟）
    memory.trim_to_token_limit(50);

    println!("裁剪后: {} 条消息", memory.get_messages().len());
    println!("系统消息保留:");
    for msg in memory.get_messages() {
        if msg.is_system() {
            println!("  ✓ {}", msg.content);
        }
    }
}
