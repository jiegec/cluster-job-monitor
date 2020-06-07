use crate::job::{Job, JobState};
use chrono::Utc;

pub fn parse_slurm_stat(dsv: &str) -> Vec<Job> {
    let mut res = vec![];
    for line in dsv.split("\n") {
        if !line.contains(" ") {
            continue;
        }
        let mut index = 0;
        let mut id = String::new();
        let mut name = String::new();
        let mut owner = String::new();
        let mut state = JobState::Unknown;
        for column in line.split(" ") {
            if column.len() == 0 {
                continue;
            }
            index += 1;
            if index == 1 {
                id = column.to_string();
            } else if index == 3 {
                name = column.to_string();
            } else if index == 4 {
                owner = column.to_string();
            } else if index == 5 && column == "PENDING" {
                state = JobState::Queuing;
            } else if index == 5 && column == "RUNNING" {
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
