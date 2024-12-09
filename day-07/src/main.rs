use std::time::Instant;

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
        let add = self.try_combinations(current + next, pos + 1, part2);
        let multiply = self.try_combinations(current * next, pos + 1, part2);

        if !part2 {
            return add || multiply;
        }

        let concat = format!("{}{}", current, next).parse::<usize>().unwrap_or(0);
        add || multiply || self.try_combinations(concat, pos + 1, part2)
    }

    fn is_valid(&self, part2: bool) -> bool {
        self.try_combinations(self.numbers[0], 1, part2)
    }
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

    println!(
        "Part 1: {}",
        lines
            .iter()
            .filter(|line| line.is_valid(false))
            .map(|line| line.sum)
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        lines
            .iter()
            .filter(|line| line.is_valid(true))
            .map(|line| line.sum)
            .sum::<usize>()
    );

    println!("Time: {:?}", start.elapsed());
}
