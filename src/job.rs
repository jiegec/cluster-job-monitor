use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum JobState {
    Running,
    Queuing,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub state: JobState,
    pub update_time: DateTime<Utc>,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.owner == other.owner
            && self.state == other.state
    }
}

impl Eq for Job {}
