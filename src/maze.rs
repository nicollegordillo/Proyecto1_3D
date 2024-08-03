use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_maze(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let maze = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(maze)
}