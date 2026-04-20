use super::settings::Settings;
use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(path: &PathBuf) -> Result<Settings> {
        if !path.exists() {
            return Ok(Settings::from_env());
        }

        let content = std::fs::read_to_string(path)?;
        let config: TomlConfig = toml::from_str(&content)?;

        let mut settings = Settings::from_env();

        if let Some(base) = config.mcorcode.api_base {
            settings.api_base = base;
        }

        if let Some(model) = config.mcorcode.model {
            settings.model = model;
        }

        if let Some(tokens) = config.context.max_tokens {
            settings.max_tokens = tokens;
        }

        if let Some(tools) = config.tools.enabled {
            settings.enabled_tools = tools;
        }

        Ok(settings)
    }

    pub fn load() -> Result<Settings> {
        let workdir = std::env::current_dir()?;
        let config_path = workdir.join(".mcorcode.toml");

        Self::load_from_file(&config_path)
    }

    pub fn create_example(path: &PathBuf) -> Result<()> {
        let example = include_str!("../../.mcorcode.toml.example");
        std::fs::write(path, example)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    mcorcode: McorcodeSection,
    #[serde(default)]
    tools: ToolsSection,
    #[serde(default)]
    context: ContextSection,
}

#[derive(Debug, Deserialize, Default)]
struct ToolsSection {
    enabled: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
struct ContextSection {
    max_tokens: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct McorcodeSection {
    #[serde(default)]
    api_base: Option<String>,
    #[serde(default)]
    model: Option<String>,
}
