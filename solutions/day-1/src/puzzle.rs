use std::{error::Error, fs};

fn parse_file_content_into_vec(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;

    let elf_calories = input
        .split("\n\n")
        .map(|group_calories| {
            group_calories
                .split("\n")
                .fold(0, |acc, calories| acc + calories.parse::<i32>().unwrap())
        })
        .collect::<Vec<i32>>();

    Ok(elf_calories)
}

/// Find max calories
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let elf_calories = parse_file_content_into_vec(file_path)?;
    let max_calories = elf_calories.iter().max().unwrap();
    Ok(*max_calories)
}

/// Find top 3 largest calories and sum them
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let mut elf_calories = parse_file_content_into_vec(file_path)?;
    elf_calories.sort_by(|a, b| b.cmp(a));
    let total_calories = elf_calories.iter().take(3).sum();
    Ok(total_calories)
}

#[cfg(test)]
mod tests {
    use super::{puzzle_1_solution, puzzle_2_solution};

    #[test]
    fn test_puzzle_1() {
        let solution = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 24000);
    }

    #[test]
    fn test_puzzle_2() {
        let solution = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 45000);
    }
}
