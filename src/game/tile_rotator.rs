pub fn rotate(tile: &Vec<String>) -> Vec<String> {
    // Convert tile to chars
    let mut tile_as_chars: Vec<Vec<char>> = Vec::new();
    for row in tile {
        let row_as_chars = row.chars().collect();
        tile_as_chars.push(row_as_chars);
    }

    let mut rotated_tile_as_chars: Vec<Vec<char>> = tile_as_chars.to_vec();
    for (y, row) in tile_as_chars.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            // println!("{x}, {y}");
            rotated_tile_as_chars[x][y] = tile_as_chars[tile_as_chars.len() - 1 - y][x];
        }
    }

    // Combine back to string
    let mut rotated_tile: Vec<String> = Vec::new();
    for row in rotated_tile_as_chars {
        let row_as_string = row.iter().collect();
        rotated_tile.push(row_as_string);
    }
    rotated_tile
}
