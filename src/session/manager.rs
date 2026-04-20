use super::state::SessionState;
use crate::memory::ChatMessageHistory;
use crate::schema::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct SessionManager {
    sessions: HashMap<String, SessionState>,
    histories: HashMap<String, ChatMessageHistory>,
    storage_path: PathBuf,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            histories: HashMap::new(),
            storage_path: PathBuf::from(".mcorcode/sessions"),
        }
    }

    pub fn with_storage_path(path: impl Into<PathBuf>) -> Self {
        Self {
            sessions: HashMap::new(),
            histories: HashMap::new(),
            storage_path: path.into(),
        }
    }

    pub fn create_session(&mut self) -> String {
        let state = SessionState::new();
        let id = state.id.clone();

        self.sessions.insert(id.clone(), state);
        self.histories.insert(id.clone(), ChatMessageHistory::new());

        id
    }

    pub fn get_session(&self, id: &str) -> Option<&SessionState> {
        self.sessions.get(id)
    }

    pub fn add_message(&mut self, session_id: &str, message: Message) -> Result<()> {
        if let Some(history) = self.histories.get_mut(session_id) {
            history.add(message);

            if let Some(state) = self.sessions.get_mut(session_id) {
                state.increment_messages();
            }
        }
        Ok(())
    }

    pub fn get_messages(&self, session_id: &str) -> Option<&[Message]> {
        self.histories.get(session_id).map(|h| h.messages())
    }

    pub fn list_sessions(&self) -> Vec<&SessionState> {
        self.sessions.values().collect()
    }

    pub fn delete_session(&mut self, id: &str) {
        self.sessions.remove(id);
        self.histories.remove(id);
    }

    pub fn clear_all(&mut self) {
        self.sessions.clear();
        self.histories.clear();
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
