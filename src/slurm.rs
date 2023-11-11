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
use crate::job::{Job, JobState};
use chrono::Utc;

pub fn parse_slurm_stat(dsv: &str) -> Vec<Job> {
    let mut res = vec![];
    for line in dsv.split("\n") {
        if !line.contains(" ") || line.starts_with("JOBID") {
            continue;
        }

        let mut state = JobState::Unknown;
        let columns: Vec<&str> = line.split(" ").filter(|s| s.len() > 0).collect();
        let id = columns[0].trim().to_string();
        let name = columns[1].trim().to_string();
        let owner = columns[2].trim().to_string();
        let raw_state = columns[3].trim().to_string();
        if raw_state == "PENDING" {
            state = JobState::Queuing;
        } else if raw_state == "RUNNING" {
            state = JobState::Running;
        } else if raw_state == "SUSPENDED" {
            state = JobState::Suspended;
        } else if raw_state == "COMPLETING" {
            state = JobState::Completing;
        } else if raw_state == "COMPLETED" {
            state = JobState::Completed;
        }
        res.push(Job {
            id,
            name,
            owner,
            state,
            update_time: Utc::now(),
        });
    }
    res
}
