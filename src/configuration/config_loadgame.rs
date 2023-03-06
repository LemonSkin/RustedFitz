use std::path::PathBuf;

use crate::configuration::config_players::ConfigPlayers;
use crate::configuration::file_parser;
use crate::FitzError;

#[derive(Debug, PartialEq)]
pub struct ConfigLoad {
    pub players: ConfigPlayers,
    pub filename: PathBuf,
}

impl ConfigLoad {
    pub fn build(players: ConfigPlayers, filename: &String) -> Result<ConfigLoad, FitzError> {
        let config_load = ConfigLoad {
            players: players,
            filename: file_parser::parse_save_file(filename)?,
        };
        return Ok(config_load);
    }
}
