use std::{convert::AsRef, str::FromStr};

use strum::{AsRefStr, EnumProperty, EnumIter};

#[derive(AsRefStr, EnumProperty, EnumIter, PartialEq)]
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

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use strum::IntoEnumIterator;

    use super::RankingType;

    #[test]
    fn test_count_and_contents() {
        assert_eq!(RankingType::iter().count(), 2);

        let rank_types: Vec<RankingType> = RankingType::iter().collect();
        assert!(rank_types == vec![RankingType::Break, RankingType::Build]);
    }

    #[test_case(RankingType::Build => "build")]
    #[test_case(RankingType::Break => "break")]
    fn test_as_str(rank_type: RankingType) -> String {
        rank_type.as_str()
    }

    #[test_case(RankingType::Build => 10)]
    #[test_case(RankingType::Break => 20)]
    fn test_targets(rank_type: RankingType) -> u8 {
        rank_type.get_targets()
    }

    #[test_case(RankingType::Build => 2)]
    #[test_case(RankingType::Break => 3)]
    fn test_winners(rank_type: RankingType) -> u8 {
        rank_type.get_winners()
    }
}
