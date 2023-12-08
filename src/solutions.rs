pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

pub trait Solver<'a> {
    fn part_1(&self, input: &'a [&'a str]) -> Result<String, ()>;
    fn part_2(&self, input: &'a [&'a str]) -> Result<String, ()>;
}

pub fn normalize_input(input: &str) -> Result<Vec<&str>, ()> {
    let lines = input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(lines)
}
