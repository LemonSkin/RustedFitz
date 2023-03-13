use std::io::{stdin, stdout, Write};

mod game_processor;
mod move_processor;
mod tile_rotator;
mod tilefile_printer;

use crate::configuration::config_newgame::ConfigNew;
use crate::configuration::Config;
use crate::GameOptions;

pub fn run(config: Config) {
    let mut game_board: Vec<Vec<char>> = Vec::new();
    let player_chars = ['*', '#'];
    let mut current_player = player_chars[0];
    let mut next_tile_to_play = 0;

    match config.game_option {
        GameOptions::View => tilefile_printer::print(&config.tilefile_contents),
        GameOptions::New(config_new) => game_board = generate_game_board(config_new),
        GameOptions::Load(config_load) => {
            game_board = config_load.board.to_vec();
            current_player = player_chars[config_load.next_player_turn];
            next_tile_to_play = config_load.next_tile_to_play;
        }
    };

    let mut game_over = false;

    while !game_over {
        print_game_board(&game_board);
        // TODO Only print tile if player is human
        // print_tile(&config.tilefile_contents[next_tile_to_play]);
        let mut move_valid = false;
        while !move_valid {
            // TODO Add logic to generate computer moves or read from stdin
            // Read user input
            print!("Player {current_player}] ");
            let player_move = read_user_input();
            move_valid = move_processor::process_player_move(
                player_move,
                &mut game_board,
                &config.tilefile_contents[next_tile_to_play],
                &current_player,
            );
        }
        // Update next tile to play
        next_tile_to_play += 1;
        if next_tile_to_play == config.tilefile_contents.len() {
            next_tile_to_play = 0;
        }

        // TODO Determine if game is won
        game_over = game_processor::check_game_over(
            &game_board,
            &config.tilefile_contents[next_tile_to_play],
        );

        if !game_over {
            // Change player
            if current_player == player_chars[0] {
                current_player = player_chars[1];
            } else {
                current_player = player_chars[0];
            }
        }
    }

    println!("Player {current_player} wins");
}

fn generate_game_board(config_new: ConfigNew) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();

    for _y in 0..config_new.height {
        let mut row: Vec<char> = Vec::new();
        for _x in 0..config_new.width {
            row.push('.');
        }
        board.push(row.to_vec());
    }

    board
}

fn print_game_board(game_board: &Vec<Vec<char>>) {
    for row in game_board {
        for point in row {
            print!("{point}");
        }
        println!();
    }
}

fn print_tile(tile: &Vec<String>) {
    for row in tile {
        for point in row.chars() {
            print!("{point}");
        }
        println!();
    }
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
    player_move
}
