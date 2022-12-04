use std::{error::Error, fs};

#[derive(Debug)]
struct Pair {
    first_elf: (u32, u32),
    second_elf: (u32, u32),
}

fn parse_file_content(file_path: &str) -> Result<Vec<Pair>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;

    let lines = input.lines();
    let mut pairs = Vec::new();
    for line in lines {
        let (first_elf, second_elf) = line.split_once(",").unwrap();
        let (first_start, first_end) = first_elf.split_once("-").unwrap();
        let (second_start, second_end) = second_elf.split_once("-").unwrap();
        pairs.push(Pair {
            first_elf: (first_start.parse()?, first_end.parse()?),
            second_elf: (second_start.parse()?, second_end.parse()?),
        });
    }

    Ok(pairs)
}

fn check_fully_overlap(pair: &Pair) -> bool {
    let (first_start, first_end, second_start, second_end) = (
        pair.first_elf.0,
        pair.first_elf.1,
        pair.second_elf.0,
        pair.second_elf.1,
    );

    (first_start <= second_start && first_end >= second_end)
        || (second_start <= first_start && second_end >= first_end)
}

fn check_any_overlap(pair: &Pair) -> bool {
    let (first_start, first_end, second_start, second_end) = (
        pair.first_elf.0,
        pair.first_elf.1,
        pair.second_elf.0,
        pair.second_elf.1,
    );

    (first_start >= second_start && first_start <= second_end)
        || (second_start >= first_start && second_start <= first_end)
}

/// Find how many pairs fully overlap
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let pairs = parse_file_content(file_path)?;
    let total = pairs
        .iter()
        .map(|pair| check_fully_overlap(pair))
        .fold(0, |acc, does_overlap| {
            acc + if does_overlap { 1 } else { 0 }
        });
    Ok(total)
}

/// Find how many pairs overlap at all
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let pairs = parse_file_content(file_path)?;
    let total = pairs
        .iter()
        .map(|pair| check_any_overlap(pair))
        .fold(0, |acc, does_overlap| {
            acc + if does_overlap { 1 } else { 0 }
        });
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{puzzle_1_solution, puzzle_2_solution};

    #[test]
    fn test_puzzle_1() {
        let solution = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_puzzle_2() {
        let solution = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 4);
    }
}
