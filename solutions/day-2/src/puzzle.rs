use std::{error::Error, fs};

enum Outcomes {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[derive(PartialEq, Copy, Clone)]
enum Choices {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Round {
    opponent_choice: Choices,
    player_choice: Choices,
}
impl Round {
    fn calculate_strategy_1_score(&self) -> u32 {
        match (self.opponent_choice, self.player_choice) {
            (Choices::Rock, Choices::Rock) => (Choices::Rock as u32) + (Outcomes::Draw as u32),
            (Choices::Rock, Choices::Paper) => (Choices::Paper as u32) + (Outcomes::Win as u32),
            (Choices::Rock, Choices::Scissors) => {
                (Choices::Scissors as u32) + (Outcomes::Lose as u32)
            }
            (Choices::Paper, Choices::Rock) => (Choices::Rock as u32) + (Outcomes::Lose as u32),
            (Choices::Paper, Choices::Paper) => (Choices::Paper as u32) + (Outcomes::Draw as u32),
            (Choices::Paper, Choices::Scissors) => {
                (Choices::Scissors as u32) + (Outcomes::Win as u32)
            }
            (Choices::Scissors, Choices::Rock) => (Choices::Rock as u32) + (Outcomes::Win as u32),
            (Choices::Scissors, Choices::Paper) => {
                (Choices::Paper as u32) + (Outcomes::Lose as u32)
            }
            (Choices::Scissors, Choices::Scissors) => {
                (Choices::Scissors as u32) + (Outcomes::Draw as u32)
            }
        }
    }

    fn lose_choice(&self) -> Choices {
        match self.opponent_choice {
            Choices::Rock => Choices::Scissors,
            Choices::Paper => Choices::Rock,
            Choices::Scissors => Choices::Paper,
        }
    }

    fn win_choice(&self) -> Choices {
        match self.opponent_choice {
            Choices::Rock => Choices::Paper,
            Choices::Paper => Choices::Scissors,
            Choices::Scissors => Choices::Rock,
        }
    }

    fn calculate_strategy_2_score(&self) -> u32 {
        match self.player_choice {
            Choices::Rock => (self.lose_choice() as u32) + (Outcomes::Lose as u32),
            Choices::Paper => (self.opponent_choice as u32) + (Outcomes::Draw as u32),
            Choices::Scissors => (self.win_choice() as u32) + (Outcomes::Win as u32),
        }
    }
}

fn map_choices_to_variant(round: &str) -> Result<Round, Box<dyn Error>> {
    let mut choices = round.split_whitespace();
    let opponent_choice = choices.next().unwrap();
    let player_choice = choices.next().unwrap();

    let opponent_choice = match opponent_choice {
        "A" => Ok(Choices::Rock),
        "B" => Ok(Choices::Paper),
        "C" => Ok(Choices::Scissors),
        _ => Err("not a valid opponent choice"),
    }?;

    let player_choice = match player_choice {
        "X" => Ok(Choices::Rock),
        "Y" => Ok(Choices::Paper),
        "Z" => Ok(Choices::Scissors),
        _ => Err("not a valid player choice"),
    }?;

    Ok(Round {
        player_choice,
        opponent_choice,
    })
}

fn parse_file_content(file_path: &str) -> Result<Vec<Round>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;
    let rounds = input
        .lines()
        .map(|round| map_choices_to_variant(round).unwrap())
        .collect::<Vec<Round>>();
    Ok(rounds)
}

/// Calculate total score following strategy 1
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let rounds = parse_file_content(file_path)?;

    let total = rounds
        .iter()
        .map(|round| round.calculate_strategy_1_score())
        .sum();

    Ok(total)
}

/// Calculate total score following strategy 2
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let rounds = parse_file_content(file_path)?;

    let total = rounds
        .iter()
        .map(|round| round.calculate_strategy_2_score())
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{puzzle_1_solution, puzzle_2_solution};

    #[test]
    fn test_puzzle_1() {
        let solution = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 15);
    }

    #[test]
    fn test_puzzle_2() {
        let solution = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 12);
    }
}
