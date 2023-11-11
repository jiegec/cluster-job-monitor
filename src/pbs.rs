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

pub fn parse_pbs_stat(dsv: &str) -> Vec<Job> {
    let mut res = vec![];
    for line in dsv.split("\n") {
        if !line.contains("|") {
            continue;
        }
        let mut id = String::new();
        let mut name = String::new();
        let mut owner = String::new();
        let mut state = JobState::Unknown;
        for column in line.split("|") {
            if column.starts_with("Job Id:") {
                id = column[8..].to_string();
            } else if column.starts_with("Job_Name=") {
                name = column[9..].to_string();
            } else if column.starts_with("Job_Owner=") {
                owner = column[11..].to_string();
            } else if column == "job_state=Q" {
                state = JobState::Queuing;
            } else if column == "job_state=R" {
                state = JobState::Running;
            }
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
