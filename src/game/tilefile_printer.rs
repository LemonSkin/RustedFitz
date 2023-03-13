use std::process;

use crate::game::tile_rotator;

pub fn print(tilefile: &Vec<Vec<String>>) {
    for tile in tilefile {
        let r90 = tile_rotator::rotate(tile);
        let r180 = tile_rotator::rotate(&r90);
        let r270 = tile_rotator::rotate(&r180);

        for row_index in 0..tile.len() {
            println!(
                "{} {} {} {}",
                tile[row_index], r90[row_index], r180[row_index], r270[row_index]
            );
        }
        println!();
    }
    process::exit(0);
}
