use std::{error::Error, fs};

const UPPERCASE_CHAR_CODE: usize = 65;
const LOWERCASE_CHAR_CODE: usize = 97;
const POSSIBLE_ITEMS_LENGTH: usize = 52;

fn parse_file_content(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(file_path)?;
    let rucksacks = input.lines().map(String::from).collect::<Vec<String>>();
    Ok(rucksacks)
}

fn get_item_index(c: char) -> usize {
    if c.is_lowercase() {
        (c as usize) - LOWERCASE_CHAR_CODE
    } else {
        (c as usize) - UPPERCASE_CHAR_CODE + (POSSIBLE_ITEMS_LENGTH / 2)
    }
}

fn find_compartment_common_item_priority(rucksack: &str) -> usize {
    let mut item_counts = [0; POSSIBLE_ITEMS_LENGTH];

    let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

    let (shorter_compartment, longer_compartment) =
        if first_compartment.len() < second_compartment.len() {
            (first_compartment, second_compartment)
        } else {
            (second_compartment, first_compartment)
        };

    for c in shorter_compartment.chars() {
        item_counts[get_item_index(c)] += 1;
    }

    for c in longer_compartment.chars() {
        let index = get_item_index(c);
        if item_counts[index] > 0 {
            // The priority starts at 1 instead of 0
            return index + 1;
        }
    }

    panic!("no common item found");
}

fn find_group_common_item_priority(group: &[String]) -> usize {
    let mut item_counts = [0; POSSIBLE_ITEMS_LENGTH];

    let (first_rucksack, second_rucksack, third_rucksack) = (
        group.get(0).unwrap(),
        group.get(1).unwrap(),
        group.get(2).unwrap(),
    );

    for c in first_rucksack.chars() {
        let index = get_item_index(c);
        if item_counts[index] == 0 {
            item_counts[index] += 1;
        }
    }

    for c in second_rucksack.chars() {
        let index = get_item_index(c);
        if item_counts[index] == 1 {
            item_counts[index] += 1;
        }
    }

    for c in third_rucksack.chars() {
        let index = get_item_index(c);
        if item_counts[index] == 2 {
            return index + 1;
        }
    }

    panic!("no common item found");
}

/// Sum priority of common item between rucksacks compartments
pub(super) fn puzzle_1_solution(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let rucksacks = parse_file_content(file_path)?;
    let total = rucksacks
        .iter()
        .map(|rucksack| find_compartment_common_item_priority(rucksack))
        .sum();
    Ok(total)
}

/// Sum priority of common item found in group of 3 rucksacks
pub(super) fn puzzle_2_solution(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let rucksacks = parse_file_content(file_path)?;
    let total = rucksacks
        .chunks(3)
        .map(|group| find_group_common_item_priority(group))
        .fold(0, |acc, priority| acc + priority);
    Ok(total)
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
        assert_eq!(solution, 70);
    }
}
