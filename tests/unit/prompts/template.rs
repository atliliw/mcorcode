//! Unit tests for PromptTemplate

use mcorcode::prompts::PromptTemplate;
use std::collections::HashMap;

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
fn test_prompt_template_format_single_variable() {
    let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World")]));
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_prompt_template_format_multiple_variables() {
    let template = PromptTemplate::new("{greeting} {name}!", vec!["greeting", "name"]);
    let result = template.format(HashMap::from([("greeting", "Hello"), ("name", "World")]));
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_prompt_template_missing_variable_keeps_placeholder() {
    let template = PromptTemplate::new("Hello {name} {missing}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World")]));
    assert_eq!(result, "Hello World {missing}!");
}

#[test]
fn test_prompt_template_empty_template() {
    let template = PromptTemplate::new("", vec![]);
    assert_eq!(template.template(), "");
    assert!(template.input_variables().is_empty());
}

#[test]
fn test_prompt_template_default() {
    let template = PromptTemplate::default();
    assert_eq!(template.template(), "");
    assert!(template.input_variables().is_empty());
}

#[test]
fn test_prompt_template_no_variables() {
    let template = PromptTemplate::new("Static text", vec![]);
    let result = template.format(HashMap::new());
    assert_eq!(result, "Static text");
}

#[test]
fn test_prompt_template_extra_input_ignored() {
    let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World"), ("extra", "Ignored")]));
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_prompt_template_repeated_variable() {
    let template = PromptTemplate::new("{a} and {a} again", vec!["a"]);
    let result = template.format(HashMap::from([("a", "test")]));
    assert_eq!(result, "test and test again");
}

#[test]
fn test_prompt_template_complex_template() {
    let template =
        PromptTemplate::from_template("You are {role}. Your task is {task}. Context: {context}");
    assert_eq!(template.input_variables().len(), 3);

    let result = template.format(HashMap::from([
        ("role", "assistant"),
        ("task", "help users"),
        ("context", "a chat app"),
    ]));
    assert!(result.contains("assistant"));
    assert!(result.contains("help users"));
    assert!(result.contains("a chat app"));
}

#[test]
fn test_prompt_template_input_variables_method() {
    let template = PromptTemplate::new("{x} {y} {z}", vec!["x", "y", "z"]);
    let vars = template.input_variables();
    assert_eq!(vars.len(), 3);
    assert!(vars.contains(&"x".to_string()));
    assert!(vars.contains(&"y".to_string()));
    assert!(vars.contains(&"z".to_string()));
}

#[test]
fn test_prompt_template_template_method() {
    let template = PromptTemplate::new("Test template", vec![]);
    assert_eq!(template.template(), "Test template");
}
