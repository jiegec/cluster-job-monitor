use chrono::Utc;
use cluster_job_monitor::{
    job::Job, pbs::parse_pbs_stat, slack::notify_slack, slurm::parse_slurm_stat,
};
use dotenv::dotenv;
use env_logger;
use jfs::Store;
use log::*;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::{io::Read, process::Command};
use structopt::StructOpt;
use timeago::Formatter;
use tokio;
use tokio::time::sleep;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    config: PathBuf,
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let args = Args::from_args();
    let mut file = File::open(&args.config)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = toml::from_str(&content)?;
    info!("Cluster job monitor is up with config {:?}", config);
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
                    msg.push_str(&format!(
                        ":heavy_plus_sign: : name *{}* owner *{}* id *{}* state *{:?}*\n",
                        job.name, job.owner, job.id, job.state
                    ));
                }
            }

            // removed jobs
            for job in &last_jobs {
                if !jobs.iter().any(|j| j.id == job.id) {
                    msg.push_str(&format!(
                        ":heavy_minus_sign: : name *{}* owner *{}* id *{}* state *{:?}* last update *{}*\n",
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
                        msg.push_str(&format!(
                            ":heavy_check_mark: : name *{}* owner *{}* id *{}* state changed: *{:?}* -> *{:?}* last update *{}*\n",
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
