use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::FitzError;

fn read_file_to_vector(file: &PathBuf) -> Option<Vec<String>> {
    // Attempt to open file
    let Ok(file_content) = File::open(&file) else {
        return None;
    };

    // Attempt to read lines into a vector of strings
    let Ok(lines) = BufReader::new(&file_content)
        .lines()
        .collect::<Result<Vec<String>, _>>() else {
            return None;
        };

    return Some(lines);
}

pub fn parse_save_file(file: &String) -> Result<PathBuf, FitzError> {
    let file_buf = PathBuf::from(file);
    let Some(lines) = read_file_to_vector(&file_buf) else {
        return Err(FitzError{ code: 6, message: "Can't access save file".to_string()});
    };

    let mut save_file_valid = true;
    let mut first_line = true;

    let mut width: usize = 0;
    'lines: for line in &lines {
        // Parse save game metadata
        if first_line {
            let line1: Vec<String> = line.split_whitespace().map(str::to_string).collect();
            // Check first line contains 4 parameters
            if line1.len() != 4 {
                save_file_valid = false;
                break;
            }

            // Check first line contains only numbers
            for param in &line1 {
                let _p: u16 = match param.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        save_file_valid = false;
                        break 'lines;
                    }
                };
            }

            // Ensure save file contains valid height for game board
            let height: usize = line1[2].parse().unwrap();
            if height != lines.len() - 1 {
                save_file_valid = false;
                break;
            }

            // Get width to process lines of game board later on
            width = line1[3].parse().unwrap();

            first_line = false;
        } else {
            // Check width of game board on each line
            if line.len() != width {
                save_file_valid = false;
                break;
            }

            // Check that line only contains valid characters
            if line.chars().any(|c| c != '*' && c != '#' && c != '.') {
                save_file_valid = false;
                break;
            }
        }
    }
    if !save_file_valid {
        return Err(FitzError {
            code: 7,
            message: "Invalid save file contents".to_string(),
        });
    }

    return Ok(file_buf);
}

pub fn parse_tile_file(file: &String) -> Result<PathBuf, FitzError> {
    let file_buf = PathBuf::from(file);
    let Some(lines) = read_file_to_vector(&file_buf) else {
            return Err(FitzError{ code: 2, message: "Can't access tilefile".to_string()});
    };

    let mut tile_file_valid = true;
    let mut line_break_count = 0;
    let mut tile_line_count = 0;
    // Parse lines for correct format
    for line in &lines {
        if line.is_empty() && tile_line_count % 5 == 0 {
            line_break_count = line_break_count + 1;
        } else if !line.is_empty() && line.chars().all(|c| c == ',' || c == '!') && line.len() == 5
        {
            tile_line_count = tile_line_count + 1;
        } else {
            tile_file_valid = false;
            break;
        }
    }
    // Ensure no incomplete tiles
    if tile_line_count % 5 != 0 {
        tile_file_valid = false;
    }

    // Ensure correct spacing between tiles
    let tile_count = tile_line_count / 5;
    if line_break_count + 1 != tile_count {
        tile_file_valid = false;
    }

    if !tile_file_valid {
        return Err(FitzError {
            code: 3,
            message: "Invalid tile file contents".to_string(),
        });
    }

    return Ok(file_buf);
}

#[cfg(test)]
mod ut {
    use super::*;
    use crate::vec_of_strings;
    mod read_file_to_vector {
        use super::*;
        #[test]
        fn test_1_read_file_to_vector() {
            let input = PathBuf::from("saves/save_valid");

            let expected = vec_of_strings!("3 1 4 3", "*..", "..#", "..#", "**#");

            let _output = match read_file_to_vector(&input) {
                Some(out) => {
                    assert_eq!(out, expected);
                }
                None => {
                    assert!(false)
                }
            };
        }
    }

    mod parse_tile_file {
        use super::*;
        #[test]
        fn test_1_tile_file_valid() {
            let input = String::from("tilefiles/tilefile_valid");

            let expected = PathBuf::from(&input);

            let _output = match parse_tile_file(&input) {
                Ok(out) => assert_eq!(expected, out),
                Err(_) => assert!(false),
            };
        }

        #[test]
        fn test_2_tile_file_not_found() {
            let input = String::from("tilefiles/non_existant");

            let expected = FitzError {
                code: 2,
                message: "Can't access tilefile".to_string(),
            };

            let _output = match parse_tile_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_3_tile_file_invalid_char() {
            let input = String::from("tilefiles/tilefile_invalid_char");

            let expected = FitzError {
                code: 3,
                message: "Invalid tile file contents".to_string(),
            };

            let _output = match parse_tile_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_4_tile_file_invalid_format() {
            let input = String::from("tilefiles/tilefile_invalid_format");

            let expected = FitzError {
                code: 3,
                message: "Invalid tile file contents".to_string(),
            };

            let _output = match parse_tile_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_5_tile_file_incomplete_tile() {
            let input = String::from("tilefiles/tilefile_invalid_format");

            let expected = FitzError {
                code: 3,
                message: "Invalid tile file contents".to_string(),
            };

            let _output = match parse_tile_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }
    }

    mod parse_save_file {
        use super::*;
        #[test]
        fn test_1_save_file_valid() {
            let input = String::from("saves/save_valid");

            let expected = PathBuf::from(&input);

            let _output = match parse_save_file(&input) {
                Ok(out) => assert_eq!(expected, out),
                Err(_) => assert!(false),
            };
        }

        #[test]
        fn test_2_save_file_not_found() {
            let input = String::from("saves/non_existant");

            let expected = FitzError {
                code: 6,
                message: "Can't access save file".to_string(),
            };

            let _output = match parse_save_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_3_save_file_invalid_metadata_char() {
            let input = String::from("saves/save_invalid_metadata_char");

            let expected = FitzError {
                code: 7,
                message: "Invalid save file contents".to_string(),
            };

            let _output = match parse_save_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_4_save_file_invalid_metadata_dimensions() {
            let input = String::from("saves/save_invalid_metadata_dimensions");

            let expected = FitzError {
                code: 7,
                message: "Invalid save file contents".to_string(),
            };

            let _output = match parse_save_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_5_save_file_invalid_board_char() {
            let input = String::from("saves/save_invalid_board_char");

            let expected = FitzError {
                code: 7,
                message: "Invalid save file contents".to_string(),
            };

            let _output = match parse_save_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }

        #[test]
        fn test_6_save_file_invalid_board_dimensions() {
            let input = String::from("saves/save_invalid_board_dimensions");

            let expected = FitzError {
                code: 7,
                message: "Invalid save file contents".to_string(),
            };

            let _output = match parse_save_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }
    }
}
