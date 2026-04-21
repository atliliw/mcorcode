//! Provider Types and Configurations

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Anthropic,
    OpenAI,
    GoogleGemini,
    AWSBedrock,
    AzureOpenAI,
    Groq,
    OpenRouter,
    Ollama,
    LMStudio,
    Custom { endpoint: String },
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::Anthropic => "anthropic",
            ProviderType::OpenAI => "openai",
            ProviderType::GoogleGemini => "google",
            ProviderType::AWSBedrock => "bedrock",
            ProviderType::AzureOpenAI => "azure",
            ProviderType::Groq => "groq",
            ProviderType::OpenRouter => "openrouter",
            ProviderType::Ollama => "ollama",
            ProviderType::LMStudio => "lmstudio",
            ProviderType::Custom { .. } => "custom",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "anthropic" => Some(ProviderType::Anthropic),
            "openai" => Some(ProviderType::OpenAI),
            "google" | "gemini" => Some(ProviderType::GoogleGemini),
            "bedrock" => Some(ProviderType::AWSBedrock),
            "azure" => Some(ProviderType::AzureOpenAI),
            "groq" => Some(ProviderType::Groq),
            "openrouter" => Some(ProviderType::OpenRouter),
            "ollama" => Some(ProviderType::Ollama),
            "lmstudio" => Some(ProviderType::LMStudio),
            _ => None,
        }
    }

    pub fn default_endpoint(&self) -> Option<&str> {
        match self {
            ProviderType::Anthropic => Some("https://api.anthropic.com/v1"),
            ProviderType::OpenAI => Some("https://api.openai.com/v1"),
            ProviderType::GoogleGemini => Some("https://generativelanguage.googleapis.com/v1"),
            ProviderType::Groq => Some("https://api.groq.com/openai/v1"),
            ProviderType::OpenRouter => Some("https://openrouter.ai/api/v1"),
            ProviderType::Ollama => Some("http://localhost:11434"),
            ProviderType::LMStudio => Some("http://localhost:1234"),
            _ => None,
        }
    }

    pub fn default_model(&self) -> Option<&str> {
        match self {
            ProviderType::Anthropic => Some("claude-3-5-sonnet-20241022"),
            ProviderType::OpenAI => Some("gpt-4"),
            ProviderType::GoogleGemini => Some("gemini-1.5-pro"),
            ProviderType::Groq => Some("llama-3.1-70b-versatile"),
            ProviderType::OpenRouter => Some("anthropic/claude-3.5-sonnet"),
            ProviderType::Ollama => Some("llama3"),
            ProviderType::LMStudio => Some("local-model"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: ProviderType,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub models: Vec<ModelConfig>,
}

impl ProviderConfig {
    pub fn new(provider: ProviderType) -> Self {
        Self {
            provider: provider.clone(),
            api_key: None,
            base_url: provider.default_endpoint().map(|s| s.to_string()),
            models: vec![],
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn with_model(mut self, model: ModelConfig) -> Self {
        self.models.push(model);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub supports_tools: bool,
    pub supports_streaming: bool,
    pub supports_thinking: bool,
    pub cost_per_1k_input: Option<f32>,
    pub cost_per_1k_output: Option<f32>,
}

impl ModelConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            max_tokens: 4096,
            supports_tools: true,
            supports_streaming: true,
            supports_thinking: false,
            cost_per_1k_input: None,
            cost_per_1k_output: None,
        }
    }

    pub fn anthropic_default() -> Self {
        Self {
            name: "claude-3-5-sonnet-20241022".to_string(),
            display_name: Some("Claude 3.5 Sonnet".to_string()),
            max_tokens: 8192,
            supports_tools: true,
            supports_streaming: true,
            supports_thinking: true,
            cost_per_1k_input: Some(3.0),
            cost_per_1k_output: Some(15.0),
        }
    }

    pub fn openai_default() -> Self {
        Self {
            name: "gpt-4".to_string(),
            display_name: Some("GPT-4".to_string()),
            max_tokens: 8192,
            supports_tools: true,
            supports_streaming: true,
            supports_thinking: false,
            cost_per_1k_input: Some(30.0),
            cost_per_1k_output: Some(60.0),
        }
    }
}
