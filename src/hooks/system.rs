//! Hook System Implementation

use std::collections::HashMap;
use serde_json::Value;
use async_trait::async_trait;

use crate::hooks::{HookTrigger, HookAction, HookResult};

pub struct Hook {
    pub trigger: HookTrigger,
    pub tool_filter: Option<String>,
    pub action: HookAction,
    pub priority: usize,
}

impl Hook {
    pub fn new(trigger: HookTrigger, action: HookAction) -> Self {
        Self {
            trigger,
            tool_filter: None,
            action,
            priority: 0,
        }
    }
    
    pub fn for_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool_filter = Some(tool.into());
        self
    }
    
    pub fn with_priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn matches(&self, trigger: HookTrigger, tool: &str) -> bool {
        self.trigger == trigger && 
        self.tool_filter.as_ref().map_or(true, |f| pattern_matches(f, tool))
    }
}

pub struct HookSystem {
    hooks: Vec<Hook>,
}

impl HookSystem {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
        }
    }
    
    pub fn add(mut self, hook: Hook) -> Self {
        self.hooks.push(hook);
        self.hooks.sort_by_key(|h| h.priority);
        self
    }
    
    pub fn execute_sync(&self, trigger: HookTrigger, tool: &str, input: &Value) -> HookResult {
        for hook in &self.hooks {
            if hook.matches(trigger, tool) {
                return self.execute_action(&hook.action, input);
            }
        }
        HookResult::Continue
    }
    
    pub async fn execute(&self, trigger: HookTrigger, tool: &str, input: &Value) -> HookResult {
        for hook in &self.hooks {
            if hook.matches(trigger, tool) {
                return self.execute_action_async(&hook.action, input).await;
            }
        }
        HookResult::Continue
    }
    
    fn execute_action(&self, action: &HookAction, _input: &Value) -> HookResult {
        match action {
            HookAction::AutoApprove => HookResult::Approved,
            HookAction::AutoDeny { reason } => HookResult::Denied(reason.clone()),
            HookAction::AskUser => HookResult::AskUser,
            HookAction::ValidateInput { schema } => {
                // TODO: validate
                HookResult::Continue
            },
            HookAction::RunCommand { cmd } => {
                // TODO: run command
                HookResult::Continue
            },
            HookAction::LogToFile { path } => {
                // TODO: log
                HookResult::Continue
            },
        }
    }
    
    async fn execute_action_async(&self, action: &HookAction, input: &Value) -> HookResult {
        match action {
            HookAction::AutoApprove => HookResult::Approved,
            HookAction::AutoDeny { reason } => HookResult::Denied(reason.clone()),
            HookAction::AskUser => HookResult::AskUser,
            HookAction::ValidateInput { schema } => {
                self.validate_input(schema, input)
            },
            HookAction::RunCommand { cmd } => {
                self.run_command(cmd).await
            },
            HookAction::LogToFile { path } => {
                self.log_to_file(path, input).await
            },
        }
    }
    
    fn validate_input(&self, schema: &Value, input: &Value) -> HookResult {
        // Simple validation: check required fields
        if let Some(obj) = schema.as_object() {
            if let Some(required) = obj.get("required").and_then(|r| r.as_array()) {
                let input_obj = input.as_object();
                for field in required {
                    if let Some(field_name) = field.as_str() {
                        if input_obj.map_or(true, |o| !o.contains_key(field_name)) {
                            return HookResult::Denied(format!("Missing required field: {}", field_name));
                        }
                    }
                }
            }
        }
        HookResult::Continue
    }
    
    async fn run_command(&self, cmd: &str) -> HookResult {
        let result = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .await;
        
        match result {
            Ok(output) => {
                if output.status.success() {
                    HookResult::Continue
                } else {
                    HookResult::Denied(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)))
                }
            },
            Err(e) => HookResult::Denied(format!("Command error: {}", e)),
        }
    }
    
    async fn log_to_file(&self, path: &str, input: &Value) -> HookResult {
        let content = serde_json::to_string(input).unwrap_or_default();
        let _ = tokio::fs::write(path, content).await;
        HookResult::Continue
    }
}

impl Default for HookSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn pattern_matches(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.starts_with("*") && pattern.ends_with("*") {
        let middle = &pattern[1..pattern.len()-1];
        return value.contains(middle);
    }
    pattern == value
}