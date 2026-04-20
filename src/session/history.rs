use crate::schema::Message;
use serde::{Deserialize, Serialize};

pub struct SessionHistory {
    entries: Vec<HistoryEntry>,
}

impl SessionHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, role: &str, content: &str) {
        self.entries.push(HistoryEntry {
            timestamp: chrono::Utc::now(),
            role: role.to_string(),
            content: content.to_string(),
        });
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn to_messages(&self) -> Vec<Message> {
        self.entries
            .iter()
            .map(|e| match e.role.as_str() {
                "user" | "human" => Message::human(&e.content),
                "assistant" | "ai" => Message::ai(&e.content),
                "system" => Message::system(&e.content),
                _ => Message::human(&e.content),
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for SessionHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub role: String,
    pub content: String,
}
