use crate::job::{Job, JobState};
use chrono::Utc;
use std::collections::HashMap;

pub fn parse_slurm_stat(dsv: &str) -> Vec<Job> {
    let mut res = vec![];
    let mut index_map: HashMap<String, usize> = HashMap::new();
    for line in dsv.split("\n") {
        if line.starts_with("ACCOUNT|") {
            // header
            for (index, column) in line.split("|").enumerate() {
                index_map.insert(column.to_string(), index);
            }

            continue;
        } else if !line.contains("|") {
            continue;
        }

        let mut state = JobState::Unknown;
        let columns: Vec<&str> = line.split("|").collect();
        let id = columns[index_map["JOBID"]].trim().to_string();
        let name = columns[index_map["NAME"]].trim().to_string();
        let owner = columns[index_map["USER"]].trim().to_string();
        let raw_state = columns[index_map["STATE "]].trim().to_string();
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
