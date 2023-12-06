use std::collections::{HashMap, HashSet};

use super::Solver;

struct Symbol {
    symbol: char,
    parts: HashSet<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}

#[derive(Clone, Default)]
struct Part {
    val: u64,
    symbols: HashSet<usize>,
}

pub struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl<'a> Schematic {
    fn parse_from_strs(s: &'a [&'a str]) -> Schematic {
        // find valid locations
        let (valid_locations, symbols) = Self::find_valid_locations(s);
        let (parts, symbols) = Self::find_parts(s, &valid_locations, symbols);

        Schematic { parts, symbols }
    }

    fn find_valid_locations(s: &'a [&'a str]) -> (HashMap<Coordinate, Vec<usize>>, Vec<Symbol>) {
        let mut valid_locations: HashMap<Coordinate, Vec<usize>> = HashMap::new();
        let mut symbols = Vec::new();
        for (y, row_val) in s.iter().enumerate() {
            for (x, c) in row_val.char_indices() {
                if c.is_numeric() || c == '.' {
                    continue;
                }

                symbols.push(Symbol {
                    symbol: c,
                    parts: HashSet::new(),
                });
                let dirs = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];

                // must be a symbol
                let valid_coordinates: Vec<Coordinate> = dirs
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;

                        if new_x >= 0
                            && new_x < s[y].len() as isize
                            && new_y >= 0
                            && new_y < s.len() as isize
                        {
                            Some(Coordinate {
                                x: new_x as u64,
                                y: new_y as u64,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                for coord in valid_coordinates {
                    valid_locations
                        .entry(coord)
                        .or_insert_with(Vec::new)
                        .push(symbols.len() - 1);
                }
            }
        }

        (valid_locations, symbols)
    }

    fn find_parts(
        s: &'a [&'a str],
        valid_locations: &HashMap<Coordinate, Vec<usize>>,
        mut symbols: Vec<Symbol>,
    ) -> (Vec<Part>, Vec<Symbol>) {
        let mut parts: Vec<Part> = vec![];
        let mut is_part = false;
        let mut part_symbols: HashSet<usize> = HashSet::new();
        let mut val_string: String = String::default();

        // find parts
        for y in 0..s.len() {
            for (x, c) in s[y].char_indices() {
                let x = x as u64;
                let y = y as u64;

                if c.is_numeric() {
                    if !is_part {
                        is_part = true;
                    }

                    if let Some(symbol_indices) = valid_locations.get(&Coordinate { x, y }) {
                        part_symbols.extend(symbol_indices);
                        for symbol_idx in symbol_indices {
                            symbols
                                .get_mut(*symbol_idx)
                                .unwrap()
                                .parts
                                .insert(parts.len());
                        }
                    }
                    val_string.push(c);
                } else {
                    if is_part {
                        parts.push(Part {
                            val: val_string.parse().unwrap(),
                            symbols: part_symbols.clone().into_iter().collect(),
                        });
                    }

                    is_part = false;
                    part_symbols.clear();
                    val_string.clear();
                }
            }
        }

        if is_part {
            parts.push(Part {
                val: val_string.parse().unwrap(),
                symbols: part_symbols.into_iter().collect(),
            });
        }

        (parts, symbols)
    }

    fn get_gear_ratio(&self, symbol: &Symbol) -> Option<u64> {
        if symbol.symbol == '*' && symbol.parts.len() == 2 {
            let parts: Vec<&usize> = symbol.parts.iter().collect();
            Some(self.parts.get(*parts[0]).unwrap().val * self.parts.get(*parts[1]).unwrap().val)
        } else {
            None
        }
    }
}

pub struct DayThreeSolver {}

impl Solver<'_> for DayThreeSolver {
    fn part_1<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let schematic = Schematic::parse_from_strs(input);
        let sum: u64 = schematic
            .parts
            .iter()
            .filter(|p| !p.symbols.is_empty())
            .map(|p| p.val)
            .sum();

        Ok(sum.to_string())
    }

    fn part_2<'a>(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let schematic = Schematic::parse_from_strs(input);

        let sum: u64 = schematic
            .symbols
            .iter()
            .filter_map(|s| schematic.get_gear_ratio(s))
            .sum();

        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{normalize_input, Solver};

    use super::DayThreeSolver;

    const INPUT: &'static str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    #[test]
    fn part_one_example() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DayThreeSolver {};
        let res: i64 = solver.part_1(&input).unwrap().parse().unwrap();

        assert_eq!(res, 4361);
    }

    #[test]
    fn part_two_example() {
        let input = normalize_input(INPUT).unwrap();
        let solver = DayThreeSolver {};
        let res: i64 = solver.part_2(&input).unwrap().parse().unwrap();

        assert_eq!(res, 467835);
    }
}
