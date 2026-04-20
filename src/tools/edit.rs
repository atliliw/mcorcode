use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::fs;

use super::base::Tool;

pub struct EditTool {
    workdir: String,
}

impl EditTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit_file"
    }

    fn description(&self) -> &str {
        "Perform exact string replacements in a file"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the file to edit"
                },
                "oldString": {
                    "type": "string",
                    "description": "The text to replace"
                },
                "newString": {
                    "type": "string",
                    "description": "The text to replace it with"
                }
            },
            "required": ["path", "oldString", "newString"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str()
            .context("Missing 'path' parameter")?;
        let old_string = args["oldString"].as_str()
            .context("Missing 'oldString' parameter")?;
        let new_string = args["newString"].as_str()
            .context("Missing 'newString' parameter")?;

        let full_path = Path::new(&self.workdir).join(path);

        let content = fs::read_to_string(&full_path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", full_path))?;

        if !content.contains(old_string) {
            anyhow::bail!("oldString not found in file");
        }

        let new_content = content.replace(old_string, new_string);

        fs::write(&full_path, &new_content)
            .await
            .with_context(|| format!("Failed to write file: {:?}", full_path))?;

        Ok(format!("Successfully edited {:?}", full_path))
    }
}