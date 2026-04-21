//! GlobTool 模式匹配单元测试

use mcorcode::tools::{GlobTool, Tool};

/// 测试 GlobTool 的基本属性
/// 验证 name 和 description 正确设置
#[test]
fn test_glob_tool_properties() {
    let tool = GlobTool::new(".");
    assert_eq!(tool.name(), "glob");
    assert_eq!(tool.description(), "Find files matching a glob pattern");
}

/// 测试 GlobTool 的 JSON 参数 schema
/// 参数应包含 pattern 作为必填字段
#[test]
fn test_glob_tool_parameters() {
    let tool = GlobTool::new(".");
    let params = tool.parameters();
    assert!(params["properties"]["pattern"].is_object());
    assert_eq!(params["required"][0], "pattern");
}

/// 测试后缀模式匹配 (*.rs)
/// 以 .rs 结尾的文件应匹配，其他文件不应匹配
#[test]
fn test_glob_pattern_suffix_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("main.rs", "*.rs"));
    assert!(tool.matches_pattern("lib.rs", "*.rs"));
    assert!(tool.matches_pattern("mod.rs", "*.rs"));
    assert!(!tool.matches_pattern("main.txt", "*.rs"));
    assert!(!tool.matches_pattern("Cargo.toml", "*.rs"));
}

/// 测试前缀模式匹配 (mod*)
/// 以 "mod" 开头的文件应匹配，其他文件不应匹配
#[test]
fn test_glob_pattern_prefix_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("mod.rs", "mod*"));
    assert!(tool.matches_pattern("mod_test.rs", "mod*"));
    assert!(tool.matches_pattern("module.rs", "mod*"));
    assert!(!tool.matches_pattern("test.rs", "mod*"));
    assert!(!tool.matches_pattern("main.rs", "mod*"));
}

/// 测试精确文件名匹配
/// 只有完全匹配的文件名才应成功
#[test]
fn test_glob_pattern_exact_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("Cargo.toml", "Cargo.toml"));
    assert!(!tool.matches_pattern("Cargo.lock", "Cargo.toml"));
}

/// 测试带通配符前缀的模式 (**/*.rs)
/// 模式前缀通配符应在匹配前被去除
#[test]
fn test_glob_pattern_with_wildcard_prefix() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("test.rs", "**/*.rs"));
    assert!(tool.matches_pattern("src/main.rs", "**/*.rs"));
}

/// 测试空字符串边界情况
/// 空字符串不应匹配任何模式
#[test]
fn test_glob_pattern_empty_cases() {
    let tool = GlobTool::new(".");
    assert!(!tool.matches_pattern("", "*.rs"));
    assert!(!tool.matches_pattern("test", "*.rs"));
}
