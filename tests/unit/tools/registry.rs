//! ToolRegistry 和工具实现单元测试

use mcorcode::tools::{EditTool, ReadTool, Tool, ToolRegistry, WriteTool};

/// 测试 ToolRegistry 的创建
/// 新建的注册器应为空，无已注册工具
#[test]
fn test_tool_registry_new() {
    let registry = ToolRegistry::new();
    assert!(registry.list().is_empty());
}

/// 测试 ToolRegistry 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_tool_registry_default() {
    let registry = ToolRegistry::default();
    assert!(registry.list().is_empty());
}

/// 测试注册单个工具
/// 注册后应可通过名称获取该工具
#[test]
fn test_tool_registry_register_single() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    assert_eq!(registry.list().len(), 1);
    assert!(registry.get("read_file").is_some());
}

/// 测试注册多个工具
/// 所有注册的工具都应在注册器中可用
#[test]
fn test_tool_registry_register_multiple() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    registry.register(Box::new(WriteTool::new(".")));
    registry.register(Box::new(EditTool::new(".")));
    assert_eq!(registry.list().len(), 3);
}

/// 测试通过名称获取已注册工具
/// get() 应返回名称匹配的工具
#[test]
fn test_tool_registry_get_existing() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ReadTool::new(".")));
    let tool = registry.get("read_file");
    assert!(tool.is_some());
    assert_eq!(tool.unwrap().name(), "read_file");
}

/// 测试获取未注册的工具
/// get() 对未知工具名应返回 None
#[test]
fn test_tool_registry_get_nonexistent() {
    let registry = ToolRegistry::new();
    assert!(registry.get("nonexistent").is_none());
}

/// 测试 ReadTool 的属性
/// 验证 name 和 description 正确设置
#[test]
fn test_read_tool_properties() {
    let tool = ReadTool::new(".");
    assert_eq!(tool.name(), "read_file");
    assert_eq!(tool.description(), "Read file contents from the filesystem");
}

/// 测试 ReadTool 的 JSON 参数 schema
/// 参数应包含 path 作为必填字段
#[test]
fn test_read_tool_parameters() {
    let tool = ReadTool::new(".");
    let params = tool.parameters();
    assert!(params["properties"]["path"].is_object());
    assert_eq!(params["required"][0], "path");
}

/// 测试 EditTool 的属性
/// 验证 name 和 description 正确设置
#[test]
fn test_edit_tool_properties() {
    let tool = EditTool::new(".");
    assert_eq!(tool.name(), "edit_file");
    assert_eq!(
        tool.description(),
        "Perform exact string replacements in a file"
    );
}

/// 测试 WriteTool 的属性
/// 验证 name 正确设置
#[test]
fn test_write_tool_properties() {
    let tool = WriteTool::new(".");
    assert_eq!(tool.name(), "write_file");
}
