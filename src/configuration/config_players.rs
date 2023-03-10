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
            "2" => PlayerType::Computer1,
            "h" => PlayerType::Human,
            _ => return Err(invalid_player),
        };

        let p2type = match p2 {
            "1" => PlayerType::Computer1,
            "2" => PlayerType::Computer1,
            "h" => PlayerType::Human,
            _ => return Err(invalid_player),
        };

        Ok(ConfigPlayers { p1type, p2type })
    }
}
