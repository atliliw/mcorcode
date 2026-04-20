//! Document structure for retrieval and storage
//!
//! Reference: langchainrust/langchainrust/src/schema/document.rs

use serde::{Deserialize, Serialize};

/// Document structure for RAG and storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document content
    pub content: String,

    /// Document metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl Document {
    /// Create a new document with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            metadata: serde_json::json!({}),
        }
    }

    /// Create a document with content and metadata
    pub fn with_metadata(content: impl Into<String>, metadata: serde_json::Value) -> Self {
        Self {
            content: content.into(),
            metadata,
        }
    }

    /// Add metadata field
    pub fn add_metadata(&mut self, key: impl Into<String>, value: serde_json::Value) {
        if let serde_json::Value::Object(ref mut map) = self.metadata {
            map.insert(key.into(), value);
        }
    }
}

impl From<String> for Document {
    fn from(content: String) -> Self {
        Document::new(content)
    }
}

impl From<&str> for Document {
    fn from(content: &str) -> Self {
        Document::new(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Test content");
        assert_eq!(doc.content, "Test content");
    }

    #[test]
    fn test_document_with_metadata() {
        let doc =
            Document::with_metadata("Test content", serde_json::json!({"source": "test.txt"}));
        assert_eq!(doc.metadata["source"], "test.txt");
    }
}
