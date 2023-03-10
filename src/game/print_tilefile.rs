use std::process;

use crate::configuration::Config;

pub fn print_tilefile(config: &Config) {
    let mut row_chars: Vec<char> = Vec::with_capacity(5);
    let mut tile_chars: Vec<Vec<char>> = Vec::with_capacity(5);
    let mut tiles_chars: Vec<Vec<Vec<char>>> = Vec::new();

    for tile in &config.tilefile_contents {
        for row in tile {
            for char in row.chars() {
                row_chars.push(char);
            }
            tile_chars.push(row_chars.to_vec());
            row_chars.clear();
        }
        tiles_chars.push(tile_chars.to_vec());
        tile_chars.clear();
    }

    for tile in tiles_chars {
        let r90 = rotate_tile(&tile);
        let r180 = rotate_tile(&r90);
        let r270 = rotate_tile(&r180);

        for x in 0..5 {
            let l0: String = tile[x].iter().collect();
            let l90: String = r90[x].iter().collect();
            let l180: String = r180[x].iter().collect();
            let l270: String = r270[x].iter().collect();
            println!("{l0} {l90} {l180} {l270}");
        }
        println!();
    }
    process::exit(0);
}

fn rotate_tile(tile: &[Vec<char>]) -> Vec<Vec<char>> {
    let m: usize = tile[0].len();
    let n: usize = tile.len();
    let mut new_vec: Vec<Vec<char>> = tile.to_vec();

    for x in 0..n {
        for y in 0..m {
            new_vec[x][y] = tile[y][n - 1 - x];
        }
    }

    return new_vec;
}
