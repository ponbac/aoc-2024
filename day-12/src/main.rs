use std::collections::VecDeque;

use aoc::Direction;
use fxhash::FxHashSet as HashSet;

const INPUT: &str = include_str!("../input1.txt");

struct Region {
    cells: Vec<(isize, isize)>,
}

impl Region {
    fn cost(&self) -> isize {
        let mut perimeter = 0;
        for &pos in &self.cells {
            for dir in Direction::ALL_BASIC {
                if !self.cells.contains(&(pos + dir)) {
                    perimeter += 1;
                }
            }
        }

        let area = self.cells.len() as isize;
        area * perimeter
    }

    fn cost_2(&self) -> isize {
        let mut corners = 0;
        for &pos in &self.cells {
            let adjacent: Vec<bool> = Direction::ALL_BASIC
                .iter()
                .map(|&dir| self.cells.contains(&(pos + dir)))
                .collect();
            let diagonal: Vec<bool> = Direction::ALL_DIAGONAL
                .iter()
                .map(|&dir| self.cells.contains(&(pos + dir)))
                .collect();

            for i in 0..4 {
                if !adjacent[i] && !adjacent[(i + 1) % 4] {
                    // Convex corner: two adjacent cells are empty
                    corners += 1;
                } else if adjacent[i] && adjacent[(i + 1) % 4] && !diagonal[i] {
                    // Concave corner: two adjacent cells are filled but diagonal is empty
                    corners += 1;
                }
            }
        }

        let area = self.cells.len() as isize;
        area * corners
    }
}

struct Garden {
    grid: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Garden {
    fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len() as isize;
        let height = grid.len() as isize;

        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> char {
        self.grid[y as usize][x as usize]
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn regions(&self) -> Vec<Region> {
        let mut explored = HashSet::default();
        let mut regions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if explored.contains(&(x, y)) {
                    continue;
                }

                let region = self.find_region(x, y, &mut explored);
                regions.push(region);
            }
        }

        regions
    }

    fn find_region(&self, x: isize, y: isize, explored: &mut HashSet<(isize, isize)>) -> Region {
        let mut cells = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((x, y));

        let id = self.get(x, y);
        while let Some((x, y)) = queue.pop_front() {
            if !self.in_bounds(x, y) {
                continue;
            }

            let cell = self.get(x, y);
            if cell != id || explored.contains(&(x, y)) {
                continue;
            }

            explored.insert((x, y));
            cells.push((x, y));

            for dir in Direction::ALL_BASIC {
                let (dx, dy) = dir.as_step();
                queue.push_back((x + dx, y + dy));
            }
        }

        Region { cells }
    }
}

fn main() {
    let garden = Garden::new(INPUT);
    let regions = garden.regions();

    println!(
        "Part 1: {}",
        regions.iter().map(|r| r.cost()).sum::<isize>()
    );
    println!(
        "Part 2: {}",
        regions.iter().map(|r| r.cost_2()).sum::<isize>()
    );
}
