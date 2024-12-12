use std::collections::VecDeque;

use aoc::Direction;
use fxhash::FxHashSet as HashSet;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug)]
struct Region {
    id: char,
    cells: Vec<(isize, isize)>,
}

impl Region {
    fn cost(&self) -> isize {
        let mut perimeter = 0;
        for &(x, y) in &self.cells {
            let adjacent = [
                (x + 1, y),
                (x.wrapping_sub(1), y),
                (x, y + 1),
                (x, y.wrapping_sub(1)),
            ];

            for adj in adjacent {
                if !self.cells.contains(&adj) {
                    perimeter += 1;
                }
            }
        }
        let area = self.cells.len() as isize;
        area * perimeter
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

            for dir in Direction::ALL {
                let (dx, dy) = dir.as_step();
                queue.push_back((x + dx, y + dy));
            }
        }

        Region { id, cells }
    }
}

fn process(input: &str) -> isize {
    let garden = Garden::new(input);
    let regions = garden.regions();

    regions.iter().map(|r| r.cost()).sum()
}

fn main() {
    println!("Part 1: {}", process(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{process, Region};

    const EXAMPLE_1: &str = r#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;

    const EXAMPLE_2: &str = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn test_example_1() {
        assert_eq!(process(EXAMPLE_1), 772);
    }

    // #[test]
    // fn test_example_2() {
    //     assert_eq!(process(EXAMPLE_2), 1930);
    // }

    #[test]
    fn test_region_cost() {
        // A region that looks like: AAAA (4 cells in a row)
        let region = Region {
            id: 'A',
            cells: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        };
        assert_eq!(region.cost(), 40); // area=4, perimeter=10

        // A single cell region
        let region = Region {
            id: 'D',
            cells: vec![(0, 0)],
        };
        assert_eq!(region.cost(), 4); // area=1, perimeter=4

        // A 2x2 square region
        let region = Region {
            id: 'B',
            cells: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };
        assert_eq!(region.cost(), 32); // area=4, perimeter=8

        // Irregular region
        let region = Region {
            id: 'C',
            cells: vec![(0, 0), (0, 1), (1, 1), (1, 2)],
        };
        assert_eq!(region.cost(), 40); // area=4, perimeter=10
    }
}
