//! CLI REPL 交互式命令行实现

use crate::agent::{AgentExecutor, BaseAgent, FunctionCallingAgent};
use crate::session::SessionManager;
use anyhow::Result;
use std::io::{self, Write};
use std::sync::Arc;

pub struct ReplConfig {
    pub prompt_prefix: String,
    pub output_prefix: String,
    pub show_tool_calls: bool,
    pub show_thinking: bool,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt_prefix: "User: ".to_string(),
            output_prefix: "Assistant: ".to_string(),
            show_tool_calls: true,
            show_thinking: false,
        }
    }
}

pub struct Repl {
    agent_executor: AgentExecutor,
    session_manager: SessionManager,
    current_session: Option<String>,
    config: ReplConfig,
    agent: Arc<FunctionCallingAgent>,
}

impl Repl {
    pub fn new(agent: FunctionCallingAgent, llm: Box<dyn crate::llm::BaseChatModel>) -> Self {
        let agent_arc = Arc::new(agent);
        let executor = AgentExecutor::new(agent_arc.clone(), llm);
        Self {
            agent_executor: executor,
            session_manager: SessionManager::new(),
            current_session: None,
            config: ReplConfig::default(),
            agent: agent_arc,
        }
    }

    pub fn with_config(mut self, config: ReplConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_session(mut self, session_id: Option<String>) -> Result<Self> {
        match session_id {
            Some(id) => {
                if self.session_manager.session_file_exists(&id) {
                    self.session_manager.load_session(&id)?;
                    self.current_session = Some(id);
                } else {
                    let new_id = self.session_manager.create_session();
                    self.current_session = Some(new_id);
                }
            }
            None => {
                let id = self.session_manager.create_session();
                self.current_session = Some(id);
            }
        }
        Ok(self)
    }

    pub fn run(&mut self) -> Result<()> {
        println!("mcorcode REPL - Interactive AI Assistant");
        println!("Commands: /help, /tools, /session, /clear, /exit");
        println!();

        let rt = tokio::runtime::Runtime::new()?;

        loop {
            print!("{}", self.config.prompt_prefix);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input.starts_with('/') {
                if self.handle_command(input)? {
                    break;
                }
            } else {
                rt.block_on(self.handle_chat(input))?;
            }
        }

        Ok(())
    }

    fn handle_command(&mut self, cmd: &str) -> Result<bool> {
        match cmd {
            "/exit" | "/quit" | "/q" => {
                println!("Goodbye!");
                if let Some(id) = &self.current_session {
                    self.session_manager.save_session(id)?;
                }
                return Ok(true);
            }
            "/help" | "/h" => {
                self.show_help();
            }
            "/tools" | "/t" => {
                self.list_tools();
            }
            "/session" | "/s" => {
                self.show_session_info();
            }
            "/clear" | "/c" => {
                self.agent_executor.clear_memory();
                println!("Memory cleared.");
            }
            "/save" => {
                if let Some(id) = &self.current_session {
                    self.session_manager.save_session(id)?;
                    println!("Session saved: {}", id);
                }
            }
            "/load" => {
                let sessions = self.session_manager.list_saved_sessions()?;
                if sessions.is_empty() {
                    println!("No saved sessions found.");
                } else {
                    println!("Saved sessions:");
                    for id in sessions {
                        println!("  - {}", id);
                    }
                }
            }
            _ => {
                println!("Unknown command: {}", cmd);
                println!("Available commands: /help, /tools, /session, /clear, /save, /load, /exit");
            }
        }
        Ok(false)
    }

    async fn handle_chat(&mut self, input: &str) -> Result<()> {
        print!("{}", self.config.output_prefix);
        io::stdout().flush()?;

        match self.agent_executor.run(input).await {
            Ok(response) => {
                println!("{}", response);

                if let Some(id) = &self.current_session {
                    self.session_manager.add_message(
                        id,
                        crate::schema::Message::human(input),
                    )?;
                    self.session_manager.add_message(
                        id,
                        crate::schema::Message::ai(&response),
                    )?;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        println!();
        Ok(())
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  /help, /h     - Show this help message");
        println!("  /tools, /t    - List available tools");
        println!("  /session, /s  - Show session information");
        println!("  /clear, /c    - Clear conversation memory");
        println!("  /save         - Save current session");
        println!("  /load         - List saved sessions");
        println!("  /exit, /q     - Exit REPL");
    }

    fn list_tools(&self) {
        let tools = BaseAgent::get_tools(self.agent.as_ref());
        println!("Available tools:");
        for tool in tools {
            println!("  - {}: {}", tool.name(), tool.description());
        }
    }

    fn show_session_info(&self) {
        match &self.current_session {
            Some(id) => {
                println!("Current session: {}", id);
                if let Some(session) = self.session_manager.get_session(id) {
                    println!("  Messages: {}", session.message_count);
                    println!("  Created: {}", session.created_at);
                    println!("  Updated: {}", session.updated_at);
                }
            }
            None => {
                println!("No active session.");
            }
        }
    }
}