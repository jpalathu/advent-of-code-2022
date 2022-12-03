#!/bin/sh

read -d '' MAIN << EOF
mod puzzle;

use puzzle::{puzzle_1_solution, puzzle_2_solution};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = puzzle_1_solution("./input/puzzle-input.txt")?;
    println!("Puzzle 1 solution: {}", solution);
    let solution = puzzle_2_solution("./input/puzzle-input.txt")?;
    println!("Puzzle 2 solution: {}", solution);
    Ok(())
}
EOF

read -d '' PUZZLE << EOF
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_file_content(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {}

    Ok(())
}

/// TODO: Explain problem
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

/// TODO: Explain problem
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{puzzle_1_solution, puzzle_2_solution};

    #[test]
    fn test_puzzle_1() {
        let answer = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_puzzle_2() {
        let answer = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(answer, 0);
    }
}
EOF

if [ $# -eq 0 ]
then
    echo "Please provide an argument for the new day"
else
    # Create the cargo package
    cd solutions   
    cargo new $1
    # Create the input directory and files
    cd $1
    mkdir input
    cd input
    touch puzzle-input.txt
    touch sample-puzzle-input.txt 
    # Set up the source code template for the puzzles
    cd ../src
    echo "$MAIN" > main.rs
    echo "$PUZZLE" > puzzle.rs
fi