use std::{convert::AsRef, str::FromStr};

use strum::{AsRefStr, EnumProperty};

#[derive(AsRefStr, EnumProperty)]
pub enum RankingType {
    #[strum(props(targets = "20", winners = "3"))]
    Break,
    #[strum(props(targets = "10", winners = "2"))]
    Build,
}

impl RankingType {
    pub fn as_str(&self) -> String {
        let str: &str = self.as_ref();
        str.to_lowercase()
    }

    pub fn get_targets(&self) -> u8 {
        self.get_str("targets").and_then(|str| u8::from_str(str).ok()).unwrap_or(0)
    }

    pub fn get_winners(&self) -> u8 {
        self.get_str("winners").and_then(|str| u8::from_str(str).ok()).unwrap_or(0)
    }
}
