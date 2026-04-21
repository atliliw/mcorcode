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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_template_new() {
        let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
        assert_eq!(template.input_variables(), &["name".to_string()]);
        assert_eq!(template.template(), "Hello {name}!");
    }

    #[test]
    fn test_prompt_template_from_template() {
        let template = PromptTemplate::from_template("Hello {name}, you are {role}!");
        assert_eq!(template.input_variables().len(), 2);
        assert!(template.input_variables().contains(&"name".to_string()));
        assert!(template.input_variables().contains(&"role".to_string()));
    }

    #[test]
    fn test_prompt_template_format() {
        let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
        let result = template.format(std::collections::HashMap::from([("name", "World")]));
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_prompt_template_format_multiple() {
        let template = PromptTemplate::new("{greeting} {name}!", vec!["greeting", "name"]);
        let result = template.format(std::collections::HashMap::from([
            ("greeting", "Hello"),
            ("name", "World"),
        ]));
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_prompt_template_missing_variable() {
        let template = PromptTemplate::new("Hello {name} {missing}!", vec!["name"]);
        let result = template.format(std::collections::HashMap::from([("name", "World")]));
        // Missing variable stays as placeholder
        assert_eq!(result, "Hello World {missing}!");
    }

    #[test]
    fn test_prompt_template_default() {
        let template = PromptTemplate::default();
        assert_eq!(template.template(), "");
        assert!(template.input_variables().is_empty());
    }

    #[test]
    fn test_extract_variables() {
        let vars = extract_variables("{a} and {b} and {c}");
        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&"a".to_string()));
        assert!(vars.contains(&"b".to_string()));
        assert!(vars.contains(&"c".to_string()));
    }
}
