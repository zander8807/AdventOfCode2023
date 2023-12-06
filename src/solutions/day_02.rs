use std::str::FromStr;

use super::Solver;

#[derive(Debug)]
struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split(",").map(|s| s.trim()).collect();

        let mut red: i64 = 0;
        let mut green: i64 = 0;
        let mut blue: i64 = 0;

        for s in split {
            let mut val_color_split = s.split(" ");
            let val: i64 = val_color_split.next().unwrap().parse().unwrap();
            let color = val_color_split.next().unwrap();

            match color {
                "red" => red = val,
                "green" => green = val,
                "blue" => blue = val,
                _ => return Err(()),
            }
        }

        Ok(Set { red, green, blue })
    }
}

struct Game {
    id: i64,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self, set: &Set) -> bool {
        self.sets
            .iter()
            .find(|s| {
                let not_possible = s.red > set.red || s.green > set.green || s.blue > set.blue;
                not_possible
            })
            .is_none()
    }

    fn find_max_set<'a>(&self) -> Set {
        let mut max_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in &self.sets {
            max_set.red = i64::max(max_set.red, set.red);
            max_set.green = i64::max(max_set.green, set.green);
            max_set.blue = i64::max(max_set.blue, set.blue)
        }

        max_set
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(": ").map(|s| s.trim());

        let id: i64 = split.next().unwrap().get(5..).unwrap().parse().unwrap();
        let sets: Vec<Set> = split
            .next()
            .unwrap()
            .split("; ")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Game { id, sets })
    }
}

pub struct DayTwoSolver {}

impl Solver<'_> for DayTwoSolver {
    fn part_1<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let set = Set {
            red: 12,
            green: 13,
            blue: 14,
        };
        let games = input
            .iter()
            .map(|&s| s.parse())
            .collect::<Result<Vec<Game>, ()>>()?;

        Ok(games
            .iter()
            .fold(0, |acc, g| acc + if g.is_possible(&set) { g.id } else { 0 })
            .to_string())
    }

    fn part_2<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let games = input
            .iter()
            .map(|&s| s.parse())
            .collect::<Result<Vec<Game>, ()>>()?;

        let cube_sum: i64 = games
            .iter()
            .map(|g| {
                let max_set = g.find_max_set();
                max_set.red * max_set.green * max_set.blue
            })
            .sum();

        Ok(cube_sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{normalize_input, Solver};

    use super::DayTwoSolver;

    #[test]
    fn part_one_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let input = normalize_input(input).unwrap();

        let solver = DayTwoSolver {};
        let res: i64 = solver.part_1(&input).unwrap().parse().unwrap();

        assert_eq!(res, 8);
    }

    #[test]
    fn part_two_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let input = normalize_input(input).unwrap();

        let solver = DayTwoSolver {};
        let res: i64 = solver.part_2(&input).unwrap().parse().unwrap();

        assert_eq!(res, 2286);
    }
}
