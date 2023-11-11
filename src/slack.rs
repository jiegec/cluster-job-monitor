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
