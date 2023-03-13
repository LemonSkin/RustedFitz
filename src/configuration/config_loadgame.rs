use crate::configuration::config_players::ConfigPlayers;
use crate::{utils, FitzError};

#[derive(Debug, PartialEq)]
pub struct ConfigLoad {
    pub players: ConfigPlayers,
    pub next_tile_to_play: usize,
    pub next_player_turn: usize,
    pub height: u16,
    pub width: u16,
    pub board: Vec<Vec<char>>,
}

impl ConfigLoad {
    pub fn build(players: ConfigPlayers, filename: &String) -> Result<ConfigLoad, FitzError> {
        let Some(lines) = utils::read_file_to_vector(filename) else {
            return Err(FitzError{ code: 6, message: "Can't access save file".to_string()});
        };

        // Generate error message
        let save_file_invalid: FitzError = FitzError {
            code: 7,
            message: "Invalid save file contents".to_string(),
        };

        // Process first line
        let first_line: Vec<String> = lines
            .first()
            .unwrap()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        if first_line.len() != 4 {
            return Err(save_file_invalid);
        }

        let Ok(next_tile_to_play) = first_line[0].parse::<usize>() else {
            return Err(save_file_invalid);
        };
        // TODO Validate that next tile to play is valid

        // Validate next player turn
        let Ok(next_player_turn) = first_line[1].parse::<usize>() else {
            return Err(save_file_invalid);
        };
        if next_player_turn != 0 && next_player_turn != 1 {
            return Err(save_file_invalid);
        }
        //TODO Convert next_player_turn to enum value

        let Ok(height) = first_line[2].parse::<u16>() else {
            return Err(save_file_invalid);
        };
        // TODO Validate that height is in range 1 .. 999

        let Ok(width) = first_line[3].parse::<u16>() else {
            return Err(save_file_invalid);
        };
        // TODO validate that width is in range 1 .. 999

        let mut board: Vec<Vec<char>> = Vec::new();
        // Validate the game board
        for line in lines.iter().skip(1) {
            if line.len() != usize::from(width) {
                return Err(save_file_invalid);
            }

            // Check for correct chars and store in memory for later use
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                if c == '*' || c == '#' || c == '.' {
                    row.push(c);
                } else {
                    return Err(save_file_invalid);
                }
            }
            board.push(row);
        }

        Ok(ConfigLoad {
            players,
            next_tile_to_play,
            next_player_turn,
            height,
            width,
            board,
        })
    }
}

// mod parse_save_file {
//     use super::*;
//     #[test]
//     fn test_1_save_file_valid() {
//         let input = String::from("saves/save_valid");

//         let expected = PathBuf::from(&input);

//         match parse_save_file(&input) {
//             Ok(_) => assert!(true),
//             Err(_) => assert!(false),
//         };
//     }

//     #[test]
//     fn test_2_save_file_not_found() {
//         let input = String::from("saves/non_existant");

//         let expected = FitzError {
//             code: 6,
//             message: "Can't access save file".to_string(),
//         };

//         match parse_save_file(&input) {
//             Ok(_) => assert!(false),
//             Err(err) => assert_eq!(expected, err),
//         };
//     }

//     #[test]
//     fn test_3_save_file_invalid_metadata_char() {
//         let input = String::from("saves/save_invalid_metadata_char");

//         let expected = FitzError {
//             code: 7,
//             message: "Invalid save file contents".to_string(),
//         };

//         match parse_save_file(&input) {
//             Ok(_) => assert!(false),
//             Err(err) => assert_eq!(expected, err),
//         };
//     }

//     #[test]
//     fn test_4_save_file_invalid_metadata_dimensions() {
//         let input = String::from("saves/save_invalid_metadata_dimensions");

//         let expected = FitzError {
//             code: 7,
//             message: "Invalid save file contents".to_string(),
//         };

//         match parse_save_file(&input) {
//             Ok(_) => assert!(false),
//             Err(err) => assert_eq!(expected, err),
//         };
//     }

//     #[test]
//     fn test_5_save_file_invalid_board_char() {
//         let input = String::from("saves/save_invalid_board_char");

//         let expected = FitzError {
//             code: 7,
//             message: "Invalid save file contents".to_string(),
//         };

//         match parse_save_file(&input) {
//             Ok(_) => assert!(false),
//             Err(err) => assert_eq!(expected, err),
//         };
//     }

//     #[test]
//     fn test_6_save_file_invalid_board_dimensions() {
//         let input = String::from("saves/save_invalid_board_dimensions");

//         let expected = FitzError {
//             code: 7,
//             message: "Invalid save file contents".to_string(),
//         };

//         match parse_save_file(&input) {
//             Ok(_) => assert!(false),
//             Err(err) => assert_eq!(expected, err),
//         };
//     }
// }
