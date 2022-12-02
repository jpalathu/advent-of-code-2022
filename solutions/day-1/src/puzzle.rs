use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_file_content_into_vec(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut elf_calories = Vec::new();
    let mut current_calories = 0;

    for line in reader.lines() {
        let line = line?;
        let calories = line.trim();

        if calories != "" {
            current_calories += calories.parse::<i32>()?;
        } else {
            elf_calories.push(current_calories);
            current_calories = 0;
        }
    }
    elf_calories.push(current_calories);

    Ok(elf_calories)
}

// Find max calories
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let elf_calories = parse_file_content_into_vec(file_path)?;
    let max_calories = elf_calories.iter().max().unwrap();
    Ok(*max_calories)
}

// Find top 3 largest calories and sum them
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let mut elf_calories = parse_file_content_into_vec(file_path)?;
    elf_calories.sort_by(|a, b| b.cmp(a));
    let total_calories = elf_calories
        .iter()
        .take(3)
        .fold(0, |acc, calories| acc + *calories);
    Ok(total_calories)
}

#[cfg(test)]
mod tests {
    use super::{puzzle_1_solution, puzzle_2_solution};

    #[test]
    fn test_puzzle_1() {
        let max_calories = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(max_calories, 24000);
    }

    #[test]
    fn test_puzzle_2() {
        let total_calories = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(total_calories, 45000);
    }
}
