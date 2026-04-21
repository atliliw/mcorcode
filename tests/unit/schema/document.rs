//! Unit tests for Document type

use mcorcode::Document;

#[test]
fn test_document_creation() {
    let doc = Document::new("Test content");
    assert_eq!(doc.content, "Test content");
    assert!(doc.metadata.is_object());
}

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

#[test]
fn test_document_from_string() {
    let doc: Document = "Simple content".to_string().into();
    assert_eq!(doc.content, "Simple content");
}

#[test]
fn test_document_from_str() {
    let doc: Document = "Content from str".into();
    assert_eq!(doc.content, "Content from str");
}

#[test]
fn test_document_add_metadata() {
    let mut doc = Document::new("Test");
    doc.add_metadata("author", serde_json::json!("John"));
    doc.add_metadata("date", serde_json::json!("2024-01-01"));
    assert_eq!(doc.metadata["author"], "John");
    assert_eq!(doc.metadata["date"], "2024-01-01");
}

#[test]
fn test_document_serialization() {
    let doc = Document::with_metadata("Content", serde_json::json!({"key": "value"}));
    let json = serde_json::to_string(&doc).unwrap();
    assert!(json.contains("\"content\":\"Content\""));
    assert!(json.contains("\"metadata\""));
}

#[test]
fn test_document_deserialization() {
    let json = "{\"content\":\"Test\",\"metadata\":{\"source\":\"file.txt\"}}";
    let doc: Document = serde_json::from_str(json).unwrap();
    assert_eq!(doc.content, "Test");
    assert_eq!(doc.metadata["source"], "file.txt");
}
