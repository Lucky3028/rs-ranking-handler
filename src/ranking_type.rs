pub enum RankingType {
    Break,
    Build,
}

impl RankingType {
    pub fn as_str(&self) -> String {
        match self {
            RankingType::Break => "break",
            RankingType::Build => "build"
        }.to_string()
    }

    pub fn get_targets(&self) -> u8 {
        match self {
            RankingType::Break => 20,
            RankingType::Build => 10
        }
    }

    pub fn get_winners(&self) -> u8 {
        match self {
            RankingType::Break => 5,
            RankingType::Build => 3
        }
    }
}
