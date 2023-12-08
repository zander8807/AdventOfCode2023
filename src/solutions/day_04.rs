use std::collections::HashSet;

use super::Solver;

fn parse_line(s: &str) -> (Vec<u64>, Vec<u64>) {
    let mut split = s.split(" | ");

    let (winning_nums_raw, scratch_nums_raw) = (split.next().unwrap(), split.next().unwrap());

    let winning_nums_raw = winning_nums_raw.split(':').last().unwrap();

    let winning_nums = winning_nums_raw
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let scratch_nums: Vec<u64> = scratch_nums_raw
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    (winning_nums, scratch_nums)
}

fn num_matches<'a>(winning_nums: &'a [u64], scratch_nums: &'a [u64]) -> u32 {
    let winning_nums = winning_nums.iter().cloned().collect::<HashSet<u64>>();
    let scratch_nums = scratch_nums.iter().cloned().collect::<HashSet<u64>>();

    winning_nums
        .intersection(&scratch_nums)
        .cloned()
        .count() as u32
}

fn calculate_score<'a>(winning_nums: &'a [u64], scratch_nums: &'a [u64]) -> u64 {
    let matching_numbers_count = num_matches(winning_nums, scratch_nums);
    if matching_numbers_count == 0 {
        0
    } else {
        2_u64.pow(matching_numbers_count - 1)
    }
}

pub struct DayFourSolver {}

impl<'a> Solver<'a> for DayFourSolver {
    fn part_1(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let games: u64 = input
            .iter()
            .map(|line| {
                let (winning_nums, scratch_nums) = parse_line(line);

                
                calculate_score(&winning_nums, &scratch_nums)
            })
            .sum();

        Ok(games.to_string())
    }

    fn part_2(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let mut num_of_instances = vec![1_u32; input.len()];

        let all_matches = input
            .iter()
            .map(|line| {
                let (winning_nums, scratch_nums) = parse_line(line);

                num_matches(&winning_nums, &scratch_nums)
            })
            .collect::<Vec<u32>>();

        println!("all matches: {:?}", all_matches);

        for (i, &matches) in all_matches.iter().enumerate() {
            if i == all_matches.len() - 1 {
                break;
            }

            if matches == 0 {
                continue;
            }

            // win all cards from the next one to the number of matches
            for j in i + 1..all_matches.len().min(i + matches as usize + 1) {
                num_of_instances[j] += num_of_instances[i];
            }
        }

        let total_instances: u32 = num_of_instances.iter().sum();
        Ok(total_instances.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{normalize_input, Solver};

    use super::DayFourSolver;

    const INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn part_1_example() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DayFourSolver {};

        let res = solver.part_1(&input).unwrap();

        assert_eq!(res, "13");
    }

    #[test]
    fn part_2_example() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DayFourSolver {};

        let res = solver.part_2(&input).unwrap();

        assert_eq!(res, "30");
    }
}
