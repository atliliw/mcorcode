use serde::{Deserialize, Serialize};

pub struct Settings {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: usize,
    pub enabled_tools: Vec<String>,
    pub workdir: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            api_base: "https://api.anthropic.com/v1".to_string(),
            api_key: String::new(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 100000,
            enabled_tools: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "edit_file".to_string(),
                "bash".to_string(),
                "grep".to_string(),
                "glob".to_string(),
            ],
            workdir: std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        }
    }

    pub fn from_env() -> Self {
        let mut settings = Self::new();

        if let Ok(key) = std::env::var("MCORCODE_API_KEY") {
            settings.api_key = key;
        } else if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            settings.api_key = key;
        } else if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            settings.api_key = key;
        }

        if let Ok(base) = std::env::var("MCORCODE_API_BASE") {
            settings.api_base = base;
        }

        if let Ok(model) = std::env::var("MCORCODE_MODEL") {
            settings.model = model;
        }

        settings
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = key.into();
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_workdir(mut self, workdir: impl Into<String>) -> Self {
        self.workdir = workdir.into();
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
