use aoc::Direction;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("../input1.txt");

struct Grid {
    cells: Vec<Vec<i32>>,
    width: isize,
    height: isize,
    trailheads: Vec<(isize, isize)>,
}

type PathsTuple = (Vec<Vec<(isize, isize)>>, Vec<Vec<(isize, isize)>>);

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        let height = cells.len() as isize;
        let width = cells.first().map_or(0, |row| row.len()) as isize;

        let trailheads: Vec<(isize, isize)> = cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &cell)| (cell == 0).then_some((x as isize, y as isize)))
            })
            .collect();

        Self {
            cells,
            width,
            height,
            trailheads,
        }
    }

    fn get(&self, pos: (isize, isize)) -> i32 {
        self.cells[pos.1 as usize][pos.0 as usize]
    }

    fn in_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn find_paths(&self, start: (isize, isize)) -> PathsTuple {
        let mut paths = Vec::new();
        let mut paths_part2 = Vec::new();
        let mut current_path = vec![start];

        self.find_paths_recursive(start, &mut current_path, &mut paths, &mut paths_part2);

        (paths, paths_part2)
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
            if !self.in_bounds(next_pos) {
                continue;
            }

            let next_value = self.get(next_pos);
            if next_value != current_value + 1 {
                continue;
            }

            if current_path.contains(&next_pos) {
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
            .map(|trailhead| {
                let (paths, paths_part2) = self.find_paths(*trailhead);
                (paths.len(), paths_part2.len())
            })
            .reduce(
                || (0, 0),
                |(acc_paths, acc_paths_part2), (new_paths_len, new_paths_part2_len)| {
                    (
                        acc_paths + new_paths_len,
                        acc_paths_part2 + new_paths_part2_len,
                    )
                },
            )
    }
}

fn main() {
    let grid = Grid::new(INPUT.trim());
    println!("Part 1: {}", grid.n_valid_paths().0);
    println!("Part 2: {}", grid.n_valid_paths().1);
}
