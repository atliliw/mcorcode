use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::fs;

use super::base::Tool;

pub struct GlobTool {
    workdir: String,
}

impl GlobTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Find files matching a glob pattern"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The glob pattern to match (e.g., '**/*.rs')"
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search in (defaults to working directory)"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let pattern = args["pattern"].as_str()
            .context("Missing 'pattern' parameter")?;
        let search_path = args["path"].as_str().unwrap_or(".");

        let full_path = Path::new(&self.workdir).join(search_path);

        let mut results = Vec::new();
        self.glob_search(&full_path, pattern, &mut results).await?;

        if results.is_empty() {
            Ok("No files found".to_string())
        } else {
            Ok(results.join("\n"))
        }
    }
}

impl GlobTool {
    async fn glob_search(
        &self,
        dir: &Path,
        pattern: &str,
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
                    Box::pin(self.glob_search(&path, pattern, results)).await?;
                }
            } else if path.is_file() {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if self.matches_pattern(file_name, pattern) {
                    let rel_path = path.strip_prefix(&self.workdir)
                        .unwrap_or(&path);
                    results.push(rel_path.display().to_string());
                }
            }
        }

        Ok(())
    }

    fn matches_pattern(&self, filename: &str, pattern: &str) -> bool {
        let pattern = pattern.trim_start_matches("**/").trim_start_matches("*/");

        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            filename.ends_with(suffix)
        } else if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len()-1];
            filename.starts_with(prefix)
        } else {
            filename == pattern
        }
    }
}