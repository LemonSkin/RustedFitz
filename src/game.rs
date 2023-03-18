// use std::io::{stdin, stdout, Write};

mod cpu_players;
mod game_processor;
mod human_player;
mod move_processor;
mod tile_rotator;
mod tilefile_printer;

use crate::configuration::config_newgame::ConfigNew;
use crate::configuration::Config;
use crate::GameOptions;
use crate::PlayerType;

#[derive(Debug)]
pub struct Player {
    ptype: PlayerType,
    pub icon: char,
    pub player_move: PlayerMove,
}
#[derive(Clone, Debug)]
pub struct PlayerMove {
    pub offset_y: i16,
    pub offset_x: i16,
    pub rotations: u16,
}

pub fn run(config: Config) {
    let mut game_board: Vec<Vec<char>> = Vec::new();
    let mut current_player = 0;
    let mut next_tile_to_play = 0;

    let mut player1 = Player {
        ptype: config.player_config.p1type,
        icon: '*',
        player_move: PlayerMove {
            offset_y: -2,
            offset_x: -2,
            rotations: 0,
        },
    };

    let mut player2 = Player {
        ptype: config.player_config.p2type,
        icon: '#',
        player_move: PlayerMove {
            offset_y: -2,
            offset_x: -2,
            rotations: 0,
        },
    };

    let players = [&mut player1, &mut player2];

    match config.game_option {
        GameOptions::View => tilefile_printer::print(&config.tilefile_contents),
        GameOptions::New(config_new) => game_board = generate_game_board(config_new),
        GameOptions::Load(config_load) => {
            game_board = config_load.board.to_vec();
            current_player = config_load.next_player_turn;
            next_tile_to_play = config_load.next_tile_to_play;
        }
    };

    let mut last_move: PlayerMove = PlayerMove {
        offset_y: -2,
        offset_x: -2,
        rotations: 0,
    };
    print_game_board(&game_board);

    'game_loop: loop {
        let mut coordinates_to_update: Vec<(usize, usize)> = Vec::new();
        match players[current_player].ptype {
            PlayerType::Human => {
                // Print the tile
                print_tile(&config.tilefile_contents[next_tile_to_play]);

                // Handle human input
                coordinates_to_update = human_player::run(
                    players[current_player],
                    &game_board,
                    &config.tilefile_contents[next_tile_to_play],
                );
            }
            PlayerType::Computer1 => {
                players[current_player].player_move = last_move.clone();

                // Generate computer move
                coordinates_to_update = cpu_players::cpu1(
                    players[current_player],
                    &game_board,
                    &config.tilefile_contents[next_tile_to_play],
                );
            }
            PlayerType::Computer2 => println!("Computer 2 turn..."),
        }

        // Coordinates should never be empty at this point
        if !coordinates_to_update.is_empty() {
            for coordinate in coordinates_to_update {
                game_board[coordinate.0][coordinate.1] = players[current_player].icon;
            }
            print_game_board(&game_board);
        }

        // Record the last move
        last_move = players[current_player].player_move.clone();

        // Update next tile to play
        next_tile_to_play += 1;
        if next_tile_to_play == config.tilefile_contents.len() {
            next_tile_to_play = 0;
        }

        // Determine if game is won
        if game_processor::check_game_over(
            &game_board,
            &config.tilefile_contents[next_tile_to_play],
        ) {
            break 'game_loop;
        }

        // Change player
        if current_player == 1 {
            current_player = 0;
        } else {
            current_player = 1;
        }
    }

    println!("Player {} wins", players[current_player].icon);
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

// fn read_user_input() -> Option<PlayerMove> {
//     let mut player_move = String::new();
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut player_move)
//         .expect("Did not enter a correct string");
//     // Remove carriage return and line feed if present
//     if let Some('\n') = player_move.chars().next_back() {
//         player_move.pop();
//     }
//     if let Some('\r') = player_move.chars().next_back() {
//         player_move.pop();
//     }

//     let vec_move: Vec<String> = player_move.split_whitespace().map(str::to_string).collect();
//     if vec_move.len() != 3 {
//         return None;
//     }

//     let Ok(offset_x) = vec_move[1].parse::<i16>() else {
//         return None;
//     };

//     let Ok(offset_y) = vec_move[0].parse::<i16>() else {
//         return None;
//     };

//     let rotations: u8 = match vec_move[2].as_str() {
//         "0" => 0,
//         "90" => 1,
//         "180" => 2,
//         "270" => 3,
//         _ => return None,
//     };

//     Some(PlayerMove {
//         offset_y,
//         offset_x,
//         rotations,
//     })
// }
