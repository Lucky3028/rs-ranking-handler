use serde::Deserialize;

/***
○○量を表す
 */
#[derive(Debug, Deserialize)]
struct Amount {
    raw_data: String,
}

/***
MCIDを表す
 */
#[derive(Debug, Deserialize)]
struct PlayerIdentifier {
    name: String,
}

/***
各プレイヤーの個別データを表す
 */
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

/***
APIからの直接の返り値の型
 */
#[derive(Debug, Deserialize)]
pub struct Rankings {
    result_count: u32,
    pub ranks: Vec<Rank>,
    total_ranked_player: u32,
}

/***
本プログラム内で必要なものだけを詰め直すDTO
 */
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
