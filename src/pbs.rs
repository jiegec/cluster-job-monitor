use crate::job::{Job, JobState};
use json;
use log::*;

pub fn parse_pbs_stat(json: &str) -> Vec<Job> {
    let mut res = vec![];
    if let Ok(root) = json::parse(json) {
        debug!("PBS version {:?}", root["pbs_version"]);
        match &root["Jobs"] {
            json::JsonValue::Object(jobs) => {
                for (id, job) in jobs.iter() {
                    let state = job["Job_state"].dump();
                    let job_state = if state == "Q" {
                        JobState::Queuing
                    } else if state == "R" {
                        JobState::Running
                    } else {
                        JobState::Unknown
                    };
                    res.push(Job {
                        id: String::from(id),
                        name: job["Job_Name"].dump(),
                        owner: job["Job_Owner"].dump(),
                        state: job_state,
                    });
                }
            }
            _ => {}
        }
    }
    res
}
