use std::io::{self, prelude::*};

use crate::seichi_api::Rankings;

pub fn pause() {
    print!("Press Enter key to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}

pub async fn deserialize(response: reqwest::Response) -> Result<Rankings, Box<(dyn std::error::Error)>> {
    Ok(response.json::<Rankings>().await?)
}
