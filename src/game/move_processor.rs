use crate::game::tile_rotator;
use crate::game::PlayerMove;

pub fn validate_player_move(
    player_move: PlayerMove,
    game_board: &Vec<Vec<char>>,
    tile: &Vec<String>,
) -> Option<Vec<(usize, usize)>> {
    // Perform given number of tile rotations
    let mut tile = tile.to_owned();
    for _n in 0..player_move.rotations {
        tile = tile_rotator::rotate(&tile);
    }
    // Generate tile as vector of Vec<char>
    let mut tile_as_chars: Vec<Vec<char>> = Vec::new();
    for row in &tile {
        let row_as_chars = row.chars().collect();
        // println!("{:?}", row_as_chars);
        tile_as_chars.push(row_as_chars);
    }

    // Generate coordinates on gameboard to update
    let mut coordinates_to_update: Vec<(usize, usize)> = Vec::new();
    for (y, row) in tile_as_chars.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if tile_as_chars[y][x] == '!' {
                // Generate game_board coordinate from tile coordinate and
                // check top and left sides of game_board (ensure index is not negative in either dimension)
                let row_index = i16::try_from(y).expect("Failed to convert y coordinate");
                let Ok(game_board_coordinate_y) = usize::try_from(row_index - 2 + player_move.offset_y) else {
                    return None;
                };
                let column_index = i16::try_from(x).expect("Failed to convert x coordinate");
                let Ok(game_board_coordinate_x) = usize::try_from(column_index - 2 + player_move.offset_x) else {
                    return None;
                };
                // Check that game_board coordinates exist on game_board and check if it is empty
                if game_board_coordinate_x < game_board[0].len()
                    && game_board_coordinate_y < game_board.len()
                    && game_board[game_board_coordinate_y][game_board_coordinate_x] == '.'
                {
                    coordinates_to_update.push((game_board_coordinate_y, game_board_coordinate_x));
                } else {
                    // If a ! will be off the bottom and right sides game_board or
                    // game_board already contains something, the move is invalid
                    return None;
                }
            }
        }
    }

    Some(coordinates_to_update)
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test_1() {
        let player_move = String::from("1 2 180");
        let mut game_board = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let tile: Vec<String> = vec![
            ",,,,,".to_string(),
            ",!!,,".to_string(),
            ",!!!,".to_string(),
            ",,,,,".to_string(),
            ",,,,,".to_string(),
        ];
    }
}
