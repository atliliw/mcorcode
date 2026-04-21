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

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn list(&self) -> Vec<&Box<dyn Tool>> {
        self.tools.values().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio::runtime::Runtime;

    #[test]
    fn test_tool_registry_new() {
        let registry = ToolRegistry::new();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_tool_registry_register() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(ReadTool::new(".")));
        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("read_file").is_some());
    }

    #[test]
    fn test_tool_registry_multiple() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(ReadTool::new(".")));
        registry.register(Box::new(WriteTool::new(".")));
        registry.register(Box::new(EditTool::new(".")));
        assert_eq!(registry.list().len(), 3);
    }

    #[test]
    fn test_read_tool_name() {
        let tool = ReadTool::new(".");
        assert_eq!(tool.name(), "read_file");
        assert_eq!(tool.description(), "Read file contents from the filesystem");
    }

    #[test]
    fn test_read_tool_parameters() {
        let tool = ReadTool::new(".");
        let params = tool.parameters();
        assert!(params["properties"]["path"].is_object());
        assert_eq!(params["required"][0], "path");
    }

    #[test]
    fn test_edit_tool_name() {
        let tool = EditTool::new(".");
        assert_eq!(tool.name(), "edit_file");
    }

    #[test]
    fn test_glob_tool_name() {
        let tool = GlobTool::new(".");
        assert_eq!(tool.name(), "glob");
    }

    #[test]
    fn test_glob_pattern_matching_suffix() {
        let tool = GlobTool::new(".");
        assert!(tool.matches_pattern("test.rs", "*.rs"));
        assert!(tool.matches_pattern("lib.rs", "*.rs"));
        assert!(!tool.matches_pattern("test.txt", "*.rs"));
    }

    #[test]
    fn test_glob_pattern_matching_prefix() {
        let tool = GlobTool::new(".");
        assert!(tool.matches_pattern("mod.rs", "mod*"));
        assert!(tool.matches_pattern("mod_test.rs", "mod*"));
        assert!(!tool.matches_pattern("test.rs", "mod*"));
    }

    #[tokio::test]
    async fn test_read_tool_missing_path() {
        let tool = ReadTool::new(".");
        let result = tool.execute(serde_json::json!({})).await;
        assert!(result.is_err());
    }
}