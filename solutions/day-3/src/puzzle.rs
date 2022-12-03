use std::{error::Error, fs};

const UPPERCASE_CHAR_CODE: usize = 65;
const LOWERCASE_CHAR_CODE: usize = 97;
const POSSIBLE_ITEMS_LENGTH: usize = 52;

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}
impl Rucksack {
    fn from_str(rucksack: &str) -> Self {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
        Self {
            first_compartment: first_compartment.to_owned(),
            second_compartment: second_compartment.to_owned(),
        }
    }

    fn find_compartment_common_item_priority(&self) -> usize {
        let mut item_counts = [0; POSSIBLE_ITEMS_LENGTH];

        let (shorter_compartment, longer_compartment) =
            if self.first_compartment.len() < self.second_compartment.len() {
                (&self.first_compartment, &self.second_compartment)
            } else {
                (&self.second_compartment, &self.first_compartment)
            };

        for c in shorter_compartment.chars() {
            item_counts[get_item_index(c)] += 1;
        }

        for c in longer_compartment.chars() {
            let index = get_item_index(c);
            if item_counts[index] != 0 {
                // The priority starts at 1 instead of 0
                return index + 1;
            }
        }

        panic!("no common item found");
    }
}

fn get_item_index(c: char) -> usize {
    if c.is_lowercase() {
        (c as usize) - LOWERCASE_CHAR_CODE
    } else {
        (c as usize) - UPPERCASE_CHAR_CODE + (POSSIBLE_ITEMS_LENGTH / 2)
    }
}

fn parse_file_content(file_path: &str) -> Result<Vec<Rucksack>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;
    let rucksacks = input
        .lines()
        .map(Rucksack::from_str)
        .collect::<Vec<Rucksack>>();
    Ok(rucksacks)
}

/// Sum priority of common item between knapsack compartments
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let rucksacks = parse_file_content(file_path)?;
    let total = rucksacks
        .iter()
        .map(|rucksack| rucksack.find_compartment_common_item_priority())
        .sum();
    Ok(total)
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
        let solution = puzzle_1_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 157);
    }

    #[test]
    fn test_puzzle_2() {
        let solution = puzzle_2_solution("./input/sample-puzzle-input.txt").unwrap();
        assert_eq!(solution, 0);
    }
}
