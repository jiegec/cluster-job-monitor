use cluster_job_monitor::{job::Job, pbs::parse_pbs_stat, slack::notify_slack};
use dotenv::dotenv;
use env_logger;
use jfs::Store;
use log::*;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::{io::Read, process::Command};
use structopt::StructOpt;
use tokio;
use tokio::time::delay_for;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    config: PathBuf,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "config")]
enum Scheduler {
    PBS(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "config")]
enum Notification {
    Slack(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    scheduler: Scheduler,
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
    let db = Store::new("cluster-job-montor").unwrap();
    let mut last_jobs = db.get::<Vec<Job>>("jobs").unwrap_or(Vec::new());
    loop {
        info!("Querying scheduler");
        let jobs = match &config.scheduler {
            Scheduler::PBS(cmd) => {
                let output = Command::new("sh").arg("-c").arg(cmd).output()?.stdout;
                let json = String::from_utf8(output).expect("valid utf8");
                parse_pbs_stat(&json)
            }
        };

        if jobs != last_jobs {
            info!("Jobs changed");

            let mut msg = String::new();
            // TODO: meaningful message
            msg = format!("{:?}", jobs);

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
        delay_for(Duration::from_millis(config.interval * 1000)).await;
    }
}
