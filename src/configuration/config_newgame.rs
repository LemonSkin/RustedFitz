use std::ops::Range;

use crate::configuration::config_players::ConfigPlayers;
use crate::FitzError;

#[derive(Debug, PartialEq)]
pub struct ConfigNew {
    pub players: ConfigPlayers,
    pub height: u16,
    pub width: u16,
}

impl ConfigNew {
    pub fn build(
        players: ConfigPlayers,
        height: &str,
        width: &str,
    ) -> Result<ConfigNew, FitzError> {
        let config_new: ConfigNew = ConfigNew {
            players,
            height: height.parse().unwrap(),
            width: width.parse().unwrap(),
        };

        // Ensure width and height are in valid range
        let valid_range: Range<u16> = 1..999;
        if !valid_range.contains(&config_new.height) || !valid_range.contains(&config_new.width) {
            return Err(FitzError {
                code: 5,
                message: "Invalid dimensions".to_string(),
            });
        }

        Ok(config_new)
    }
}
