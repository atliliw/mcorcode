//! Unit tests for ModelManager

use mcorcode::ModelManager;

#[test]
fn test_model_manager_new() {
    let manager = ModelManager::new();
    assert!(manager.list_models().is_empty());
    assert_eq!(manager.default_model(), "gpt-4");
}

#[test]
fn test_model_manager_default() {
    let manager = ModelManager::default();
    assert!(manager.list_models().is_empty());
}

#[test]
fn test_model_manager_set_default_model() {
    let mut manager = ModelManager::new();
    manager.set_default_model("gpt-3.5-turbo");
    assert_eq!(manager.default_model(), "gpt-3.5-turbo");
}

#[test]
fn test_model_manager_set_default_model_another() {
    let mut manager = ModelManager::new();
    manager.set_default_model("claude-3-sonnet");
    assert_eq!(manager.default_model(), "claude-3-sonnet");
}

#[test]
fn test_model_manager_get_client_empty() {
    let manager = ModelManager::new();
    assert!(manager.get_client(None).is_none());
    assert!(manager.get_client(Some("gpt-4")).is_none());
    assert!(manager.get_client(Some("unknown-model")).is_none());
}

#[test]
fn test_model_manager_list_models_empty() {
    let manager = ModelManager::new();
    let models = manager.list_models();
    assert!(models.is_empty());
}

#[test]
fn test_model_manager_default_model_returns_string() {
    let manager = ModelManager::new();
    let model = manager.default_model();
    assert!(model.starts_with("gpt"));
}
