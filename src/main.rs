use anyhow::Result;
use mcorcode::agent::Agent;
use mcorcode::cli::{args::Commands, parse_args};
use mcorcode::llm::LlmClient;
use mcorcode::tools::{BashTool, EditTool, GlobTool, GrepTool, ReadTool, ToolRegistry, WriteTool};
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> Result<()> {
    let args = parse_args();

    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let workdir = args
        .workdir
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let api_key = std::env::var("MCORCODE_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .expect("API key required (set MCORCODE_API_KEY, ANTHROPIC_API_KEY, or OPENAI_API_KEY)");

    let api_base = std::env::var("MCORCODE_API_BASE")
        .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string());

    let model = args
        .model
        .unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string());

    let llm_client = LlmClient::new(&api_base, &api_key, &model);

    let mut tool_registry = ToolRegistry::new();
    let workdir_str = workdir.to_string_lossy().to_string();

    tool_registry.register(Arc::new(ReadTool::new(&workdir_str)));
    tool_registry.register(Arc::new(WriteTool::new(&workdir_str)));
    tool_registry.register(Arc::new(EditTool::new(&workdir_str)));
    tool_registry.register(Arc::new(BashTool::new(&workdir_str)));
    tool_registry.register(Arc::new(GrepTool::new(&workdir_str)));
    tool_registry.register(Arc::new(GlobTool::new(&workdir_str)));

    let rt = tokio::runtime::Runtime::new()?;

    match args.command {
        Some(Commands::Chat { prompt }) => {
            rt.block_on(run_chat(llm_client, tool_registry, prompt))?;
        }
        Some(Commands::Exec { task }) => {
            rt.block_on(run_exec(llm_client, tool_registry, task))?;
        }
        Some(Commands::Tools) => {
            list_tools(&tool_registry);
        }
        Some(Commands::Config) => {
            show_config(&workdir, &api_base, &model);
        }
        Some(Commands::Init { force }) => {
            init_config(&workdir, force)?;
        }
        None => {
            rt.block_on(run_chat(llm_client, tool_registry, None))?;
        }
    }

    Ok(())
}

async fn run_chat(
    mut llm_client: LlmClient,
    mut tool_registry: ToolRegistry,
    initial_prompt: Option<String>,
) -> Result<()> {
    let mut agent = Agent::new(llm_client, tool_registry);

    println!("mcorcode - Interactive Chat");
    println!("Type your message and press Enter. Type 'exit' to quit.");
    println!();

    if let Some(prompt) = initial_prompt {
        println!("User: {}", prompt);
        let response = agent.run(&prompt).await?;
        println!("Assistant: {}", response);
    }

    loop {
        println!();
        println!("User: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let response = agent.run(input).await?;
        println!("Assistant: {}", response);
    }

    Ok(())
}

async fn run_exec(
    mut llm_client: LlmClient,
    mut tool_registry: ToolRegistry,
    task: String,
) -> Result<()> {
    let mut agent = Agent::new(llm_client, tool_registry);

    let response = agent.run(&task).await?;

    println!("{}", response);

    Ok(())
}

fn list_tools(tool_registry: &ToolRegistry) {
    println!("Available Tools:");
    println!();

    for tool in tool_registry.list() {
        println!("  {} - {}", tool.name(), tool.description());
    }
}

fn show_config(workdir: &PathBuf, api_base: &str, model: &str) {
    println!("Current Configuration:");
    println!();
    println!("  Working Directory: {}", workdir.display());
    println!("  API Base: {}", api_base);
    println!("  Model: {}", model);
}

fn init_config(workdir: &PathBuf, force: bool) -> Result<()> {
    let config_file = workdir.join(".mcorcode.toml");

    if config_file.exists() && !force {
        println!("Config file already exists. Use --force to overwrite.");
        return Ok(());
    }

    let config_content = include_str!("../.mcorcode.toml.example");

    std::fs::write(&config_file, config_content)?;

    println!("Created config file at {}", config_file.display());

    Ok(())
}
