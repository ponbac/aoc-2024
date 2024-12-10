const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
    trailheads: Vec<(isize, isize)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as isize;
        let width = cells.first().map_or(0, |row| row.len()) as isize;

        let trailheads: Vec<(isize, isize)> = cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &cell)| (cell == '0').then_some((x as isize, y as isize)))
            })
            .collect();

        Self {
            cells,
            width,
            height,
            trailheads,
        }
    }
}

fn main() {
    let grid = Grid::new(EXAMPLE.trim());
    println!("{}", grid);
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

    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn add(self, rhs: Direction) -> Self::Output {
        let step = rhs.as_step();
        (self.0 + step.0, self.1 + step.1)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
