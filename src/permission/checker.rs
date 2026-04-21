use serde_json::Value;
use std::collections::HashMap;

use crate::hooks::{HookResult, HookSystem, HookTrigger};
use crate::permission::mode::PermissionMode;
use crate::permission::policy::PermissionPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionAction {
    Allow,
    Deny,
    AskUser,
    Sandbox,
}

#[derive(Debug, Clone)]
pub struct PermissionResult {
    pub action: PermissionAction,
    pub reason: Option<String>,
}

impl PermissionResult {
    pub fn allow() -> Self {
        Self {
            action: PermissionAction::Allow,
            reason: None,
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            action: PermissionAction::Deny,
            reason: Some(reason.into()),
        }
    }

    pub fn ask_user() -> Self {
        Self {
            action: PermissionAction::AskUser,
            reason: None,
        }
    }

    pub fn sandbox() -> Self {
        Self {
            action: PermissionAction::Sandbox,
            reason: None,
        }
    }

    pub fn is_allowed(&self) -> bool {
        matches!(self.action, PermissionAction::Allow)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self.action, PermissionAction::Deny)
    }
}

pub struct PermissionChecker {
    mode: PermissionMode,
    policies: Vec<PermissionPolicy>,
    hooks: Option<HookSystem>,
    dangerous_commands: Vec<String>,
    allowed_paths: Option<Vec<String>>,
}

impl PermissionChecker {
    pub fn new(mode: PermissionMode) -> Self {
        Self {
            mode,
            policies: Vec::new(),
            hooks: None,
            dangerous_commands: vec![
                "rm -rf".to_string(),
                "sudo".to_string(),
                "chmod 777".to_string(),
                "dd if=".to_string(),
                "mkfs".to_string(),
                "fdisk".to_string(),
                ":(){ :|:& };:".to_string(), // fork bomb
            ],
            allowed_paths: None,
        }
    }

    pub fn with_hooks(mut self, hooks: HookSystem) -> Self {
        self.hooks = Some(hooks);
        self
    }

    pub fn with_policy(mut self, policy: PermissionPolicy) -> Self {
        self.policies.push(policy);
        self
    }

    pub fn with_dangerous_commands(mut self, cmds: Vec<String>) -> Self {
        self.dangerous_commands = cmds;
        self
    }

    pub fn with_allowed_paths(mut self, paths: Vec<String>) -> Self {
        self.allowed_paths = Some(paths);
        self
    }

    pub fn check(&self, tool: &str, input: &Value) -> PermissionResult {
        // 1. Hook 检查
        if let Some(hooks) = &self.hooks {
            let result = hooks.execute_sync(HookTrigger::PreToolCall, tool, input);
            if result.is_denied() {
                return PermissionResult::deny("Denied by hook");
            }
            if result.is_approved() {
                return PermissionResult::allow();
            }
        }

        // 2. Policy 检查
        for policy in &self.policies {
            if policy.matches(tool, input) {
                return policy.result.clone();
            }
        }

        // 3. 工具特定检查
        if tool == "bash" {
            return self.check_bash(input);
        }

        if is_file_tool(tool) {
            return self.check_file_tool(tool, input);
        }

        // 4. 模式检查
        match self.mode {
            PermissionMode::AcceptAll => PermissionResult::allow(),

            PermissionMode::PlanMode => PermissionResult::deny("Plan mode - no execution"),

            PermissionMode::AcceptEdits => {
                if is_file_tool(tool) {
                    PermissionResult::allow()
                } else if is_shell_tool(tool) {
                    PermissionResult::ask_user()
                } else {
                    PermissionResult::ask_user()
                }
            }

            PermissionMode::Sandbox => PermissionResult::sandbox(),

            PermissionMode::Default => PermissionResult::ask_user(),
        }
    }

    fn check_bash(&self, input: &Value) -> PermissionResult {
        let command = input.get("command").and_then(|v| v.as_str()).unwrap_or("");

        for dangerous in &self.dangerous_commands {
            if command.contains(dangerous) {
                return PermissionResult::deny(format!(
                    "Dangerous command detected: {}",
                    dangerous
                ));
            }
        }

        match self.mode {
            PermissionMode::AcceptAll => PermissionResult::allow(),
            PermissionMode::PlanMode => PermissionResult::deny("Plan mode"),
            _ => PermissionResult::ask_user(),
        }
    }

    fn check_file_tool(&self, tool: &str, input: &Value) -> PermissionResult {
        let path = input.get("path").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(allowed) = &self.allowed_paths {
            let is_allowed = allowed.iter().any(|p| path.starts_with(p));
            if !is_allowed {
                return PermissionResult::deny(format!("Path not allowed: {}", path));
            }
        }

        match self.mode {
            PermissionMode::AcceptAll => PermissionResult::allow(),
            PermissionMode::AcceptEdits => PermissionResult::allow(),
            PermissionMode::PlanMode => PermissionResult::deny("Plan mode"),
            PermissionMode::Sandbox => PermissionResult::sandbox(),
            PermissionMode::Default => PermissionResult::ask_user(),
        }
    }

    pub fn set_mode(&mut self, mode: PermissionMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> PermissionMode {
        self.mode
    }
}

fn is_file_tool(tool: &str) -> bool {
    matches!(tool, "read" | "write" | "edit" | "glob")
}

fn is_shell_tool(tool: &str) -> bool {
    matches!(tool, "bash" | "git")
}
