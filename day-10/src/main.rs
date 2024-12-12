use aoc::Direction;
use fxhash::FxHashSet as HashSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug)]
struct Grid {
    cells: Vec<i32>,
    width: isize,
    height: isize,
    trailheads: Vec<(isize, isize)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                    .collect()
            })
            .collect();

        let height = rows.len() as isize;
        let width = rows.first().map_or(0, Vec::len) as isize;
        let cells: Vec<i32> = rows.into_iter().flatten().collect();

        let trailheads = cells
            .iter()
            .enumerate()
            .filter(|(_, &cell)| cell == 0)
            .map(|(i, _)| {
                let x = (i as isize) % width;
                let y = (i as isize) / width;
                (x, y)
            })
            .collect();

        Self {
            cells,
            width,
            height,
            trailheads,
        }
    }

    fn get(&self, (x, y): (isize, isize)) -> i32 {
        self.cells[(y * self.width + x) as usize]
    }

    fn in_bounds(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn find_paths(&self, start: (isize, isize)) -> (usize, usize) {
        let mut current_path = vec![start];
        let mut seen_endpoints = HashSet::default();
        let mut counts = (0, 0);
        self.find_paths_recursive(start, &mut current_path, &mut seen_endpoints, &mut counts);
        counts
    }

    fn find_paths_recursive(
        &self,
        current: (isize, isize),
        current_path: &mut Vec<(isize, isize)>,
        seen_endpoints: &mut HashSet<(isize, isize)>,
        counts: &mut (usize, usize),
    ) {
        if self.get(current) == 9 {
            counts.1 += 1; // Part 2
            if seen_endpoints.insert(current) {
                counts.0 += 1; // Part 1
            }
            return;
        }

        let current_value = self.get(current);
        for dir in Direction::ALL_BASIC {
            let next_pos = current + dir;
            if !self.in_bounds(next_pos)
                || self.get(next_pos) != current_value + 1
                || current_path.contains(&next_pos)
            {
                continue;
            }

            current_path.push(next_pos);
            self.find_paths_recursive(next_pos, current_path, seen_endpoints, counts);
            current_path.pop();
        }
    }

    fn n_valid_paths(&self) -> (usize, usize) {
        self.trailheads
            .par_iter()
            .map(|&start| self.find_paths(start))
            .reduce(
                || (0, 0),
                |(acc_paths, acc_paths2), (paths, paths2)| (acc_paths + paths, acc_paths2 + paths2),
            )
    }
}

fn main() {
    let start = Instant::now();
    let grid = Grid::new(INPUT.trim());
    let (part1, part2) = grid.n_valid_paths();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("Time: {:?}", start.elapsed());
}
