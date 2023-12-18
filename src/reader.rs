use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a text file to a string
pub fn file_to_string(path: &str) -> String {
    let f = fs::read_to_string(path);
    match f {
        Ok(s) => s,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

/// Reads a text file to a vector of strings
pub fn split_lines_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().collect::<Result<_, _>>()?)
}
