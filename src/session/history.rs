use crate::schema::Message;
use chrono::{DateTime, Utc};
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
            timestamp: Utc::now(),
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

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.content.contains(query))
            .collect()
    }

    pub fn filter_by_role(&self, role: &str) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.role == role).collect()
    }

    pub fn filter_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    pub fn last_n(&self, n: usize) -> Vec<&HistoryEntry> {
        let start = self.entries.len().saturating_sub(n);
        self.entries[start..].iter().collect()
    }

    pub fn first_n(&self, n: usize) -> Vec<&HistoryEntry> {
        let end = self.entries.len().min(n);
        self.entries[..end].iter().collect()
    }
}

impl Default for SessionHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub role: String,
    pub content: String,
}
