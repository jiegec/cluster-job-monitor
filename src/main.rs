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
use clap::Parser;
use cluster_job_monitor::{
    job::Job, pbs::parse_pbs_stat, slack::notify_slack, slurm::parse_slurm_stat,
};
use dotenv::dotenv;
use env_logger;
use jfs::Store;
use log::*;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::{io::Read, process::Command};
use timeago::Formatter;
use tokio;
use tokio::time::sleep;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(short, long, default_value = "history.log")]
    history: PathBuf,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "config")]
enum Scheduler {
    PBS(String),
    Slurm(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "config")]
enum Notification {
    Slack(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    scheduler: Scheduler,
    name: String,
    interval: u64,
    notification: Option<Notification>,
}

#[derive(Serialize, Debug)]
struct HistoryEntry {
    now: DateTime<Utc>,
    update_time: Option<DateTime<Utc>>,
    action: String,
    name: String,
    owner: String,
    id: String,
    state: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let args = Args::parse();
    let mut file = File::open(&args.config)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = toml::from_str(&content)?;
    info!("Cluster job monitor is up with config {:?}", config);

    let mut history = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&args.history)?;

    let db = Store::new("cluster-job-monitor-store").unwrap();
    let mut last_jobs = db.get::<Vec<Job>>("jobs").unwrap_or(Vec::new());
    loop {
        info!("Querying scheduler");
        let mut jobs = match &config.scheduler {
            Scheduler::PBS(cmd) => {
                let output = Command::new("sh").arg("-c").arg(cmd).output()?.stdout;
                let content = String::from_utf8(output).expect("valid utf8");
                parse_pbs_stat(&content)
            }
            Scheduler::Slurm(cmd) => {
                let output = Command::new("sh").arg("-c").arg(cmd).output()?.stdout;
                let content = String::from_utf8(output).expect("valid utf8");
                parse_slurm_stat(&content)
            }
        };
        jobs.sort();
        debug!("Got {:?}", jobs);

        if jobs != last_jobs {
            info!("Jobs changed");

            let mut msg = format!(
                "Cluster {} job changes (current {} jobs):\n",
                config.name,
                jobs.len()
            );
            let now = Utc::now();

            // added jobs
            for job in &jobs {
                if !last_jobs.iter().any(|j| j.id == job.id) {
                    let entry = HistoryEntry {
                        now: Utc::now(),
                        update_time: None,
                        action: "add".to_string(),
                        name: job.name.clone(),
                        owner: job.owner.clone(),
                        id: job.id.clone(),
                        state: format!("{:?}", job.state),
                    };
                    writeln!(history, "{}", serde_json::to_string(&entry)?)?;

                    msg.push_str(&format!(
                        ":heavy_plus_sign: : name *{}* owner *{}* id *{}* state *{}*\n",
                        job.name, job.owner, job.id, job.state
                    ));
                }
            }

            // removed jobs
            for job in &last_jobs {
                if !jobs.iter().any(|j| j.id == job.id) {
                    let entry = HistoryEntry {
                        now: Utc::now(),
                        update_time: Some(job.update_time),
                        action: "removed".to_string(),
                        name: job.name.clone(),
                        owner: job.owner.clone(),
                        id: job.id.clone(),
                        state: format!("{:?}", job.state),
                    };
                    writeln!(history, "{}", serde_json::to_string(&entry)?)?;

                    msg.push_str(&format!(
                        ":heavy_minus_sign: : name *{}* owner *{}* id *{}* state *{}* last update *{}*\n",
                        job.name,
                        job.owner,
                        job.id,
                        job.state,
                        Formatter::new().convert_chrono(job.update_time, now)
                    ));
                }
            }

            // state changed
            for job in &mut jobs {
                if let Some(old_job) = last_jobs.iter().find(|j| j.id == job.id) {
                    if old_job.state != job.state {
                        let entry = HistoryEntry {
                            now: Utc::now(),
                            update_time: Some(job.update_time),
                            action: "changed".to_string(),
                            name: job.name.clone(),
                            owner: job.owner.clone(),
                            id: job.id.clone(),
                            state: format!("{:?}", job.state),
                        };
                        writeln!(history, "{}", serde_json::to_string(&entry)?)?;

                        msg.push_str(&format!(
                            ":heavy_check_mark: : name *{}* owner *{}* id *{}* state changed: *{}* -> *{}* last update *{}*\n",
                            job.name, job.owner, job.id, old_job.state, job.state, Formatter::new().convert_chrono(old_job.update_time, now)
                        ));
                    } else {
                        job.update_time = old_job.update_time;
                    }
                }
            }

            match &config.notification {
                Some(Notification::Slack(url)) => {
                    notify_slack(url, &msg).await;
                }
                _ => {}
            }

            db.save_with_id(&jobs, "jobs").unwrap();
            last_jobs = jobs;
        } else {
            info!("Jobs unchanged");
        }

        // add some randomness to sleep time
        let mut rng = rand::thread_rng();
        let scale: f64 = 0.9 + rng.gen::<f64>() * 0.2;
        let millis = (config.interval as f64 * scale * 1000.0) as u64;
        sleep(Duration::from_millis(millis)).await;
    }
}
