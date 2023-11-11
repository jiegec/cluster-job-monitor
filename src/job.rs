// Copyright (C) 2020-2023 Jiajie Chen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
