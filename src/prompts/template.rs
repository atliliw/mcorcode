use std::collections::HashMap;

pub struct PromptTemplate {
    template: String,
    input_variables: Vec<String>,
}

impl PromptTemplate {
    pub fn new(template: impl Into<String>, input_variables: Vec<&str>) -> Self {
        Self {
            template: template.into(),
            input_variables: input_variables.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn from_template(template: impl Into<String>) -> Self {
        let template_str = template.into();
        let vars = extract_variables(&template_str);
        Self {
            template: template_str,
            input_variables: vars,
        }
    }

    pub fn format(&self, inputs: HashMap<&str, &str>) -> String {
        let mut result = self.template.clone();
        for (key, value) in inputs {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    pub fn input_variables(&self) -> &[String] {
        &self.input_variables
    }

    pub fn template(&self) -> &str {
        &self.template
    }
}

fn extract_variables(template: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let mut start = 0;

    while let Some(open) = template[start..].find('{') {
        if let Some(close) = template[start + open..].find('}') {
            let var_start = start + open + 1;
            let var_end = start + open + close;
            if var_start < var_end {
                vars.push(template[var_start..var_end].to_string());
            }
            start = var_end + 1;
        } else {
            break;
        }
    }

    vars
}

impl Default for PromptTemplate {
    fn default() -> Self {
        Self::from_template("")
    }
}
