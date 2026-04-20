use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct SessionState {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub active: bool,
}

impl SessionState {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            message_count: 0,
            active: true,
        }
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        let mut state = Self::new();
        state.id = id.into();
        state
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn increment_messages(&mut self) {
        self.message_count += 1;
        self.touch();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.touch();
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
