use std::collections::HashMap;
use std::sync::Arc;

use langchainrust::core::language_models::BaseChatModel;
use langchainrust::language_models::openai::OpenAIError;
use langchainrust::language_models::{OpenAIChat, OpenAIConfig};

use crate::llm::providers::{ProviderConfig, ProviderType};

pub struct ModelManager {
    providers: HashMap<String, ProviderConfig>,
    clients: HashMap<String, Arc<OpenAIChat>>,
    default_model: String,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            clients: HashMap::new(),
            default_model: "gpt-4".to_string(),
        }
    }

    pub fn from_env() -> Self {
        let mut manager = Self::new();

        // OpenAI
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            manager.add_openai(key);
        }

        // Anthropic via OpenRouter
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            manager.add_openrouter(key);
        }

        // Groq (OpenAI-compatible)
        if let Ok(key) = std::env::var("GROQ_API_KEY") {
            manager.add_groq(key);
        }

        // Ollama (local)
        manager.add_ollama();

        // 统一 key
        if let Ok(key) = std::env::var("MCORCODE_API_KEY") {
            if manager.clients.is_empty() {
                manager.add_openai(key);
            }
        }

        manager
    }

    fn add_openai(&mut self, api_key: String) {
        let config = ProviderConfig::new(ProviderType::OpenAI).with_api_key(api_key);

        let client = OpenAIChat::new(OpenAIConfig {
            api_key: config.api_key.clone().unwrap(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        });

        self.providers.insert("openai".to_string(), config);
        self.clients.insert("gpt-4".to_string(), Arc::new(client));
        self.default_model = "gpt-4".to_string();
    }

    fn add_openrouter(&mut self, api_key: String) {
        let config = ProviderConfig::new(ProviderType::OpenRouter).with_api_key(api_key);

        let client = OpenAIChat::new(OpenAIConfig {
            api_key: config.api_key.clone().unwrap(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            model: "anthropic/claude-3.5-sonnet".to_string(),
            ..Default::default()
        });

        self.providers.insert("openrouter".to_string(), config);
        self.clients
            .insert("claude-3.5-sonnet".to_string(), Arc::new(client));
        self.default_model = "claude-3.5-sonnet".to_string();
    }

    fn add_groq(&mut self, api_key: String) {
        let config = ProviderConfig::new(ProviderType::Groq).with_api_key(api_key);

        let client = OpenAIChat::new(OpenAIConfig {
            api_key: config.api_key.clone().unwrap(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            model: "llama-3.1-70b-versatile".to_string(),
            ..Default::default()
        });

        self.providers.insert("groq".to_string(), config);
        self.clients
            .insert("llama-3.1-70b-versatile".to_string(), Arc::new(client));
    }

    fn add_ollama(&mut self) {
        let config = ProviderConfig::new(ProviderType::Ollama);

        let client = OpenAIChat::new(OpenAIConfig {
            api_key: "ollama".to_string(),
            base_url: "http://localhost:11434/v1".to_string(),
            model: "llama3".to_string(),
            ..Default::default()
        });

        self.providers.insert("ollama".to_string(), config);
        self.clients.insert("llama3".to_string(), Arc::new(client));
    }

    pub fn get_client(&self, model_name: Option<&str>) -> Option<Arc<OpenAIChat>> {
        let name = model_name.unwrap_or(&self.default_model);
        self.clients.get(name).cloned()
    }

    pub fn set_default_model(&mut self, model: impl Into<String>) {
        self.default_model = model.into();
    }

    pub fn default_model(&self) -> &str {
        &self.default_model
    }

    pub fn list_models(&self) -> Vec<&str> {
        self.clients.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_manager_new() {
        let manager = ModelManager::new();
        assert!(manager.list_models().is_empty());
        assert_eq!(manager.default_model(), "gpt-4");
    }

    #[test]
    fn test_model_manager_default() {
        let manager = ModelManager::default();
        assert!(manager.list_models().is_empty());
    }

    #[test]
    fn test_model_manager_set_default_model() {
        let mut manager = ModelManager::new();
        manager.set_default_model("gpt-3.5-turbo");
        assert_eq!(manager.default_model(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_model_manager_get_client_none() {
        let manager = ModelManager::new();
        assert!(manager.get_client(None).is_none());
        assert!(manager.get_client(Some("gpt-4")).is_none());
    }
}
