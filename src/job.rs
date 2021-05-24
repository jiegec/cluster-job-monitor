use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum JobState {
    Queuing,
    Running,
    Suspended,
    Completing,
    Completed,
    Unknown,
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobState::Queuing => write!(f, ":alarm_clock:"),
            JobState::Running => write!(f, ":arrow_forward:"),
            JobState::Suspended => write!(f, ":double_vertical_bar:"),
            JobState::Completed | JobState::Completing => write!(f, ":checkered_flag:"),
            JobState::Unknown => write!(f, ":question:"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd)]
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
