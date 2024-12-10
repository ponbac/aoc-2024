use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("../input1.txt");

struct Line {
    sum: usize,
    numbers: Vec<usize>,
}

impl Line {
    fn try_combinations(&self, current: usize, pos: usize, part2: bool) -> bool {
        if pos >= self.numbers.len() {
            return current == self.sum;
        }

        let next = self.numbers[pos];
        if self.try_combinations(current + next, pos + 1, part2) {
            return true;
        }
        if self.try_combinations(current * next, pos + 1, part2) {
            return true;
        }
        if part2 {
            let concat = format!("{}{}", current, next).parse::<usize>().unwrap_or(0);
            return self.try_combinations(concat, pos + 1, part2);
        }

        false
    }

    fn is_valid(&self, part2: bool) -> bool {
        self.try_combinations(self.numbers[0], 1, part2)
    }
}

fn solve(lines: &[Line], part2: bool) -> usize {
    lines
        .par_iter()
        .filter(|line| line.is_valid(part2))
        .map(|line| line.sum)
        .sum()
}

fn main() {
    let start = Instant::now();
    let lines: Vec<Line> = INPUT
        .lines()
        .map(|line| {
            let (sum, numbers) = line.split_once(':').unwrap();
            let sum = sum.parse().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            Line { sum, numbers }
        })
        .collect();

    println!("Part 1: {}", solve(&lines, false));
    println!("Part 2: {}", solve(&lines, true));
    println!("Time: {:?}", start.elapsed());
}
