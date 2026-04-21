//! PermissionPolicy 权限策略单元测试

use mcorcode::permission::policy::ValuePattern;
use mcorcode::permission::{PermissionPolicy, PermissionResult};

/// 测试 allow_tool 创建策略
/// allow_tool 应创建一个允许特定工具的策略
#[test]
fn test_policy_allow_tool() {
    let policy = PermissionPolicy::allow_tool("bash");

    assert_eq!(policy.tool_pattern, "bash");
    assert!(policy.input_pattern.is_none());
    assert!(policy.result.is_allowed());
}

/// 测试 deny_tool 创建策略
/// deny_tool 应创建一个拒绝特定工具的策略，带原因
#[test]
fn test_policy_deny_tool() {
    let policy = PermissionPolicy::deny_tool("rm", "Dangerous tool");

    assert_eq!(policy.tool_pattern, "rm");
    assert!(policy.input_pattern.is_none());
    assert!(policy.result.is_denied());
    assert_eq!(policy.result.reason, Some("Dangerous tool".to_string()));
}

/// 测试策略的精确匹配
/// matches 对精确工具名应正确匹配
#[test]
fn test_policy_matches_exact() {
    let policy = PermissionPolicy::allow_tool("bash");

    // 匹配 bash
    assert!(policy.matches("bash", &serde_json::json!({})));

    // 不匹配其他工具
    assert!(!policy.matches("read", &serde_json::json!({})));
}

/// 测试策略的通配符匹配
/// "*" 应匹配所有工具名
#[test]
fn test_policy_matches_wildcard() {
    let policy = PermissionPolicy::allow_tool("*");

    assert!(policy.matches("bash", &serde_json::json!({})));
    assert!(policy.matches("read", &serde_json::json!({})));
    assert!(policy.matches("edit", &serde_json::json!({})));
}

/// 测试策略的前缀通配符匹配
/// "*_file" 应匹配以 "_file" 结尾的工具
#[test]
fn test_policy_matches_prefix_wildcard() {
    let policy = PermissionPolicy::allow_tool("*_file");

    assert!(policy.matches("read_file", &serde_json::json!({})));
    assert!(policy.matches("write_file", &serde_json::json!({})));
    assert!(policy.matches("edit_file", &serde_json::json!({})));
    assert!(!policy.matches("bash", &serde_json::json!({})));
}

/// 测试策略的后缀通配符匹配
/// "bash*" 应匹配以 "bash" 开头的工具
#[test]
fn test_policy_matches_suffix_wildcard() {
    let policy = PermissionPolicy::allow_tool("bash*");

    assert!(policy.matches("bash", &serde_json::json!({})));
    assert!(policy.matches("bash_tool", &serde_json::json!({})));
    assert!(!policy.matches("read_file", &serde_json::json!({})));
}

/// 测试策略的中间通配符匹配
/// "*tool*" 应匹配包含 "tool" 的工具名
#[test]
fn test_policy_matches_middle_wildcard() {
    let policy = PermissionPolicy::allow_tool("*tool*");

    assert!(policy.matches("my_tool", &serde_json::json!({})));
    assert!(policy.matches("tool_test", &serde_json::json!({})));
    assert!(policy.matches("my_tool_test", &serde_json::json!({})));
    assert!(!policy.matches("bash", &serde_json::json!({})));
}

/// 测试 ValuePattern Exact 匹配
/// Exact 应精确匹配整个值
#[test]
fn test_value_pattern_exact() {
    let pattern = ValuePattern::Exact(serde_json::json!({"path": "/tmp/test"}));

    // 精确匹配
    assert!(pattern.matches(&serde_json::json!({"path": "/tmp/test"})));

    // 不匹配不同的值
    assert!(!pattern.matches(&serde_json::json!({"path": "/tmp/other"})));
}

/// 测试 ValuePattern Contains 匹配
/// Contains 应检查字符串是否包含子串
#[test]
fn test_value_pattern_contains() {
    let pattern = ValuePattern::Contains("dangerous".to_string());

    // 包含 dangerous
    assert!(pattern.matches(&serde_json::json!("this is dangerous")));

    // 不包含
    assert!(!pattern.matches(&serde_json::json!("safe command")));
}

/// 测试 ValuePattern Regex 匹配
/// Regex 应使用正则表达式匹配
#[test]
fn test_value_pattern_regex() {
    let pattern = ValuePattern::Regex("^test.*".to_string());

    // 匹配正则
    assert!(pattern.matches(&serde_json::json!("test_file")));
    assert!(pattern.matches(&serde_json::json!("testing")));

    // 不匹配
    assert!(!pattern.matches(&serde_json::json!("other_file")));
}
