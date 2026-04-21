//! 工具定义类型系统
//!
//! 提供工具定义的统一格式，支持转换为不同 LLM API 格式

use serde::{Deserialize, Serialize};

pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

impl ToolDefinition {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: serde_json::json!({"type": "object", "properties": {}}),
        }
    }

    pub fn with_parameters(mut self, params: serde_json::Value) -> Self {
        self.parameters = params;
        self
    }

    pub fn to_openai_format(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": self.parameters
            }
        })
    }

    pub fn to_anthropic_format(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "input_schema": self.parameters
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

impl From<ToolDefinition> for FunctionSpec {
    fn from(def: ToolDefinition) -> Self {
        Self {
            name: def.name,
            description: def.description,
            parameters: def.parameters,
        }
    }
}

impl FunctionSpec {
    pub fn to_openai_function(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "parameters": self.parameters
        })
    }
}
