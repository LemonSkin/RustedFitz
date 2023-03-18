use crate::{utils, FitzError};

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

    Ok(tiles)
}
