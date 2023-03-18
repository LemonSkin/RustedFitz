use crate::{FitzError, PlayerType};

#[derive(Debug, PartialEq)]
pub struct ConfigPlayers {
    pub p1type: PlayerType,
    pub p2type: PlayerType,
}

impl ConfigPlayers {
    pub fn build(p1: &str, p2: &str) -> Result<ConfigPlayers, FitzError> {
        let invalid_player = FitzError {
            code: 4,
            message: "Invalid player type".to_string(),
        };

        let p1type = match p1 {
            "1" => PlayerType::Computer1,
            "2" => PlayerType::Computer2,
            "h" => PlayerType::Human,
            _ => return Err(invalid_player),
        };

        let p2type = match p2 {
            "1" => PlayerType::Computer1,
            "2" => PlayerType::Computer2,
            "h" => PlayerType::Human,
            _ => return Err(invalid_player),
        };

        Ok(ConfigPlayers { p1type, p2type })
    }
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_1_1_valid_chars() {
        let expected: ConfigPlayers = ConfigPlayers {
            p1type: PlayerType::Human,
            p2type: PlayerType::Human,
        };
        assert_eq!(ConfigPlayers::build("h", "h"), Ok(expected));
    }

    #[test]
    fn test_1_2_valid_chars() {
        let expected: ConfigPlayers = ConfigPlayers {
            p1type: PlayerType::Computer1,
            p2type: PlayerType::Computer1,
        };
        assert_eq!(ConfigPlayers::build("1", "1"), Ok(expected));
    }

    #[test]
    fn test_1_3_valid_chars() {
        let expected: ConfigPlayers = ConfigPlayers {
            p1type: PlayerType::Computer2,
            p2type: PlayerType::Computer2,
        };
        assert_eq!(ConfigPlayers::build("2", "2"), Ok(expected));
    }

    #[test]
    fn test_2_1_invalid_chars() {
        let expected = FitzError {
            code: 4,
            message: "Invalid player type".to_string(),
        };

        assert_eq!(ConfigPlayers::build("g", "h"), Err(expected));
    }

    #[test]
    fn test_2_2_invalid_chars() {
        let expected = FitzError {
            code: 4,
            message: "Invalid player type".to_string(),
        };

        assert_eq!(ConfigPlayers::build("1", "6"), Err(expected));
    }
}
