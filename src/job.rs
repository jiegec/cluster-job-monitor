use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum JobState {
    Running,
    Queuing,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub state: JobState,
}
