pub mod base;
pub mod read;
pub mod write;
pub mod edit;
pub mod bash;
pub mod grep;
pub mod glob;

use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;

pub use base::Tool;
pub use read::ReadTool;
pub use write::WriteTool;
pub use edit::EditTool;
pub use bash::BashTool;
pub use grep::GrepTool;
pub use glob::GlobTool;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list(&self) -> Vec<&dyn Tool> {
        self.tools.values().map(|t| t.as_ref()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}