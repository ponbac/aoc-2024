use fxhash::FxHashSet as HashSet;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
    visited: HashSet<((isize, isize), Direction)>,
    start_position: (isize, isize),
}

#[derive(Debug, PartialEq)]
enum GuardState {
    OutOfBounds,
    InLoop,
}

impl Guard {
    fn new(start_position: (isize, isize)) -> Self {
        let mut visited = HashSet::default();
        visited.insert((start_position, Direction::Up));

        Self {
            position: start_position,
            direction: Direction::Up,
            visited,
            start_position,
        }
    }

    fn move_in_curr_direction(&mut self) {
        self.position = self.position + self.direction;
        self.visited.insert((self.position, self.direction));
    }

    fn reset(&mut self) {
        self.position = self.start_position;
        self.visited.clear();
        self.visited.insert((self.start_position, Direction::Up));
        self.direction = Direction::Up;
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
        match self.move_guard() {
            GuardState::InLoop => GuardState::InLoop,
            GuardState::OutOfBounds => GuardState::OutOfBounds,
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
        let mut next_pos = self.guard.position + self.guard.direction;
        while self.is_in_bounds(next_pos) {
            if self.get(next_pos) == '#' {
                self.guard.direction = self.guard.direction.turn_right();
                return self.move_guard();
            }

            let state = (next_pos, self.guard.direction);
            if self.guard.visited.contains(&state) {
                return GuardState::InLoop;
            }

            self.guard.move_in_curr_direction();
            next_pos = self.guard.position + self.guard.direction;
        }

        GuardState::OutOfBounds
    }
}

fn main() {
    let start = Instant::now();
    let grid = Grid::parse(INPUT.trim());

    let mut part1_grid = grid.clone();
    part1_grid.run();
    let visited_positions: HashSet<_> = part1_grid
        .guard
        .visited
        .iter()
        .map(|((x, y), _)| (*x, *y))
        .collect();
    println!("Part 1: {}", visited_positions.len());

    let valid_positions: Vec<_> = visited_positions
        .into_par_iter()
        .filter(|&pos| grid.get(pos) == '.' && pos != grid.guard.start_position)
        .filter_map(|obstacle| {
            let mut grid_clone = grid.clone();
            grid_clone.set_obstacle(obstacle);
            grid_clone.guard.reset();
            (grid_clone.run() == GuardState::InLoop).then_some(obstacle)
        })
        .collect();
    println!("Part 2: {}", valid_positions.len());
    println!("Time: {:?}", start.elapsed());
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
