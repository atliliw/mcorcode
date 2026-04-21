//! ProviderType、ProviderConfig、ModelConfig 单元测试

use mcorcode::llm::{ModelConfig, ProviderConfig, ProviderType};

/// 测试所有 ProviderType 的 as_str 方法
/// 每种提供商类型应返回正确的字符串标识
#[test]
fn test_provider_type_as_str() {
    assert_eq!(ProviderType::Anthropic.as_str(), "anthropic");
    assert_eq!(ProviderType::OpenAI.as_str(), "openai");
    assert_eq!(ProviderType::GoogleGemini.as_str(), "google");
    assert_eq!(ProviderType::Groq.as_str(), "groq");
    assert_eq!(ProviderType::Ollama.as_str(), "ollama");
    assert_eq!(ProviderType::OpenRouter.as_str(), "openrouter");
    assert_eq!(ProviderType::LMStudio.as_str(), "lmstudio");
    assert_eq!(ProviderType::AWSBedrock.as_str(), "bedrock");
    assert_eq!(ProviderType::AzureOpenAI.as_str(), "azure");
}

/// 测试 from_str 解析有效提供商字符串
/// 所有有效字符串应解析为正确的 ProviderType
#[test]
fn test_provider_type_from_str_valid() {
    assert_eq!(
        ProviderType::from_str("anthropic"),
        Some(ProviderType::Anthropic)
    );
    assert_eq!(ProviderType::from_str("openai"), Some(ProviderType::OpenAI));
    assert_eq!(
        ProviderType::from_str("google"),
        Some(ProviderType::GoogleGemini)
    );
    assert_eq!(
        ProviderType::from_str("gemini"),
        Some(ProviderType::GoogleGemini)
    );
    assert_eq!(
        ProviderType::from_str("bedrock"),
        Some(ProviderType::AWSBedrock)
    );
    assert_eq!(
        ProviderType::from_str("azure"),
        Some(ProviderType::AzureOpenAI)
    );
    assert_eq!(ProviderType::from_str("groq"), Some(ProviderType::Groq));
    assert_eq!(
        ProviderType::from_str("openrouter"),
        Some(ProviderType::OpenRouter)
    );
    assert_eq!(ProviderType::from_str("ollama"), Some(ProviderType::Ollama));
    assert_eq!(
        ProviderType::from_str("lmstudio"),
        Some(ProviderType::LMStudio)
    );
}

/// 测试 from_str 解析无效字符串
/// 无效字符串应返回 None
#[test]
fn test_provider_type_from_str_invalid() {
    assert_eq!(ProviderType::from_str("invalid"), None);
    assert_eq!(ProviderType::from_str(""), None);
    assert_eq!(ProviderType::from_str("unknown"), None);
}

/// 测试各提供商的 default_endpoint 方法
/// 有默认端点的提供商应返回 Some
#[test]
fn test_provider_type_default_endpoint() {
    assert!(ProviderType::Anthropic.default_endpoint().is_some());
    assert!(ProviderType::OpenAI.default_endpoint().is_some());
    assert!(ProviderType::Groq.default_endpoint().is_some());
    assert!(ProviderType::Ollama.default_endpoint().is_some());
    assert!(ProviderType::LMStudio.default_endpoint().is_some());
    assert!(ProviderType::OpenRouter.default_endpoint().is_some());
}

/// 测试 default_endpoint 返回的具体值
/// 验证默认端点 URL 正确
#[test]
fn test_provider_type_default_endpoint_values() {
    assert_eq!(
        ProviderType::OpenAI.default_endpoint(),
        Some("https://api.openai.com/v1")
    );
    assert_eq!(
        ProviderType::Ollama.default_endpoint(),
        Some("http://localhost:11434")
    );
    assert_eq!(
        ProviderType::LMStudio.default_endpoint(),
        Some("http://localhost:1234")
    );
}

/// 测试各提供商的 default_model 方法
/// 有默认模型的提供商应返回 Some
#[test]
fn test_provider_type_default_model() {
    assert!(ProviderType::Anthropic.default_model().is_some());
    assert!(ProviderType::OpenAI.default_model().is_some());
    assert!(ProviderType::Groq.default_model().is_some());
    assert!(ProviderType::Ollama.default_model().is_some());
}

/// 测试 default_model 返回的具体值
/// 验证默认模型名称正确
#[test]
fn test_provider_type_default_model_values() {
    assert_eq!(
        ProviderType::Anthropic.default_model(),
        Some("claude-3-5-sonnet-20241022")
    );
    assert_eq!(ProviderType::OpenAI.default_model(), Some("gpt-4"));
    assert_eq!(ProviderType::Ollama.default_model(), Some("llama3"));
}

/// 测试 ProviderConfig 的创建
/// 新配置应设置提供商类型，其他字段为空
#[test]
fn test_provider_config_new() {
    let config = ProviderConfig::new(ProviderType::OpenAI);
    assert_eq!(config.provider, ProviderType::OpenAI);
    assert!(config.api_key.is_none());
    assert!(config.models.is_empty());
}

/// 测试 with_api_key 方法
/// API key 应正确设置
#[test]
fn test_provider_config_with_api_key() {
    let config = ProviderConfig::new(ProviderType::OpenAI).with_api_key("test-api-key");
    assert_eq!(config.api_key, Some("test-api-key".to_string()));
}

/// 测试 with_base_url 方法
/// 自定义端点 URL 应正确设置
#[test]
fn test_provider_config_with_base_url() {
    let config = ProviderConfig::new(ProviderType::OpenAI).with_base_url("https://custom.api.com");
    assert_eq!(config.base_url, Some("https://custom.api.com".to_string()));
}

/// 测试 with_model 方法
/// 模型配置应添加到 models 列表
#[test]
fn test_provider_config_with_model() {
    let model = ModelConfig::new("gpt-4-turbo");
    let config = ProviderConfig::new(ProviderType::OpenAI).with_model(model);
    assert_eq!(config.models.len(), 1);
}

/// 测试链式配置构建
/// 多个配置方法可链式调用
#[test]
fn test_provider_config_chained_builders() {
    let config = ProviderConfig::new(ProviderType::Anthropic)
        .with_api_key("key")
        .with_base_url("https://custom.url");

    assert_eq!(config.provider, ProviderType::Anthropic);
    assert_eq!(config.api_key, Some("key".to_string()));
    assert_eq!(config.base_url, Some("https://custom.url".to_string()));
}

/// 测试 ModelConfig 的创建
/// 新模型配置应有名称和默认设置
#[test]
fn test_model_config_new() {
    let config = ModelConfig::new("gpt-4");
    assert_eq!(config.name, "gpt-4");
    assert!(config.display_name.is_none());
    assert!(config.supports_tools);
    assert!(config.supports_streaming);
    assert!(!config.supports_thinking);
}

/// 测试 Anthropic 默认模型配置
/// Claude 模型应支持思考功能
#[test]
fn test_model_config_anthropic_default() {
    let config = ModelConfig::anthropic_default();
    assert_eq!(config.name, "claude-3-5-sonnet-20241022");
    assert_eq!(config.display_name, Some("Claude 3.5 Sonnet".to_string()));
    assert!(config.supports_tools);
    assert!(config.supports_streaming);
    assert!(config.supports_thinking);
}

/// 测试 OpenAI 默认模型配置
/// GPT-4 不支持思考功能
#[test]
fn test_model_config_openai_default() {
    let config = ModelConfig::openai_default();
    assert_eq!(config.name, "gpt-4");
    assert_eq!(config.display_name, Some("GPT-4".to_string()));
    assert!(config.supports_tools);
    assert!(config.supports_streaming);
    assert!(!config.supports_thinking);
}

/// 测试 ProviderType 的相等性比较
/// 相同提供商应相等，不同提供商不应相等
#[test]
fn test_provider_type_equality() {
    assert_eq!(ProviderType::OpenAI, ProviderType::OpenAI);
    assert_eq!(ProviderType::Anthropic, ProviderType::Anthropic);
    assert_ne!(ProviderType::OpenAI, ProviderType::Anthropic);
}

/// 测试 ProviderType 的 Clone trait
/// 克隆的提供商类型应与原类型相等
#[test]
fn test_provider_type_clone() {
    let provider = ProviderType::Groq;
    let cloned = provider.clone();
    assert_eq!(provider, cloned);
}
