use super::state::SessionState;
use crate::memory::ChatMessageHistory;
use crate::schema::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct SessionManager {
    sessions: HashMap<String, SessionState>,
    histories: HashMap<String, ChatMessageHistory>,
    storage_path: PathBuf,
    auto_save: bool,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            histories: HashMap::new(),
            storage_path: PathBuf::from(".mcorcode/sessions"),
            auto_save: false,
        }
    }

    pub fn with_storage_path(path: impl Into<PathBuf>) -> Self {
        Self {
            sessions: HashMap::new(),
            histories: HashMap::new(),
            storage_path: path.into(),
            auto_save: false,
        }
    }

    pub fn with_auto_save(mut self, enabled: bool) -> Self {
        self.auto_save = enabled;
        self
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

            if self.auto_save {
                self.save_session(session_id)?;
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

    pub fn save_session(&self, id: &str) -> Result<()> {
        fs::create_dir_all(&self.storage_path)?;

        let state = self
            .sessions
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("Session '{}' not found", id))?;
        let history = self
            .histories
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("History for session '{}' not found", id))?;

        let state_file = self.storage_path.join(format!("{}.state.json", id));
        let history_file = self.storage_path.join(format!("{}.history.json", id));

        let state_json = serde_json::to_string_pretty(state)?;
        let history_json = serde_json::to_string_pretty(&history.messages())?;

        fs::write(&state_file, state_json)?;
        fs::write(&history_file, history_json)?;

        Ok(())
    }

    pub fn load_session(&mut self, id: &str) -> Result<()> {
        let state_file = self.storage_path.join(format!("{}.state.json", id));
        let history_file = self.storage_path.join(format!("{}.history.json", id));

        let state_json = fs::read_to_string(&state_file)?;
        let history_json = fs::read_to_string(&history_file)?;

        let state: SessionState = serde_json::from_str(&state_json)?;
        let messages: Vec<Message> = serde_json::from_str(&history_json)?;

        let mut history = ChatMessageHistory::new();
        for msg in messages {
            history.add(msg);
        }

        self.sessions.insert(id.to_string(), state);
        self.histories.insert(id.to_string(), history);

        Ok(())
    }

    pub fn list_saved_sessions(&self) -> Result<Vec<String>> {
        if !self.storage_path.exists() {
            return Ok(Vec::new());
        }

        let mut session_ids = Vec::new();
        for entry in fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".state.json") {
                let id = filename.replace(".state.json", "");
                session_ids.push(id);
            }
        }

        Ok(session_ids)
    }

    pub fn delete_session_file(&self, id: &str) -> Result<()> {
        let state_file = self.storage_path.join(format!("{}.state.json", id));
        let history_file = self.storage_path.join(format!("{}.history.json", id));

        if state_file.exists() {
            fs::remove_file(&state_file)?;
        }
        if history_file.exists() {
            fs::remove_file(&history_file)?;
        }

        Ok(())
    }

    pub fn save_all(&self) -> Result<()> {
        for id in self.sessions.keys() {
            self.save_session(id)?;
        }
        Ok(())
    }

    pub fn session_exists(&self, id: &str) -> bool {
        self.sessions.contains_key(id)
    }

    pub fn session_file_exists(&self, id: &str) -> bool {
        self.storage_path
            .join(format!("{}.state.json", id))
            .exists()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
