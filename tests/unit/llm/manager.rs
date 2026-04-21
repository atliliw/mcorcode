//! ModelManager 单元测试

use mcorcode::ModelManager;

/// 测试 ModelManager 的创建
/// 新管理器应有空模型列表和默认模型名
#[test]
fn test_model_manager_new() {
    let manager = ModelManager::new();
    assert!(manager.list_models().is_empty());
    assert_eq!(manager.default_model(), "gpt-4");
}

/// 测试 ModelManager 的 default 实现
/// default 应与 new() 行为一致
#[test]
fn test_model_manager_default() {
    let manager = ModelManager::default();
    assert!(manager.list_models().is_empty());
}

/// 测试 set_default_model 方法
/// 默认模型名应可修改
#[test]
fn test_model_manager_set_default_model() {
    let mut manager = ModelManager::new();
    manager.set_default_model("gpt-3.5-turbo");
    assert_eq!(manager.default_model(), "gpt-3.5-turbo");
}

/// 测试 set_default_model 设置其他模型
/// 可设置任意字符串作为默认模型名
#[test]
fn test_model_manager_set_default_model_another() {
    let mut manager = ModelManager::new();
    manager.set_default_model("claude-3-sonnet");
    assert_eq!(manager.default_model(), "claude-3-sonnet");
}

/// 测试 get_client 方法在空管理器时的行为
/// 无注册模型时应返回 None
#[test]
fn test_model_manager_get_client_empty() {
    let manager = ModelManager::new();
    assert!(manager.get_client(None).is_none());
    assert!(manager.get_client(Some("gpt-4")).is_none());
    assert!(manager.get_client(Some("unknown-model")).is_none());
}

/// 测试 list_models 方法在空管理器时
/// 应返回空列表
#[test]
fn test_model_manager_list_models_empty() {
    let manager = ModelManager::new();
    let models = manager.list_models();
    assert!(models.is_empty());
}

/// 测试 default_model 返回字符串
/// 默认模型名应以 "gpt" 开头
#[test]
fn test_model_manager_default_model_returns_string() {
    let manager = ModelManager::new();
    let model = manager.default_model();
    assert!(model.starts_with("gpt"));
}
