use std::{collections::HashMap, ops::Add};

use super::Solver;

use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_TO_NUM: HashMap<&'static str, i64> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 0),
    ]
    .iter()
    .cloned()
    .collect();
    static ref WORD_TO_NUM_REV: HashMap<&'static str, i64> = [
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
        ("net", 0),
    ]
    .iter()
    .cloned()
    .collect();
}

struct CalibrationValue {
    parse_words: bool,
    val: i64,
}

impl CalibrationValue {
    fn find_number<'a>(&self, chars: &[char], is_reversed: bool) -> Option<i64> {
        // if we don't need to worry about parsing the words, let's just find the first occurrence of numbers
        if !self.parse_words {
            return chars
                .iter()
                .find(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap().into());
        };

        for i in 0..chars.len() {
            if chars[i].is_numeric() {
                return Some(chars[i].to_digit(10).unwrap().into());
            }
            let mut curr_str = "".to_string();
            for curr_i in i..usize::min(i + 5, chars.len()) {
                let curr_char = chars[curr_i];

                curr_str.push(curr_char);
                if is_reversed {
                    if let Some(num) = WORD_TO_NUM_REV.get(&*curr_str) {
                        return Some(*num);
                    }
                } else {
                    if let Some(num) = WORD_TO_NUM.get(&*curr_str) {
                        return Some(*num);
                    }
                }
            }
        }

        None
    }
}

impl<'a> Add<&'a str> for CalibrationValue {
    type Output = Result<Self, ()>;
    fn add(self, rhs: &'a str) -> Result<Self, ()> {
        let chars: Vec<char> = rhs.chars().collect();
        let first = self.find_number(&chars, false);
        let chars: Vec<char> = rhs.chars().rev().collect();
        let last = self.find_number(&chars, true);

        let val: i64 = match (first, last) {
            (Some(first), Some(last)) => {
                let s = format!("{}{}", first, last);
                s.parse().map_err(|_| ())
            }
            _ => Err(()),
        }?;
        Ok(Self {
            parse_words: self.parse_words,
            val: self.val + val,
        })
    }
}

pub struct DayOneSolver {}

impl Solver<'_> for DayOneSolver {
    fn part_1<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let calibration_value = CalibrationValue {
            parse_words: false,
            val: 0,
        };
        let res = input
            .iter()
            .try_fold(calibration_value, |acc, &c| (acc + c))?;

        Ok(res.val.to_string())
    }

    fn part_2<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let calibration_value = CalibrationValue {
            parse_words: true,
            val: 0,
        };
        let res = input
            .iter()
            .try_fold(calibration_value, |acc, &c| (acc + c))?;

        Ok(res.val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{normalize_input, Solver};

    use super::DayOneSolver;

    #[test]
    fn part_one_example() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let input = normalize_input(input).unwrap();

        let solver = DayOneSolver {};

        let solution: i64 = solver.part_1(&input).unwrap().parse().unwrap();

        assert_eq!(solution, 142);
    }

    #[test]
    fn part_two_example() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let input = normalize_input(input).unwrap();

        let solver = DayOneSolver {};

        let solution: i64 = solver.part_2(&input).unwrap().parse().unwrap();

        assert_eq!(solution, 281);
    }
}
