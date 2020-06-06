use dotenv::dotenv;
use env_logger;
use std::time::Duration;
use tokio;
use tokio::time::delay_for;
use log::*;

async fn timer() {
    loop {
        delay_for(Duration::from_millis(1000 * 60 * 30)).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Cluster job monitor is up");
    timer().await;
}
