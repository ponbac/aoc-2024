use fxhash::FxHashMap as HashMap;
use std::{num::ParseIntError, str::FromStr, time::Instant};

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Stone(u64);

impl Stone {
    fn split(&self) -> Vec<Stone> {
        match self.0 {
            0 => vec![Stone(1)],
            n if n.to_string().len() % 2 == 0 => {
                let s = n.to_string();
                let (left, right) = s.split_at(s.len() / 2);
                vec![Stone(left.parse().unwrap()), Stone(right.parse().unwrap())]
            }
            _ => vec![Stone(self.0 * 2024)],
        }
    }
}

fn blink(stones: &HashMap<Stone, u64>) -> HashMap<Stone, u64> {
    let mut new_stones = HashMap::default();
    for (&stone, &count) in stones.iter() {
        for new_stone in stone.split() {
            new_stones
                .entry(new_stone)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }
    new_stones
}

fn main() {
    let start = Instant::now();

    let stones_input = INPUT
        .split_whitespace()
        .map(|s| s.parse::<Stone>().unwrap())
        .collect::<Vec<_>>();

    let mut stones_map: HashMap<Stone, u64> = HashMap::default();
    for stone in stones_input {
        *stones_map.entry(stone).or_insert(0) += 1;
    }

    let part1_stones = (0..25).fold(stones_map.clone(), |stones_acc, _| blink(&stones_acc));
    let part2_stones = (0..75).fold(stones_map, |stones_acc, _| blink(&stones_acc));
    println!("Part 1: {}", part1_stones.values().sum::<u64>());
    println!("Part 2: {}", part2_stones.values().sum::<u64>());

    println!("Time: {:?}", start.elapsed());
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
