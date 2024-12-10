use aoc::Direction;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<i32>>,
    width: isize,
    height: isize,
    trailheads: Vec<(isize, isize)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                    .collect()
            })
            .collect();

        let height = cells.len() as isize;
        let width = cells.first().map_or(0, Vec::len) as isize;

        let trailheads = cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == 0)
                    .map(move |(x, _)| (x as isize, y as isize))
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
        self.cells[y as usize][x as usize]
    }

    fn in_bounds(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn find_paths(&self, start: (isize, isize)) -> (usize, usize) {
        let mut paths = Vec::new();
        let mut paths_part2 = Vec::new();
        let mut current_path = vec![start];

        self.find_paths_recursive(start, &mut current_path, &mut paths, &mut paths_part2);
        (paths.len(), paths_part2.len())
    }

    fn find_paths_recursive(
        &self,
        current: (isize, isize),
        current_path: &mut Vec<(isize, isize)>,
        all_paths: &mut Vec<Vec<(isize, isize)>>,
        all_paths_part2: &mut Vec<Vec<(isize, isize)>>,
    ) {
        if self.get(current) == 9 {
            all_paths_part2.push(current_path.clone());
            if !all_paths
                .iter()
                .any(|path| path.last() == current_path.last())
            {
                all_paths.push(current_path.clone());
            }
            return;
        }

        let current_value = self.get(current);

        for dir in Direction::ALL {
            let next_pos = current + dir;
            if !self.in_bounds(next_pos)
                || self.get(next_pos) != current_value + 1
                || current_path.contains(&next_pos)
            {
                continue;
            }

            current_path.push(next_pos);
            self.find_paths_recursive(next_pos, current_path, all_paths, all_paths_part2);
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
