use crate::game;

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

    // Otherwise perform complex check by generating an adjacency score
    // TODO Correct behaviour for complex shapes by generating adjacency vector
    // First generate padded tile
    let mut tile_as_chars: Vec<Vec<char>> = Vec::new();
    let tile_padding: Vec<char> = vec![',', ',', ',', ',', ',', ',', ','];
    tile_as_chars.push(tile_padding.to_owned());
    for row in tile {
        let mut row_as_chars: Vec<char> = row.chars().collect();
        row_as_chars.insert(0, '.');
        row_as_chars.push(',');
        tile_as_chars.push(row_as_chars);
    }
    tile_as_chars.push(tile_padding);

    let mut adjacency_score_tile = 0;
    for (y, row) in tile_as_chars.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            // Skip any padding or non playable pieces
            if *col != '!' {
                continue;
            } else {
                for i in y - 1..y + 2 {
                    for j in x - 1..x + 2 {
                        if tile_as_chars[i][j] == '!' && !(i == y && j == x) {
                            adjacency_score_tile += 1;
                        }
                    }
                }
            }
        }
    }

    // Next generate padded gameboard
    let mut padded_game_board = game_board.to_vec();
    for row in &mut padded_game_board {
        row.insert(0, ',');
        row.push(',');
    }
    let mut game_board_padding: Vec<char> = Vec::new();
    for i in 0..padded_game_board[0].len() {
        game_board_padding.push(',');
    }
    padded_game_board.insert(0, game_board_padding.to_owned());
    padded_game_board.push(game_board_padding);

    let mut adjacency_score_board = 0;
    for (y, row) in padded_game_board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != '.' {
                continue;
            } else {
                for i in y - 1..y + 2 {
                    for j in x - 1..x + 2 {
                        if padded_game_board[i][j] == '.' && !(i == y && j == x) {
                            adjacency_score_board += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{adjacency_score_tile}, {adjacency_score_board}");

    if adjacency_score_tile > adjacency_score_board {
        return true;
    }

    false
}
