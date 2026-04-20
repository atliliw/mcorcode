use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use futures::stream::{self, StreamExt};

use super::base::Tool;

pub struct GrepTool {
    workdir: String,
}

impl GrepTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for regex patterns in files"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search in (defaults to working directory)"
                },
                "include": {
                    "type": "string",
                    "description": "File pattern to include (e.g., '*.rs')"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let pattern = args["pattern"].as_str()
            .context("Missing 'pattern' parameter")?;
        let search_path = args["path"].as_str().unwrap_or(".");
        let include = args["include"].as_str();

        let regex = Regex::new(pattern)
            .with_context(|| format!("Invalid regex pattern: {}", pattern))?;

        let full_path = Path::new(&self.workdir).join(search_path);

        let mut results = Vec::new();
        self.search_directory(&full_path, &regex, include, &mut results).await?;

        if results.is_empty() {
            Ok("No matches found".to_string())
        } else {
            Ok(results.join("\n"))
        }
    }
}

impl GrepTool {
    async fn search_directory(
        &self,
        dir: &Path,
        regex: &Regex,
        include: Option<&str>,
        results: &mut Vec<String>,
    ) -> Result<()> {
        let mut entries = fs::read_dir(dir).await
            .with_context(|| format!("Failed to read directory: {:?}", dir))?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if !dir_name.starts_with('.') && dir_name != "target" && dir_name != "node_modules" {
                    Box::pin(self.search_directory(&path, regex, include, results)).await?;
                }
            } else if path.is_file() {
                let path_str = path.to_string_lossy();

                if let Some(inc) = include {
                    if !path_str.ends_with(inc) {
                        continue;
                    }
                }

                if let Ok(content) = fs::read_to_string(&path).await {
                    for (line_num, line) in content.lines().enumerate() {
                        if regex.is_match(line) {
                            let rel_path = path.strip_prefix(&self.workdir)
                                .unwrap_or(&path);
                            results.push(format!("{}:{}: {}", rel_path.display(), line_num + 1, line));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}