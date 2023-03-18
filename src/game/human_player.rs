use std::io::{stdin, stdout, Write};

use crate::game::{move_processor, Player, PlayerMove};

pub fn run(
    player: &mut Player,
    game_board: &Vec<Vec<char>>,
    tile: &Vec<String>,
) -> Vec<(usize, usize)> {
    loop {
        print!("Player {}] ", player.icon);
        if let Some(player_move) = read_user_input() {
            player.player_move = player_move;
            if let Some(coords) =
                move_processor::validate_move(&player.player_move, game_board, tile)
            {
                return coords;
            }
        }
    }
}

fn read_user_input() -> Option<PlayerMove> {
    let mut player_move = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut player_move)
        .expect("Did not enter a correct string");
    // Remove carriage return and line feed if present
    if let Some('\n') = player_move.chars().next_back() {
        player_move.pop();
    }
    if let Some('\r') = player_move.chars().next_back() {
        player_move.pop();
    }

    let vec_move: Vec<String> = player_move.split_whitespace().map(str::to_string).collect();
    if vec_move.len() != 3 {
        return None;
    }

    let Ok(offset_x) = vec_move[1].parse::<i16>() else {
        return None;
    };

    let Ok(offset_y) = vec_move[0].parse::<i16>() else {
        return None;
    };

    let rotations: u16 = match vec_move[2].as_str() {
        "0" => 0,
        "90" => 1,
        "180" => 2,
        "270" => 3,
        _ => return None,
    };

    Some(PlayerMove {
        offset_y,
        offset_x,
        rotations,
    })
}
