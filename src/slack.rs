use log::*;
use std::collections::HashMap;

pub async fn notify_slack(url: &str, msg: &str) {
    let mut map = HashMap::new();
    map.insert("text", msg);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&map)
        .send()
        .await
        .expect("Send message to slack");
    info!("Message posted to slack with code {:?}", res.status());
}
