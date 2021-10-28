use std::{collections::HashMap, io::{self, prelude::*}};

use anyhow::Context;

pub fn pause() {
    print!("Press Enter key to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}

pub async fn fetch(
    url: &str,
    query: Option<HashMap<String, String>>,
) -> anyhow::Result<reqwest::Response> {
    let response = reqwest::Client::new()
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await
        .context("APIとの通信中にエラーが発生しました")?;
    Ok(response)
}
