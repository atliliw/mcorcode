//! ToolDefinition 类型系统单元测试
//! 测试工具定义的创建和格式转换

use mcorcode::tools::ToolDefinition;

/// 测试 ToolDefinition 的基本创建
/// new() 应正确设置 name、description 和默认参数
#[test]
fn test_tool_definition_new() {
    let def = ToolDefinition::new("read_file", "Read file contents");
    assert_eq!(def.name, "read_file");
    assert_eq!(def.description, "Read file contents");
    assert!(def.parameters.is_object());
}

/// 测试 ToolDefinition 的自定义参数
/// with_parameters 应替换默认参数 schema
#[test]
fn test_tool_definition_with_parameters() {
    let params = serde_json::json!({
        "type": "object",
        "properties": {
            "path": {"type": "string"}
        },
        "required": ["path"]
    });

    let def = ToolDefinition::new("read", "Read").with_parameters(params.clone());

    assert_eq!(def.parameters, params);
}

/// 测试 ToolDefinition 的 OpenAI 格式转换
/// to_openai_format 应生成正确的 OpenAI tools 格式
#[test]
fn test_tool_definition_to_openai_format() {
    let def = ToolDefinition::new("bash", "Run bash command")
        .with_parameters(serde_json::json!({"type": "object"}));

    let openai = def.to_openai_format();
    assert_eq!(openai["type"], "function");
    assert_eq!(openai["function"]["name"], "bash");
    assert_eq!(openai["function"]["description"], "Run bash command");
    assert!(openai["function"]["parameters"].is_object());
}

/// 测试 ToolDefinition 的 Anthropic 格式转换
/// to_anthropic_format 应生成正确的 Anthropic tools 格式
#[test]
fn test_tool_definition_to_anthropic_format() {
    let def = ToolDefinition::new("edit", "Edit file")
        .with_parameters(serde_json::json!({"type": "object"}));

    let anthropic = def.to_anthropic_format();
    assert_eq!(anthropic["name"], "edit");
    assert_eq!(anthropic["description"], "Edit file");
    assert!(anthropic["input_schema"].is_object());
}

/// 测试 ToolDefinition 的复杂参数 schema
/// 复嵌套参数 schema 应正确保留
#[test]
fn test_tool_definition_complex_parameters() {
    let params = serde_json::json!({
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "File path"},
            "content": {"type": "string", "description": "File content"},
            "options": {
                "type": "object",
                "properties": {
                    "encoding": {"type": "string"}
                }
            }
        },
        "required": ["path", "content"]
    });

    let def = ToolDefinition::new("write", "Write file").with_parameters(params.clone());

    assert_eq!(
        def.parameters["properties"]["options"]["properties"]["encoding"]["type"],
        "string"
    );
}

/// 测试多个 ToolDefinition 的 OpenAI 格式列表生成
/// 多个定义应生成多个 OpenAI 格式对象
#[test]
fn test_multiple_definitions_to_openai() {
    let defs = vec![
        ToolDefinition::new("read", "Read"),
        ToolDefinition::new("write", "Write"),
    ];

    let openai_tools: Vec<_> = defs
        .iter()
        .map(|d: &ToolDefinition| d.to_openai_format())
        .collect();

    assert_eq!(openai_tools.len(), 2);
    assert_eq!(openai_tools[0]["function"]["name"], "read");
    assert_eq!(openai_tools[1]["function"]["name"], "write");
}

/// 测试 ToolDefinition 的空描述
/// 空描述应被接受并正确处理
#[test]
fn test_tool_definition_empty_description() {
    let def = ToolDefinition::new("tool", "");
    assert_eq!(def.description, "");
}

/// 测试 ToolDefinition 的特殊字符名称
/// 特殊字符名称应被正确处理
#[test]
fn test_tool_definition_special_characters() {
    let def = ToolDefinition::new("my_tool_v2", "My tool");
    assert_eq!(def.name, "my_tool_v2");
}
