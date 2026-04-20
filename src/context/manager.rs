use crate::llm::{Message, ToolCall};

pub struct ContextManager {
    messages: Vec<Message>,
    max_tokens: usize,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_tokens: 100000,
        }
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(Message {
            role: "user".to_string(),
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub fn add_assistant_message(&mut self, response: &crate::llm::LlmResponse) {
        self.messages.push(Message {
            role: "assistant".to_string(),
            content: response.content.clone(),
            tool_calls: response.tool_calls.clone(),
            tool_call_id: None,
        });
    }

    pub fn add_tool_result(&mut self, tool_call_id: &str, result: &str) {
        self.messages.push(Message {
            role: "tool".to_string(),
            content: result.to_string(),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.to_string()),
        });
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}
