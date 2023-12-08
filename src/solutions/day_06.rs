use super::Solver;

pub struct DaySixSolver {}

fn find_solutions(distance: u64, total_duration: u64) -> Option<u64> {
    let a = 1.0;
    let b = -(total_duration as f64);
    let c = distance as f64;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real roots
        None
    } else {
        let t_start = (-b - discriminant.sqrt()) / (2.0 * a);
        let t_end = (-b + discriminant.sqrt()) / (2.0 * a);

        let t_start = t_start;
        let t_end = t_end;

        Some(t_end.floor() as u64 - t_start.ceil() as u64 + 1)
    }
}

impl<'a> Solver<'a> for DaySixSolver {
    fn part_1(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let times = input[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let distances = input[1]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let res: u64 = times
            .iter()
            .zip(distances.iter())
            .map(|(&t, &d)| {
                // We want to beat the previous record by at least one.
                let target_distance = d + 1;
                find_solutions(target_distance, t).unwrap()
            })
            .reduce(|acc, solution| acc * solution)
            .unwrap();

        Ok(res.to_string())
    }

    fn part_2(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let time: u64 = input[0]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();
        let distance: u64 = input[1]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();

        let res = find_solutions(distance + 1, time).unwrap();

        Ok(res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::normalize_input;

    const INPUT: &str = "
    Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DaySixSolver {};

        let res = solver.part_1(&input).unwrap();

        assert_eq!(res, "288".to_string());
    }

    #[test]
    fn test_part_2() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DaySixSolver {};

        let res = solver.part_2(&input).unwrap();

        assert_eq!(res, "71503".to_string());
    }
}
