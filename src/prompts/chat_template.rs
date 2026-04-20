use crate::schema::Message;
use std::collections::HashMap;

pub struct ChatPromptTemplate {
    system_template: Option<String>,
    human_template: String,
    ai_template: Option<String>,
}

impl ChatPromptTemplate {
    pub fn new(human_template: impl Into<String>) -> Self {
        Self {
            system_template: None,
            human_template: human_template.into(),
            ai_template: None,
        }
    }

    pub fn with_system(mut self, template: impl Into<String>) -> Self {
        self.system_template = Some(template.into());
        self
    }

    pub fn with_ai(mut self, template: impl Into<String>) -> Self {
        self.ai_template = Some(template.into());
        self
    }

    pub fn format_messages(&self, inputs: HashMap<&str, &str>) -> Vec<Message> {
        let mut messages = Vec::new();

        if let Some(sys) = &self.system_template {
            messages.push(Message::system(format_template(sys, &inputs)));
        }

        messages.push(Message::human(format_template(
            &self.human_template,
            &inputs,
        )));

        messages
    }

    pub fn format(&self, input: &str) -> Vec<Message> {
        let inputs = HashMap::from([("input", input)]);
        self.format_messages(inputs)
    }
}

fn format_template(template: &str, inputs: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in inputs {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

impl Default for ChatPromptTemplate {
    fn default() -> Self {
        Self::new("{input}")
    }
}
