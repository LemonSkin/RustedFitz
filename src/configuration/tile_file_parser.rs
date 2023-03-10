use crate::{utils, FitzError};

//TODO Change to return a Vec<Vec<String>> of tiles
pub fn parse_tile_file(file: &String) -> Result<Vec<Vec<String>>, FitzError> {
    let Some(lines) = utils::read_file_to_vector(file) else {
            return Err(FitzError{ code: 2, message: "Can't access tilefile".to_string()});
    };

    let tile_file_invalid = FitzError {
        code: 3,
        message: "Invalid tile file contents".to_string(),
    };

    let mut tiles: Vec<Vec<String>> = Vec::new();
    let mut tile: Vec<String> = Vec::new();
    for line in &lines {
        if !line.is_empty() && line.chars().all(|c| c == ',' || c == '!') && line.len() == 5 {
            tile.push(line.clone());
        } else if line.is_empty() && tile.len() == 5 {
            tiles.push(tile.to_vec());
            tile.clear();
        } else {
            return Err(tile_file_invalid);
        }
    }
    tiles.push(tile.to_vec());

    for tile in &tiles {
        if tile.len() != 5 {
            return Err(tile_file_invalid);
        }
    }
    // println!("{:?}", tiles);

    // let mut line_break_count = 0;
    // let mut tile_line_count = 0;
    // // Parse lines for correct format
    // for line in &lines {
    //     if line.is_empty() && tile_line_count % 5 == 0 {
    //         line_break_count += 1;
    //     } else if !line.is_empty() && line.chars().all(|c| c == ',' || c == '!') && line.len() == 5
    //     {
    //         tile_line_count += 1;
    //     } else {
    //         return Err(tile_file_invalid);
    //     }
    // }
    // // Ensure no incomplete tiles
    // if tile_line_count % 5 != 0 {
    //     return Err(tile_file_invalid);
    // }

    // // Ensure correct spacing between tiles
    // let tile_count = tile_line_count / 5;
    // if line_break_count + 1 != tile_count {
    //     return Err(tile_file_invalid);
    // }

    Ok(tiles)
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
            match parse_tile_file(&input) {
                Ok(_) => assert!(true),
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

            match parse_tile_file(&input) {
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

            match parse_tile_file(&input) {
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

            match parse_tile_file(&input) {
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

            match parse_tile_file(&input) {
                Ok(_) => assert!(false),
                Err(err) => assert_eq!(expected, err),
            };
        }
    }
}
