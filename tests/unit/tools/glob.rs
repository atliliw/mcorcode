//! Unit tests for GlobTool

use mcorcode::tools::{GlobTool, Tool};

#[test]
fn test_glob_tool_properties() {
    let tool = GlobTool::new(".");
    assert_eq!(tool.name(), "glob");
    assert_eq!(tool.description(), "Find files matching a glob pattern");
}

#[test]
fn test_glob_tool_parameters() {
    let tool = GlobTool::new(".");
    let params = tool.parameters();
    assert!(params["properties"]["pattern"].is_object());
    assert_eq!(params["required"][0], "pattern");
}

#[test]
fn test_glob_pattern_suffix_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("main.rs", "*.rs"));
    assert!(tool.matches_pattern("lib.rs", "*.rs"));
    assert!(tool.matches_pattern("mod.rs", "*.rs"));
    assert!(!tool.matches_pattern("main.txt", "*.rs"));
    assert!(!tool.matches_pattern("Cargo.toml", "*.rs"));
}

#[test]
fn test_glob_pattern_prefix_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("mod.rs", "mod*"));
    assert!(tool.matches_pattern("mod_test.rs", "mod*"));
    assert!(tool.matches_pattern("module.rs", "mod*"));
    assert!(!tool.matches_pattern("test.rs", "mod*"));
    assert!(!tool.matches_pattern("main.rs", "mod*"));
}

#[test]
fn test_glob_pattern_exact_matching() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("Cargo.toml", "Cargo.toml"));
    assert!(!tool.matches_pattern("Cargo.lock", "Cargo.toml"));
}

#[test]
fn test_glob_pattern_with_wildcard_prefix() {
    let tool = GlobTool::new(".");
    assert!(tool.matches_pattern("test.rs", "**/*.rs"));
    assert!(tool.matches_pattern("src/main.rs", "**/*.rs"));
}

#[test]
fn test_glob_pattern_empty_cases() {
    let tool = GlobTool::new(".");
    assert!(!tool.matches_pattern("", "*.rs"));
    assert!(!tool.matches_pattern("test", "*.rs"));
}
