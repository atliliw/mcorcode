use crate::permission::checker::PermissionResult;
use serde_json::Value;

pub struct PermissionPolicy {
    pub tool_pattern: String,
    pub input_pattern: Option<ValuePattern>,
    pub result: PermissionResult,
}

impl PermissionPolicy {
    pub fn allow_tool(tool: impl Into<String>) -> Self {
        Self {
            tool_pattern: tool.into(),
            input_pattern: None,
            result: PermissionResult::allow(),
        }
    }

    pub fn deny_tool(tool: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            tool_pattern: tool.into(),
            input_pattern: None,
            result: PermissionResult::deny(reason),
        }
    }

    pub fn matches(&self, tool: &str, input: &Value) -> bool {
        if !pattern_matches(&self.tool_pattern, tool) {
            return false;
        }

        if let Some(pattern) = &self.input_pattern {
            pattern.matches(input)
        } else {
            true
        }
    }
}

pub enum ValuePattern {
    Exact(Value),
    Contains(String),
    Regex(String),
}

impl ValuePattern {
    pub fn matches(&self, value: &Value) -> bool {
        match self {
            ValuePattern::Exact(expected) => value == expected,
            ValuePattern::Contains(s) => {
                if let Some(str_val) = value.as_str() {
                    str_val.contains(s)
                } else if let Some(obj) = value.as_object() {
                    obj.values()
                        .any(|v| v.as_str().map_or(false, |s| s.contains(s)))
                } else {
                    false
                }
            }
            ValuePattern::Regex(pattern) => {
                let re = regex::Regex::new(pattern).unwrap();
                if let Some(str_val) = value.as_str() {
                    re.is_match(str_val)
                } else {
                    false
                }
            }
        }
    }
}

fn pattern_matches(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if pattern.starts_with("*") && pattern.ends_with("*") {
        let middle = &pattern[1..pattern.len() - 1];
        return value.contains(middle);
    }

    if pattern.starts_with("*") {
        return value.ends_with(&pattern[1..]);
    }

    if pattern.ends_with("*") {
        return value.starts_with(&pattern[..pattern.len() - 1]);
    }

    pattern == value
}
