// use std::path::PathBuf;

use crate::{FitzError, GameOptions};
mod tile_file_parser;

mod config_loadgame;
use config_loadgame::ConfigLoad;

pub mod config_newgame;
use config_newgame::ConfigNew;

mod config_players;
use config_players::ConfigPlayers;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub tilefile_contents: Vec<Vec<String>>,
    pub game_option: GameOptions<ConfigNew, ConfigLoad>,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, FitzError> {
        let input: Vec<String> = args.skip(1).collect();

        if input.len() == 1 || input.len() == 4 || input.len() == 5 {
            let mut config: Config = Config {
                tilefile_contents: tile_file_parser::parse_tile_file(&input[0])?,
                game_option: GameOptions::View,
            };

            // Generate configuration for new or load game
            if input.len() > 1 {
                let players: ConfigPlayers = ConfigPlayers::build(&input[1], &input[2])?;
                // Assign load configuration or else generate configuration for new game
                if input.len() == 4 {
                    let config_load = ConfigLoad::build(players, &input[3])?;
                    config.game_option = GameOptions::Load(config_load);
                } else {
                    let config_new = ConfigNew::build(players, &input[3], &input[4])?;
                    config.game_option = GameOptions::New(config_new);
                }
            }

            return Ok(config);
        }

        Err(FitzError {
            code: 2,
            message: "Usage: fitz tilefile [p1type p2type [height width | filename]]".to_string(),
        })
    }
}
