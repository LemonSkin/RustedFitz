use crate::game::{move_processor, Player, PlayerMove};

pub fn cpu1(
    player: &mut Player,
    game_board: &Vec<Vec<char>>,
    tile: &Vec<String>,
) -> Vec<(usize, usize)> {
    let r_start = player.player_move.offset_y;
    let c_start = player.player_move.offset_x;
    player.player_move.rotations = 0;
    let length = i16::try_from(game_board.len()).expect("Failed to parse length");
    let width = i16::try_from(game_board[0].len()).expect("Failed to parse width");
    loop {
        if let Some(coords) = move_processor::validate_move(&player.player_move, game_board, tile) {
            println!(
                "Player {} => {} {} rotated {}",
                player.icon,
                player.player_move.offset_y,
                player.player_move.offset_x,
                player.player_move.rotations * 90
            );
            return coords;
        };
        player.player_move.offset_x += 1;
        if player.player_move.offset_x > width + 2 {
            player.player_move.offset_x = -2;
            player.player_move.offset_y += 1;
        }

        if player.player_move.offset_y > length + 2 {
            player.player_move.offset_y = -2;
        }

        if player.player_move.offset_x == c_start && player.player_move.offset_y == r_start {
            player.player_move.rotations += 1;
            if player.player_move.rotations == 4 {
                player.player_move.rotations = 0;
            }
        }
    }
}
