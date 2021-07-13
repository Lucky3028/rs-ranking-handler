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

#[derive(Clone, Debug)]
pub struct Lottery {
    pub ranking_type: String,
    pub rank: u32,
    pub amount: String,
    pub player_name: String,
}

impl Lottery {
    fn new(
        ranking_type: impl Into<String>,
        rank: u32,
        amount: impl Into<String>,
        player_name: impl Into<String>,
    ) -> Lottery {
        Lottery {
            ranking_type: ranking_type.into(),
            rank,
            amount: amount.into(),
            player_name: player_name.into(),
        }
    }

    pub fn convert(rank: &Rank) -> Lottery {
        Lottery::new(&rank.r_type, rank.rank, &rank.amount.raw_data, &rank.player.name)
    }
}
