use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// TODO Change to accept a string
pub fn read_file_to_vector(file: &String) -> Option<Vec<String>> {
    // Attempt to open file
    let Ok(file_content) = File::open(PathBuf::from(file)) else {
        return None;
    };

    // Attempt to read lines into a vector of strings
    let Ok(lines) = BufReader::new(&file_content)
        .lines()
        .collect::<Result<Vec<String>, _>>() else {
            return None;
        };

    Some(lines)
}
