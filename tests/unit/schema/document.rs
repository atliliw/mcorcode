//! Document 类型单元测试

use mcorcode::Document;

/// 测试 Document 的基本创建
/// 验证 content 字段正确设置，metadata 初始化为空对象
#[test]
fn test_document_creation() {
    let doc = Document::new("Test content");
    assert_eq!(doc.content, "Test content");
    assert!(doc.metadata.is_object());
}

/// 测试带元数据的 Document 创建
/// 验证 content 和 metadata 同时正确设置
#[test]
fn test_document_with_metadata() {
    let doc = Document::with_metadata(
        "Test content",
        serde_json::json!({"source": "test.txt", "page": 1}),
    );
    assert_eq!(doc.content, "Test content");
    assert_eq!(doc.metadata["source"], "test.txt");
    assert_eq!(doc.metadata["page"], 1);
}

/// 测试 From<String> trait 实现
/// 可直接从 String 创建 Document
#[test]
fn test_document_from_string() {
    let doc: Document = "Simple content".to_string().into();
    assert_eq!(doc.content, "Simple content");
}

/// 测试 From<&str> trait 实现
/// 可直接从字符串字面量创建 Document
#[test]
fn test_document_from_str() {
    let doc: Document = "Content from str".into();
    assert_eq!(doc.content, "Content from str");
}

/// 测试 add_metadata 方法
/// 可向已存在的 Document 添加键值对
#[test]
fn test_document_add_metadata() {
    let mut doc = Document::new("Test");
    doc.add_metadata("author", serde_json::json!("John"));
    doc.add_metadata("date", serde_json::json!("2024-01-01"));
    assert_eq!(doc.metadata["author"], "John");
    assert_eq!(doc.metadata["date"], "2024-01-01");
}

/// 测试 Document 的 JSON 序列化
/// 验证 content 和 metadata 都正确序列化
#[test]
fn test_document_serialization() {
    let doc = Document::with_metadata("Content", serde_json::json!({"key": "value"}));
    let json = serde_json::to_string(&doc).unwrap();
    assert!(json.contains("\"content\":\"Content\""));
    assert!(json.contains("\"metadata\""));
}

/// 测试 Document 的 JSON 反序列化
/// 验证可从 JSON 字符串解析 Document
#[test]
fn test_document_deserialization() {
    let json = "{\"content\":\"Test\",\"metadata\":{\"source\":\"file.txt\"}}";
    let doc: Document = serde_json::from_str(json).unwrap();
    assert_eq!(doc.content, "Test");
    assert_eq!(doc.metadata["source"], "file.txt");
}
