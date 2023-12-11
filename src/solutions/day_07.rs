use std::{cmp::Ordering, str::FromStr, collections::BinaryHeap};

use super::Solver;

pub struct DaySevenSolver {}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rank {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Wildcard,
}

impl Rank {
    fn value(&self) -> u32 {
        match self {
            Rank::A => 14,
            Rank::K => 13,
            Rank::Q => 12,
            Rank::J => 11,
            Rank::T => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
            Rank::Wildcard => 1,
        }
    }
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rank::A),
            "K" => Ok(Rank::K),
            "Q" => Ok(Rank::Q),
            "J" => Ok(Rank::J),
            "T" => Ok(Rank::T),
            "9" => Ok(Rank::Nine),
            "8" => Ok(Rank::Eight),
            "7" => Ok(Rank::Seven),
            "6" => Ok(Rank::Six),
            "5" => Ok(Rank::Five),
            "4" => Ok(Rank::Four),
            "3" => Ok(Rank::Three),
            "2" => Ok(Rank::Two),
            "W" => Ok(Rank::Wildcard),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl TryFrom<HandType> for usize {
    type Error = ();

    fn try_from(value: HandType) -> Result<Self, Self::Error> {
        match value {
            HandType::FiveOfAKind => Ok(6),
            HandType::FourOfAKind => Ok(5),
            HandType::FullHouse => Ok(4),
            HandType::ThreeOfAKind => Ok(3),
            HandType::TwoPair => Ok(2),
            HandType::OnePair => Ok(1),
            HandType::HighCard => Ok(0),
        }
    }
}

impl TryFrom<usize> for HandType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(HandType::FiveOfAKind),
            5 => Ok(HandType::FourOfAKind),
            4 => Ok(HandType::FullHouse),
            3 => Ok(HandType::ThreeOfAKind),
            2 => Ok(HandType::TwoPair),
            1 => Ok(HandType::OnePair),
            0 => Ok(HandType::HighCard),
            _ => Err(()),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = usize::try_from(self.clone()).unwrap();
        let b = usize::try_from(other.clone()).unwrap();

        a.cmp(&b)
    }
}

impl From<Vec<Rank>> for HandType {
    fn from(value: Vec<Rank>) -> Self {
        let mut ranks = value;
        ranks.sort();

        let total = ranks.len();
        let ranks = ranks.iter().filter(|r| r != &&Rank::Wildcard).collect::<Vec<_>>();
        let wildcards = total - ranks.len();

        let mut counts = [0; 15];
        for rank in ranks {
            counts[rank.value() as usize] += 1;
        }

        let t = if counts.iter().any(|&c| c == 5) {
            // can't get any higher than 5 of a kind
            return HandType::FiveOfAKind
        } else if counts.iter().any(|&c| c == 4) {
            HandType::FourOfAKind
        } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            HandType::FullHouse
        } else if counts.iter().any(|&c| c == 3) {
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|&&c| c == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.iter().any(|&c| c == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        match wildcards {
            5 => {
                HandType::FiveOfAKind
            },
            4 => {
                HandType::FiveOfAKind
            },
            3 => {
                match t {
                    HandType::OnePair => HandType::FiveOfAKind,
                    _ => HandType::FourOfAKind
                }
            },
            2 => {
                match t {
                    HandType::ThreeOfAKind => HandType::FiveOfAKind,
                    HandType::OnePair => HandType::FourOfAKind,
                    _ => HandType::ThreeOfAKind
                }
            },
            1 => {
                match t {
                    HandType::FourOfAKind => HandType::FiveOfAKind,
                    HandType::ThreeOfAKind => HandType::FourOfAKind,
                    HandType::TwoPair => HandType::FullHouse,
                    HandType::OnePair => HandType::ThreeOfAKind,
                    _ => HandType::OnePair
                }
            },
            _ => {
                t
            }
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Hand {
    bet: u64,
    cards: Vec<Rank>,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split_ascii_whitespace();
        
        let cards: Vec<Rank> = split
            .next()
            .expect("should be two parts...")
            .chars()
            .map(|s| {
                s.to_string().parse()
            })
            .collect::<Result<Vec<Rank>, ()>>()
            .unwrap_or_default();

        let bet = split.next().unwrap().parse().unwrap();

        Ok(Hand { bet, cards })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = HandType::from(self.cards.clone());
        let other_type = HandType::from(other.cards.clone());

        if self_type != other_type {
            self_type.cmp(&other_type)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl<'a> Solver<'a> for DaySevenSolver {
    fn part_1(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let hands: Vec<Hand> = input.iter()
            .map(|s| s.parse().unwrap())
            .collect::<BinaryHeap<Hand>>()
            .into_sorted_vec();

        let res = hands.iter().enumerate()
            .fold(0, |acc, c| {
                acc + (c.0 + 1) as u64 * c.1.bet
            });

        Ok(res.to_string())
    }

    fn part_2(&self, input: &'a [&'a str]) -> Result<String, ()> {
        let hands: Vec<Hand> = input.iter()
            .map(|s| {
                let s = s.replace('J',  "W");
                s.parse().unwrap()
            })
            .collect::<BinaryHeap<Hand>>()
            .into_sorted_vec();

        let res = hands.iter().enumerate()
            .fold(0, |acc, c| {
                acc + (c.0 + 1) as u64 * c.1.bet
            });

        Ok(res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{day_07::{DaySevenSolver}, normalize_input, Solver};

    const INPUT: &str = r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#;

    #[test]
    fn part_1_example() {
        let solver = DaySevenSolver {};
        let input = normalize_input(&INPUT).unwrap();

        let res = solver.part_1(&input).unwrap();
        assert_eq!(res, "6440");
    }

    #[test]
    fn part_2_example() {
        let solver = DaySevenSolver {};
        let input = normalize_input(&INPUT).unwrap();

        let res = solver.part_2(&input).unwrap();
        assert_eq!(res, "5905");
    }
}
