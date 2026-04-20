use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::fs;

use super::base::Tool;

pub struct ReadTool {
    workdir: String,
}

impl ReadTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read file contents from the filesystem"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the file to read"
                },
                "offset": {
                    "type": "integer",
                    "description": "The line number to start reading from (1-indexed)"
                },
                "limit": {
                    "type": "integer",
                    "description": "The maximum number of lines to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str()
            .context("Missing 'path' parameter")?;

        let full_path = Path::new(&self.workdir).join(path);

        let content = fs::read_to_string(&full_path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", full_path))?;

        let offset = args["offset"].as_u64().unwrap_or(1) as usize;
        let limit = args["limit"].as_u64().unwrap_or(2000) as usize;

        let lines: Vec<&str> = content.lines().collect();
        let start = (offset - 1).min(lines.len());
        let end = (start + limit).min(lines.len());

        let result: String = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{}: {}", start + i + 1, line))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(result)
    }
}