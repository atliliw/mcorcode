use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::Value;
use tokio::process::Command;

use super::base::Tool;

pub struct BashTool {
    workdir: String,
}

impl BashTool {
    pub fn new(workdir: &str) -> Self {
        Self {
            workdir: workdir.to_string(),
        }
    }
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute a bash command in the working directory"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The command to execute"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in milliseconds (default 120000)"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let command = args["command"].as_str()
            .context("Missing 'command' parameter")?;

        let timeout = args["timeout"].as_u64().unwrap_or(120000);

        let output = tokio::time::timeout(
            std::time::Duration::from_millis(timeout),
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&self.workdir)
                .output()
        )
        .await
        .context("Command timed out")??;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Ok(format!("Exit code: {}\nStdout: {}\nStderr: {}",
                output.status.code().unwrap_or(-1),
                stdout,
                stderr
            ))
        }
    }
}