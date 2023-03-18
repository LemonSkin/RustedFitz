use crate::game::move_processor;
use crate::game::PlayerMove;

pub fn check_game_over(game_board: &Vec<Vec<char>>, tile: &Vec<String>) -> bool {
    // Simple check: count number of ! in next tile and compare to number of empty spaces in game_board
    let mut empty_space_count = 0;
    for row in game_board {
        for c in row {
            if *c == '.' {
                empty_space_count += 1;
            }
        }
    }
    let mut tile_piece_count = 0;
    for row in tile {
        for c in row.chars() {
            if c == '!' {
                tile_piece_count += 1;
            }
        }
    }
    if tile_piece_count > empty_space_count {
        return true;
    }

    // Perform complex check via brute force - test tile for every rotation and return when
    // one match is found
    // To test, we have to go from -2,-2 .. game_board.len()+3, game_board[0].len()+3
    let max_y = i16::try_from(game_board.len() + 3).expect("Couldn't generate max_y");
    let max_x = i16::try_from(game_board[0].len() + 3).expect("Couldn't generate max_x");
    for y in -2..max_y {
        for x in -2..max_x {
            for rotation in 0..4 {
                let player_move = PlayerMove {
                    offset_y: y,
                    offset_x: x,
                    rotations: rotation,
                };
                // If any valid move exists, continue the game
                if let Some(_valid_move) =
                    move_processor::validate_move(&player_move, game_board, tile)
                {
                    return false;
                }
            }
        }
    }

    // No valid moves found so the game is over
    true
}
