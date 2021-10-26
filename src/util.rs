use std::{collections::HashMap, io::{self, prelude::*}};

use crate::seichi_api::Rankings;

pub fn pause() {
    print!("Press Enter key to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}

pub async fn deserialize(response: reqwest::Response) -> Result<Rankings, Box<(dyn std::error::Error)>> {
    Ok(response.json::<Rankings>().await?)
}

pub async fn fetch(
    url: &str,
    query: Option<HashMap<String, String>>,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;
    Ok(response)
}
