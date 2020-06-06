use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum JobState {
    Running,
    Queuing,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub state: JobState
}