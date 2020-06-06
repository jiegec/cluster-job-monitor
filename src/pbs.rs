use crate::job::{Job, JobState};
use json;
use log::*;

pub fn parse_pbs_stat(json: &str) -> Vec<Job> {
    let mut res = vec![];
    if let Ok(root) = json::parse(json) {
        info!("PBS version {:?}", root["pbs_version"]);
        match &root["Jobs"] {
            json::JsonValue::Object(jobs) => {
                for (id, job) in jobs.iter() {
                    res.push(Job {
                        id: String::from(id),
                        name: job["Job_Name"].dump(),
                        owner: job["Job_Owner"].dump(),
                        state: JobState::Queuing
                    });
                }
            }
            _ => {}
        }
    }
    res
}