use std::{fmt::Display, num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input1.txt");

struct Stone(u64);

impl Stone {
    fn split(&self) -> Vec<Self> {
        match self.0 {
            0 => vec![Self(1)],
            n if n.to_string().len() % 2 == 0 => {
                let s = n.to_string();
                let (left, right) = s.split_at(s.len() / 2);
                vec![Self(left.parse().unwrap()), Self(right.parse().unwrap())]
            }
            _ => vec![Self(self.0 * 2024)],
        }
    }
}

fn blink(stones: &[Stone]) -> Vec<Stone> {
    stones.iter().flat_map(|s| s.split()).collect()
}

fn main() {
    let stones = INPUT
        .split_whitespace()
        .map(|s| s.parse::<Stone>().unwrap())
        .collect::<Vec<_>>();

    let final_stones = (0..75).fold(stones, |stones, _| blink(&stones));
    println!("Part 1: {}", final_stones.len());
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().unwrap()))
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
