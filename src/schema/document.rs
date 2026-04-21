//! Document structure for retrieval and storage
//!
//! Reference: langchainrust/langchainrust/src/schema/document.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Document structure for RAG and storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document content
    pub content: String,

    /// Document metadata
    #[serde(default)]
    pub metadata: serde_json::Value,

    /// 文档唯一 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 来源路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// 页码（用于 PDF 等分页文档）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,

    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl Document {
    /// Create a new document with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            metadata: serde_json::json!({}),
            id: None,
            source: None,
            page: None,
            created_at: Some(Utc::now()),
        }
    }

    /// Create a document with content and metadata
    pub fn with_metadata(content: impl Into<String>, metadata: serde_json::Value) -> Self {
        Self {
            content: content.into(),
            metadata,
            id: None,
            source: None,
            page: None,
            created_at: Some(Utc::now()),
        }
    }

    /// 设置文档 ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// 设置来源路径
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 设置页码
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// Add metadata field
    pub fn add_metadata(&mut self, key: impl Into<String>, value: serde_json::Value) {
        if let serde_json::Value::Object(ref mut map) = self.metadata {
            map.insert(key.into(), value);
        }
    }

    /// 将文档分割为多个小块
    pub fn split(&self, chunk_size: usize) -> Vec<Document> {
        if chunk_size == 0 || self.content.is_empty() {
            return vec![self.clone()];
        }

        let content = &self.content;
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < content.len() {
            let end = std::cmp::min(start + chunk_size, content.len());
            let chunk_content = content[start..end].to_string();

            let chunk = Document {
                content: chunk_content,
                metadata: self.metadata.clone(),
                id: None,
                source: self.source.clone(),
                page: self.page,
                created_at: self.created_at,
            };
            chunks.push(chunk);

            start = end;
        }

        chunks
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
