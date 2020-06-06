use dotenv::dotenv;
use env_logger;
use std::time::Duration;
use tokio;
use tokio::time::delay_for;
use log::*;
use structopt::StructOpt;
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::path::PathBuf;
use std::{process::Command, io::Read};
use cluster_job_monitor;

#[derive(StructOpt)]
struct Args {
	#[structopt(short, long)]
	config: PathBuf,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "config")]
enum Scheduler {
	PBS(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum Notification {
	Slack(String)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
	scheduler: Scheduler,
	interval: u64,
	notification: Option<Notification>
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
	loop {
		info!("Querying scheduler");
		match &config.scheduler {
			Scheduler::PBS(cmd) => {
				let output = Command::new("sh").arg("-c").arg(cmd).output()?.stdout;
				let json = String::from_utf8(output).expect("valid utf8");
				let jobs = cluster_job_monitor::pbs::parse_pbs_stat(&json);
				println!("{:?}", jobs);
			}
		}
		delay_for(Duration::from_millis(config.interval * 1000)).await;
	}
}
