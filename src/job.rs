use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum JobState {
    Running,
    Queuing,
    Unknown,
}

impl fmt::Debug for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobState::Running => write!(f, ":arrow_forward:"),
            JobState::Queuing => write!(f, ":double_vertical_bar:"),
            JobState::Unknown => write!(f, ":black_right_pointing_triangle:"),
        }
    }
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
