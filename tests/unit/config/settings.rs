//! Settings 配置设置单元测试

use mcorcode::config::Settings;

/// 测试 Settings 的创建
/// 新建的配置应有默认值：API base、model、max_tokens、enabled_tools、workdir
#[test]
fn test_settings_new() {
    let settings = Settings::new();

    // 默认 API base
    assert_eq!(settings.api_base, "https://api.anthropic.com/v1");

    // 默认 model
    assert_eq!(settings.model, "claude-3-5-sonnet-20241022");

    // 默认 max_tokens
    assert_eq!(settings.max_tokens, 100000);

    // 默认 enabled_tools
    assert_eq!(settings.enabled_tools.len(), 6);
    assert!(settings.enabled_tools.contains(&"read_file".to_string()));
    assert!(settings.enabled_tools.contains(&"write_file".to_string()));
    assert!(settings.enabled_tools.contains(&"edit_file".to_string()));
    assert!(settings.enabled_tools.contains(&"bash".to_string()));
    assert!(settings.enabled_tools.contains(&"grep".to_string()));
    assert!(settings.enabled_tools.contains(&"glob".to_string()));

    // 默认 API key 为空
    assert!(settings.api_key.is_empty());
}

/// 测试 Settings 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.api_base, "https://api.anthropic.com/v1");
    assert!(settings.api_key.is_empty());
}

/// 测试 with_api_key builder 方法
/// with_api_key 应设置自定义 API key
#[test]
fn test_settings_with_api_key() {
    let settings = Settings::new().with_api_key("my-secret-key");
    assert_eq!(settings.api_key, "my-secret-key");
}

/// 测试 with_model builder 方法
/// with_model 应设置自定义 model 名称
#[test]
fn test_settings_with_model() {
    let settings = Settings::new().with_model("claude-3-opus");
    assert_eq!(settings.model, "claude-3-opus");
}

/// 测试 with_workdir builder 方法
/// with_workdir 应设置自定义工作目录
#[test]
fn test_settings_with_workdir() {
    let settings = Settings::new().with_workdir("/custom/path");
    assert_eq!(settings.workdir, "/custom/path");
}

/// 测试 builder 方法链式调用
/// 所有 builder 方法应能链式组合
#[test]
fn test_settings_builder_chain() {
    let settings = Settings::new()
        .with_api_key("test-key")
        .with_model("custom-model")
        .with_workdir("/test/dir");

    assert_eq!(settings.api_key, "test-key");
    assert_eq!(settings.model, "custom-model");
    assert_eq!(settings.workdir, "/test/dir");
}

/// 测试 from_env 创建配置
/// from_env 应从环境变量读取配置（此测试不设置环境变量，验证默认值）
#[test]
fn test_settings_from_env_default() {
    // 不设置环境变量时，应使用默认值
    let settings = Settings::from_env();

    // API key 应为空（未设置环境变量）
    // 注意：此测试假设测试环境未设置相关环境变量
    assert_eq!(settings.api_base, "https://api.anthropic.com/v1");
}
