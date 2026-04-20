use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::fs;

use super::base::Tool;

pub struct WriteTool {
    workdir: String,
}

impl WriteTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file, creating it if it doesn't exist or overwriting if it does"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str()
            .context("Missing 'path' parameter")?;
        let content = args["content"].as_str()
            .context("Missing 'content' parameter")?;

        let full_path = Path::new(&self.workdir).join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        fs::write(&full_path, content)
            .await
            .with_context(|| format!("Failed to write file: {:?}", full_path))?;

        Ok(format!("Successfully wrote to {:?}", full_path))
    }
}