use std::collections::HashMap;
use std::{fmt::Display, num::ParseIntError, str::FromStr, time::Instant};

const INPUT: &str = include_str!("../input1.txt");

const EXAMPLE: &str = "125 17";

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (&value, &count) in stones.iter() {
        match value {
            0 => {
                *new_stones.entry(1).or_insert(0) += count;
            }
            n if n.to_string().len() % 2 == 0 => {
                let s = n.to_string();
                let (left, right) = s.split_at(s.len() / 2);
                let left_val = left.parse::<u64>().unwrap();
                let right_val = right.parse::<u64>().unwrap();
                new_stones
                    .entry(left_val)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                new_stones
                    .entry(right_val)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
            _ => {
                *new_stones.entry(value * 2024).or_insert(0) += count;
            }
        }
    }
    new_stones
}

fn main() {
    let start = Instant::now();

    let stones_input = INPUT
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    for stone in stones_input {
        *stones_map.entry(stone).or_insert(0) += 1;
    }

    let final_stones = (0..75).fold(stones_map, |stones, _| blink(&stones));
    println!("Part 1: {}", final_stones.values().sum::<u64>());

    println!("Time: {:?}", start.elapsed());
}

// impl FromStr for Stone {
//     type Err = ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self(s.parse().unwrap()))
//     }
// }

// impl Display for Stone {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
