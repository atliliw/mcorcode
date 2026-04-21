//! PromptTemplate 单元测试

use mcorcode::prompts::PromptTemplate;
use std::collections::HashMap;

/// 测试带显式变量的 PromptTemplate 创建
/// 输入变量列表应精确保留
#[test]
fn test_prompt_template_new() {
    let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
    assert_eq!(template.input_variables(), &["name".to_string()]);
    assert_eq!(template.template(), "Hello {name}!");
}

/// 测试从模板自动提取变量
/// 模板中的变量应自动检测
#[test]
fn test_prompt_template_from_template() {
    let template = PromptTemplate::from_template("Hello {name}, you are {role}!");
    assert_eq!(template.input_variables().len(), 2);
    assert!(template.input_variables().contains(&"name".to_string()));
    assert!(template.input_variables().contains(&"role".to_string()));
}

/// 测试单变量模板格式化
/// 占位符应替换为提供的值
#[test]
fn test_prompt_template_format_single_variable() {
    let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World")]));
    assert_eq!(result, "Hello World!");
}

/// 测试多变量模板格式化
/// 所有占位符应替换为各自值
#[test]
fn test_prompt_template_format_multiple_variables() {
    let template = PromptTemplate::new("{greeting} {name}!", vec!["greeting", "name"]);
    let result = template.format(HashMap::from([("greeting", "Hello"), ("name", "World")]));
    assert_eq!(result, "Hello World!");
}

/// 测试输入缺少变量时的行为
/// 缺少的变量占位符应保持不变
#[test]
fn test_prompt_template_missing_variable_keeps_placeholder() {
    let template = PromptTemplate::new("Hello {name} {missing}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World")]));
    assert_eq!(result, "Hello World {missing}!");
}

/// 测试空模板处理
/// 空模板应格式化为空字符串
#[test]
fn test_prompt_template_empty_template() {
    let template = PromptTemplate::new("", vec![]);
    assert_eq!(template.template(), "");
    assert!(template.input_variables().is_empty());
}

/// 测试 PromptTemplate 的 default 实现
/// default 应产生空模板
#[test]
fn test_prompt_template_default() {
    let template = PromptTemplate::default();
    assert_eq!(template.template(), "");
    assert!(template.input_variables().is_empty());
}

/// 测试无变量模板
/// 静态文本格式化后应保持不变
#[test]
fn test_prompt_template_no_variables() {
    let template = PromptTemplate::new("Static text", vec![]);
    let result = template.format(HashMap::new());
    assert_eq!(result, "Static text");
}

/// 测试忽略多余输入
/// 不在变量列表中的输入不应影响输出
#[test]
fn test_prompt_template_extra_input_ignored() {
    let template = PromptTemplate::new("Hello {name}!", vec!["name"]);
    let result = template.format(HashMap::from([("name", "World"), ("extra", "Ignored")]));
    assert_eq!(result, "Hello World!");
}

/// 测试模板中重复变量
/// 同一变量出现多次应全部替换
#[test]
fn test_prompt_template_repeated_variable() {
    let template = PromptTemplate::new("{a} and {a} again", vec!["a"]);
    let result = template.format(HashMap::from([("a", "test")]));
    assert_eq!(result, "test and test again");
}

/// 测试多变量复杂模板
/// 所有变量应按正确顺序替换
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

/// 测试 input_variables() 访问器方法
/// 应返回声明的变量列表
#[test]
fn test_prompt_template_input_variables_method() {
    let template = PromptTemplate::new("{x} {y} {z}", vec!["x", "y", "z"]);
    let vars = template.input_variables();
    assert_eq!(vars.len(), 3);
    assert!(vars.contains(&"x".to_string()));
    assert!(vars.contains(&"y".to_string()));
    assert!(vars.contains(&"z".to_string()));
}

/// 测试 template() 访问器方法
/// 应返回原始模板字符串
#[test]
fn test_prompt_template_template_method() {
    let template = PromptTemplate::new("Test template", vec![]);
    assert_eq!(template.template(), "Test template");
}
