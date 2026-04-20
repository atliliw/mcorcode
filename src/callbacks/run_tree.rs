use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum RunType {
    LLM,
    Tool,
    Chain,
    Agent,
}

pub struct RunTree {
    pub run_id: String,
    pub run_type: RunType,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub parent_run_id: Option<String>,
}

impl RunTree {
    pub fn new(run_type: RunType, name: impl Into<String>) -> Self {
        Self {
            run_id: Uuid::new_v4().to_string(),
            run_type,
            name: name.into(),
            start_time: Utc::now(),
            end_time: None,
            parent_run_id: None,
        }
    }

    pub fn with_parent(
        run_type: RunType,
        name: impl Into<String>,
        parent_run_id: impl Into<String>,
    ) -> Self {
        Self {
            run_id: Uuid::new_v4().to_string(),
            run_type,
            name: name.into(),
            start_time: Utc::now(),
            end_time: None,
            parent_run_id: Some(parent_run_id.into()),
        }
    }

    pub fn end(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn duration_ms(&self) -> Option<i64> {
        self.end_time
            .map(|end| (end - self.start_time).num_milliseconds())
    }
}
