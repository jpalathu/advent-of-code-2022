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
