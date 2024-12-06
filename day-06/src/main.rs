use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

const INPUT: &str = include_str!("../input1.txt");

const EXAMPLE_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

#[derive(Debug, Clone)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
    visited: HashSet<((isize, isize), Direction)>,
    start_position: (isize, isize),
}

#[derive(Debug, PartialEq)]
enum GuardState {
    Ok,
    OutOfBounds,
    InLoop,
}

impl Guard {
    fn new(start_position: (isize, isize)) -> Self {
        let mut visited = HashSet::new();
        visited.insert((start_position, Direction::Up));

        Self {
            position: start_position,
            direction: Direction::Up,
            visited,
            start_position,
        }
    }

    fn move_in_direction(&mut self, direction: Direction) {
        self.position = self.position + direction;
        self.visited.insert((self.position, direction));
    }

    fn reset(&mut self) {
        self.position = self.start_position;
        self.visited.clear();
        self.visited.insert((self.start_position, Direction::Up));
        self.direction = Direction::Up;
    }

    fn n_visited(&self) -> usize {
        self.visited
            .iter()
            .map(|((x, y), _)| (*x, *y))
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<char>,
    width: isize,
    height: isize,
    guard: Guard,
    obstacle: (isize, isize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as isize;
        let width = lines[0].len() as isize;

        let cells = vec!['.'; (width * height) as usize];
        let mut grid = Self {
            cells,
            width,
            height,
            guard: Guard::new((0, 0)),
            obstacle: (-1, -1),
        };

        let mut start_position = (0, 0);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as isize, y as isize);
                match c {
                    '^' => start_position = pos,
                    c => grid.set(pos, c),
                }
            }
        }

        grid.guard = Guard::new(start_position);
        grid
    }

    fn set(&mut self, position: (isize, isize), value: char) {
        self.cells[(position.1 * self.width + position.0) as usize] = value;
    }

    fn get(&self, position: (isize, isize)) -> char {
        self.cells[(position.1 * self.width + position.0) as usize]
    }

    fn run(&mut self) -> GuardState {
        loop {
            match self.move_guard() {
                GuardState::Ok => continue,
                state => return state,
            }
        }
    }

    fn set_obstacle(&mut self, position: (isize, isize)) {
        self.set(position, '#');
        self.obstacle = position;
    }

    fn is_in_bounds(&self, position: (isize, isize)) -> bool {
        position.0 >= 0 && position.0 < self.width && position.1 >= 0 && position.1 < self.height
    }

    fn move_guard(&mut self) -> GuardState {
        let new_position = self.guard.position + self.guard.direction;
        if !self.is_in_bounds(new_position) {
            return GuardState::OutOfBounds;
        }

        if self.get(new_position) == '#' {
            self.guard.direction = self.guard.direction.turn_right();
            self.move_guard()
        } else {
            let state = (new_position, self.guard.direction);
            if self.guard.visited.contains(&state) {
                return GuardState::InLoop;
            }

            self.guard.move_in_direction(self.guard.direction);
            GuardState::Ok
        }
    }
}

fn main() {
    let grid = Grid::parse(INPUT.trim());

    // Part 1
    let mut part1_grid = grid.clone();
    part1_grid.run();
    println!("{}", part1_grid.guard.n_visited());

    // Part 2
    let valid_positions: Vec<_> = (0..grid.width)
        .into_par_iter()
        .flat_map(|x| (0..grid.height).into_par_iter().map(move |y| (x, y)))
        .filter(|&pos| grid.get(pos) == '.' && pos != grid.guard.start_position)
        .filter_map(|obstacle| {
            let mut grid_clone = grid.clone();
            grid_clone.set_obstacle(obstacle);
            grid_clone.guard.reset();
            (grid_clone.run() == GuardState::InLoop).then_some(obstacle)
        })
        .collect();

    println!("Found {} valid obstacle positions", valid_positions.len());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_step(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn add(self, rhs: Direction) -> Self::Output {
        let step = rhs.as_step();
        (self.0 + step.0, self.1 + step.1)
    }
}
