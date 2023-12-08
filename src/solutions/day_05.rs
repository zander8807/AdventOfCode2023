use std::{collections::HashMap, ops::Range, str::FromStr};

use super::Solver;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum MapsIdentifier {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for MapsIdentifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil" => Ok(MapsIdentifier::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapsIdentifier::SoilToFertilizer),
            "fertilizer-to-water" => Ok(MapsIdentifier::FertilizerToWater),
            "water-to-light" => Ok(MapsIdentifier::WaterToLight),
            "light-to-temperature" => Ok(MapsIdentifier::LightToTemperature),
            "temperature-to-humidity" => Ok(MapsIdentifier::TemperatureToHumidity),
            "humidity-to-location" => Ok(MapsIdentifier::HumidityToLocation),
            _ => Err(()),
        }
    }
}

struct Almanac {
    maps: HashMap<MapsIdentifier, Vec<(Range<u64>, u64)>>,
}

impl Almanac {
    fn find_dest(&self, seed: u64) -> u64 {
        [
            MapsIdentifier::SeedToSoil,
            MapsIdentifier::SoilToFertilizer,
            MapsIdentifier::FertilizerToWater,
            MapsIdentifier::WaterToLight,
            MapsIdentifier::LightToTemperature,
            MapsIdentifier::TemperatureToHumidity,
            MapsIdentifier::HumidityToLocation,
        ]
            .iter()
            .map(|maps_identifier| self.maps.get(maps_identifier).unwrap())
            .fold(seed, |last_dest, maps| {
                maps.iter()
                    .find_map(|(range, dest)| {
                        if range.contains(&last_dest) {
                            Some(last_dest - range.start + dest)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(last_dest)
            })
    }

    fn find_min_in_seed_range(&self, left: u64, right: u64, min: u64) -> u64 {
        let left_val = self.find_dest(left);
        let right_val = self.find_dest(right);

        let is_linearly_increasing = right_val > left_val && right_val - left_val == right - left;
        if is_linearly_increasing || left == right {
            // since a range should be linearly increasing, we can stop execution once we find a case where
            // the difference between the two values is equal to the range size
            return min.min(left_val);
        };

        let mid_point = (right - left) / 2;
        let left_min = self.find_min_in_seed_range(left, left + mid_point, min);
        let right_min = self.find_min_in_seed_range(left + mid_point + 1, right, min);
        left_min.min(right_min)
    }

    fn new_from_lines<'a>(lines: &'a [&'a str]) -> Almanac {
        let mut maps: HashMap<MapsIdentifier, Vec<(Range<u64>, u64)>> = HashMap::new();
        let mut map_identifier: Option<MapsIdentifier> = None;
        for line in lines {
            if line.is_empty() {
                // end of last map
            } else if line.ends_with(':') {
                // slicing to only get the map identifier (removing " map:")
                map_identifier = Some(line[0..line.len() - 5].parse().unwrap());
            } else {
                let mut vals = line.split_ascii_whitespace();
                let (dest, source, range): (u64, u64, u64) = (
                    vals.next().unwrap().parse().unwrap(),
                    vals.next().unwrap().parse().unwrap(),
                    vals.next().unwrap().parse().unwrap(),
                );
                maps.entry(map_identifier.clone().expect("shouldn't be none"))
                    .or_default()
                    .push((source..(source + range), dest))
            }
        }

        maps.values_mut()
            .into_iter()
            .for_each(|map| map.sort_by_key(|k| k.0.start));
        Almanac { maps }
    }
}

pub struct DayFiveSolver {}

impl<'a> Solver<'a> for DayFiveSolver {
    fn part_1(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let seeds: Vec<u64> = input[0]
            .split(':')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse().map_err(|_| ()))
            .collect::<Result<Vec<u64>, ()>>()?;

        let almanac = Almanac::new_from_lines(&input[1..]);

        let res = seeds
            .iter()
            .map(|seed| {
                let dest = almanac.find_dest(*seed);
                println!("seed {} goes to location {}", seed, dest);
                dest
            })
            .min()
            .unwrap();

        Ok(res.to_string())
    }

    fn part_2(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let seed_ranges: Vec<(u64, u64)> = input[0]
            .split(':')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse().map_err(|_| ()))
            .collect::<Result<Vec<u64>, ()>>()?
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let almanac = Almanac::new_from_lines(&input[1..]);

        let mut min = u64::MAX;
        for (seed_start, range) in seed_ranges {
            min = min.min(almanac.find_min_in_seed_range(seed_start, seed_start + range, u64::MAX))
        }

        Ok(min.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{normalize_input, Solver};

    use super::DayFiveSolver;

    const INPUT: &str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    ";

    #[test]
    fn part_1_example() {
        let input = normalize_input(INPUT).unwrap();

        let solver = DayFiveSolver {};
        let res = solver.part_1(&input).unwrap();

        assert_eq!(res, "35");
    }

    #[test]
    fn part_2_example() {
        let input = normalize_input(INPUT).unwrap();

        let solver = DayFiveSolver {};
        let res = solver.part_2(&input).unwrap();

        assert_eq!(res, "46");
    }
}
