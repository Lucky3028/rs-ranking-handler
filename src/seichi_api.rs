use serde::Deserialize;
use std::error;

#[derive(Debug, Deserialize)]
struct Amount {
    raw_data: String,
}

#[derive(Debug, Deserialize)]
struct PlayerIdentifier {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    rank: u32,
    #[serde(rename = "type")]
    r_type: String,
    #[serde(rename = "data")]
    amount: Amount,
    lastquit: String,
    player: PlayerIdentifier,
}

#[derive(Debug, Deserialize)]
pub struct Rankings {
    result_count: u32,
    pub ranks: Vec<Rank>,
    total_ranked_player: u32,
}

pub async fn deserialize(response: reqwest::Response) -> Result<Rankings, Box<(dyn error::Error)>> {
    Ok(response.json::<Rankings>().await?)
}
