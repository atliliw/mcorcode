use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcorcode")]
#[command(about = "A Claude Code-like CLI tool", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, help = "Working directory")]
    pub workdir: Option<String>,

    #[arg(short, long, help = "Model to use")]
    pub model: Option<String>,

    #[arg(long, help = "Enable debug logging")]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start interactive chat session")]
    Chat {
        #[arg(short, long, help = "Initial prompt")]
        prompt: Option<String>,
    },

    #[command(about = "Execute a single task")]
    Exec {
        #[arg(short, long, help = "Task description")]
        task: String,
    },

    #[command(about = "List available tools")]
    Tools,

    #[command(about = "Show configuration")]
    Config,

    #[command(about = "Initialize mcorcode in current directory")]
    Init {
        #[arg(long, help = "Force overwrite existing config")]
        force: bool,
    },
}
