use crate::job::{Job, JobState};

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
                name = column[10..].to_string();
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
        });
    }
    res
}
