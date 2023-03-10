use std::io::{stdin, stdout, Write};

mod move_processor;
mod print_tilefile;

use crate::configuration::config_newgame::ConfigNew;
use crate::configuration::Config;
use crate::{game, GameOptions};

pub fn run(config: &Config) {
    let mut game_board: Vec<Vec<char>> = Vec::new();

    match &config.game_option {
        GameOptions::View => print_tilefile::print_tilefile(config),
        GameOptions::New(config_new) => game_board = generate_game_board(&config_new),
        GameOptions::Load(config_load) => game_board = config_load.board.to_vec(),
    };
    let player_chars = ['*', '#'];
    let mut current_player = player_chars[0];
    let mut game_over = false;

    while !game_over {
        print_game_board(&game_board);
        let mut move_valid = false;
        while !move_valid {
            // TODO Add logic to generate computer moves or read from stdin
            // Read user input
            print!("Player {current_player}] ");
            let player_move = read_user_input();

            match move_processor::process_player_move(
                &player_move,
                &game_board,
                &config.tilefile_contents[0],
            ) {
                Some(g) => {
                    game_board = g;
                    move_valid = true;
                }
                None => move_valid = false,
            }
        }

        // TODO Determine if game is won

        // Change player
        if current_player == player_chars[0] {
            current_player = player_chars[1];
        } else {
            current_player = player_chars[0];
        }
    }
}

fn generate_game_board(config_new: &ConfigNew) -> Vec<Vec<char>> {
    let height = usize::from(config_new.height);
    let width = usize::from(config_new.width);

    let mut row: Vec<char> = Vec::with_capacity(width);
    let mut board: Vec<Vec<char>> = Vec::with_capacity(usize::from(height));

    for _i in 0..height {
        for _j in 0..width {
            row.push('.');
        }
        board.push(row.to_vec());
        row.clear();
    }

    return board;
}

fn print_game_board(game_board: &Vec<Vec<char>>) {
    for row in game_board {
        for point in row {
            print!("{point}");
        }
        println!();
    }
    println!();
}

fn read_user_input() -> String {
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
    return player_move;
}
