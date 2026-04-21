//! Unit tests for ToolRegistry

use mcorcode::tools::{EditTool, ReadTool, Tool, ToolRegistry, WriteTool};

#[test]
fn test_tool_registry_new() {
    let registry = ToolRegistry::new();
    assert!(registry.list().is_empty());
}

#[test]
fn test_tool_registry_default() {
    let registry = ToolRegistry::default();
    assert!(registry.list().is_empty());
}

#[test]
fn test_tool_registry_register_single() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    assert_eq!(registry.list().len(), 1);
    assert!(registry.get("read_file").is_some());
}

#[test]
fn test_tool_registry_register_multiple() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    registry.register(Box::new(WriteTool::new(".")));
    registry.register(Box::new(EditTool::new(".")));
    assert_eq!(registry.list().len(), 3);
}

#[test]
fn test_tool_registry_get_existing() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    let tool = registry.get("read_file");
    assert!(tool.is_some());
    assert_eq!(tool.unwrap().name(), "read_file");
}

#[test]
fn test_tool_registry_get_nonexistent() {
    let registry = ToolRegistry::new();
    assert!(registry.get("nonexistent").is_none());
}

#[test]
fn test_read_tool_properties() {
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
fn test_edit_tool_properties() {
    let tool = EditTool::new(".");
    assert_eq!(tool.name(), "edit_file");
    assert_eq!(
        tool.description(),
        "Perform exact string replacements in a file"
    );
}

#[test]
fn test_write_tool_properties() {
    let tool = WriteTool::new(".");
    assert_eq!(tool.name(), "write_file");
}
