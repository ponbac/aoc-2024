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
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = grid.len() as isize;
        let width = grid.first().map_or(0, |row| row.len()) as isize;

        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, pos: (isize, isize)) -> Option<char> {
        let (x, y) = pos;
        if self.in_bounds(pos) {
            Some(self.grid[y as usize][x as usize])
        } else {
            None
        }
    }

    fn in_bounds(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn regions(&self) -> Vec<Region> {
        let mut explored = HashSet::default();
        let mut regions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if explored.contains(&pos) {
                    continue;
                }

                if let Some(region) = self.find_region(pos, &mut explored) {
                    regions.push(region);
                }
            }
        }

        regions
    }

    fn find_region(
        &self,
        start: (isize, isize),
        explored: &mut HashSet<(isize, isize)>,
    ) -> Option<Region> {
        let mut cells = Vec::new();
        let start_char = self.get(start)?;

        let mut queue = VecDeque::from([start]);
        while let Some(pos) = queue.pop_front() {
            if explored.contains(&pos) || self.get(pos) != Some(start_char) {
                continue;
            }

            explored.insert(pos);
            cells.push(pos);

            queue.extend(Direction::ALL_BASIC.iter().map(|&dir| pos + dir));
        }

        Some(Region { cells })
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
