use std::{collections::HashSet, fmt::Display};

use aoc::{Direction, Point};

const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;
const EXAMPLE2: &str = r#"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"#;

struct Grid {
    cells: Vec<Vec<char>>,
    robot: Point,
    boxes: HashSet<Point>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let robot = cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|&c| c == '@')
                    .map(|x| Point::new(x as isize, y as isize))
            })
            .unwrap();
        let boxes = cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 'O')
                    .map(move |(x, _)| Point::new(x as isize, y as isize))
            })
            .collect();

        Self {
            cells,
            robot,
            boxes,
        }
    }

    fn get(&self, point: impl Into<Point>) -> char {
        let point = point.into();
        self.cells[point.y as usize][point.x as usize]
    }

    fn set(&mut self, point: impl Into<Point>, value: char) {
        let point = point.into();
        self.cells[point.y as usize][point.x as usize] = value;
    }

    fn move_robot(&mut self, direction: &Direction) {
        self.set(self.robot, '.');
        self.robot += *direction;
        self.set(self.robot, '@');
    }

    fn move_box(&mut self, pos: &Point, direction: &Direction) {
        let new_pos = *pos + *direction;
        match self.get(new_pos) {
            '#' => (),
            'O' => {
                self.move_box(&new_pos, direction);
                // If the next position is now empty (meaning boxes were successfully moved)
                if self.get(new_pos) == '.' {
                    self.set(*pos, '.');
                    self.set(new_pos, 'O');
                    self.boxes.remove(pos);
                    self.boxes.insert(new_pos);
                }
            }
            _ => {
                self.set(*pos, '.');
                self.set(new_pos, 'O');
                self.boxes.remove(pos);
                self.boxes.insert(new_pos);
            }
        }
    }

    /// If robot is blocked by box, move the box along with the robot if possible.
    fn run_path(&mut self, path: &[Direction]) {
        for direction in path {
            let next_pos = self.robot + *direction;
            match self.get(next_pos) {
                '#' => continue,
                'O' => {
                    self.move_box(&next_pos, direction);
                    if self.get(next_pos) == '.' {
                        self.move_robot(direction);
                    }
                }
                _ => self.move_robot(direction),
            }
        }
    }

    fn run_path_part_2(&mut self, path: &[Direction]) {
        for direction in path {
            let next_pos = self.robot + *direction;
            match self.get(next_pos) {
                '#' => continue,
                '[' | ']' => {
                    todo!()
                }
                _ => self.move_robot(direction),
            }
        }
    }

    fn can_move_wide_box(&self, left_pos: &Point, direction: &Direction) -> bool {
        todo!()
    }

    fn move_wide_box(&mut self, left_pos: &Point, direction: &Direction) {
        todo!()
    }

    fn widen_grid(&mut self) {
        let height = self.cells.len();
        let width = self.cells[0].len();
        let mut new_cells = vec![vec!['.'; width * 2]; height];

        // Process each cell in the grid
        (0..height).for_each(|y| {
            for x in 0..width {
                let new_x = x * 2;
                match self.get((x, y)) {
                    '#' => {
                        new_cells[y][new_x] = '#';
                        new_cells[y][new_x + 1] = '#';
                    }
                    'O' if y > 0 && y < height - 1 && x > 0 && x < width - 1 => {
                        new_cells[y][new_x] = '[';
                        new_cells[y][new_x + 1] = ']';
                    }
                    '@' if y > 0 && y < height - 1 && x > 0 && x < width - 1 => {
                        new_cells[y][new_x] = '@';
                        new_cells[y][new_x + 1] = '.';
                    }
                    _ => (), // Leave as '.'
                }
            }
        });

        // Update positions
        self.robot.x *= 2;
        self.boxes = self
            .boxes
            .iter()
            .map(|p| Point::new(p.x * 2, p.y))
            .collect();
        self.cells = new_cells;
    }

    fn gps_sum(&self) -> isize {
        self.boxes.iter().map(|point| 100 * point.y + point.x).sum()
    }
}

fn main() {
    let (grid, path) = EXAMPLE.trim().split_once("\n\n").unwrap();
    let mut grid = Grid::new(grid);
    let path: Vec<Direction> = path
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| c.into())
        .collect();

    println!("Part 1:");
    println!("{}", grid);
    grid.run_path(&path);
    println!("{}", grid);
    println!("Part 1: {}", grid.gps_sum());

    // Part 2 - Create a new grid from the original input
    let mut grid2 = Grid::new(EXAMPLE2.trim().split_once("\n\n").unwrap().0);
    grid2.widen_grid();
    println!("\nPart 2:");
    println!("{}", grid2);
    grid2.run_path_part_2(&path);
    println!("{}", grid2);
    println!("Part 2: {}", grid2.gps_sum());
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}
