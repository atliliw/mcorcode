pub mod base;
pub mod definition;
pub mod read;
pub mod write;
pub mod edit;
pub mod bash;
pub mod grep;
pub mod glob;

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub use base::Tool;
pub use definition::{ToolDefinition, FunctionSpec};
pub use read::ReadTool;
pub use write::WriteTool;
pub use edit::EditTool;
pub use bash::BashTool;
pub use grep::GrepTool;
pub use glob::GlobTool;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.remove(name)
    }

    pub fn list(&self) -> Vec<&Arc<dyn Tool>> {
        self.tools.values().collect()
    }

    pub fn get_all_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values()
            .map(|t| ToolDefinition::new(t.name(), t.description())
                .with_parameters(t.parameters()))
            .collect()
    }

    pub fn get_openai_tools(&self) -> Vec<Value> {
        self.get_all_definitions()
            .iter()
            .map(|d| d.to_openai_format())
            .collect()
    }

    pub fn get_anthropic_tools(&self) -> Vec<Value> {
        self.get_all_definitions()
            .iter()
            .map(|d| d.to_anthropic_format())
            .collect()
    }

    pub async fn execute(&self, name: &str, args: Value) -> Result<String> {
        let tool = self.get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", name))?;
        tool.execute(args).await
    }

    pub fn list_names(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}