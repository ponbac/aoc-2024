const INPUT: &str = include_str!("../input1.txt");

#[derive(Copy, Clone)]
enum Direction {
    Horizontal,   // →
    Vertical,     // ↓
    DiagonalDown, // ↘
    DiagonalUp,   // ↗
}

impl Direction {
    fn as_step(&self) -> (isize, isize) {
        match self {
            Direction::Horizontal => (1, 0),
            Direction::Vertical => (0, 1),
            Direction::DiagonalDown => (1, 1),
            Direction::DiagonalUp => (1, -1),
        }
    }

    const ALL: [Direction; 4] = [
        Direction::Horizontal,
        Direction::Vertical,
        Direction::DiagonalDown,
        Direction::DiagonalUp,
    ];
}

struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as isize;
        let width = cells.first().map_or(0, |row| row.len()) as isize;
        Self {
            cells,
            width,
            height,
        }
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn find_pattern(&self, pattern: &[char], start: (isize, isize), direction: Direction) -> bool {
        let step = direction.as_step();
        let end_x = start.0 + step.0 * (pattern.len() as isize - 1);
        let end_y = start.1 + step.1 * (pattern.len() as isize - 1);

        if !self.in_bounds(end_x, end_y) {
            return false;
        }

        let chars: Vec<char> = (0..pattern.len() as isize)
            .map(|i| {
                let x = start.0 + step.0 * i;
                let y = start.1 + step.1 * i;
                self.cells[y as usize][x as usize]
            })
            .collect();

        chars == pattern || chars.iter().rev().eq(pattern)
    }

    fn find_all_patterns(&self, pattern: &[char]) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .flat_map(|(x, y)| {
                Direction::ALL
                    .iter()
                    .filter(move |dir| self.find_pattern(pattern, (x, y), **dir))
            })
            .count()
    }

    fn find_crosses(&self, pattern: &[char]) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                self.find_pattern(pattern, (x, y), Direction::DiagonalDown)
                    && self.find_pattern(pattern, (x, y + 2), Direction::DiagonalUp)
            })
            .count()
    }
}

fn main() {
    let grid = Grid::new(INPUT);

    let xmas_patterns = grid.find_all_patterns(&['X', 'M', 'A', 'S']);
    println!("XMAS: {}", xmas_patterns);
    let crosses = grid.find_crosses(&['M', 'A', 'S']);
    println!("MAS X: {}", crosses);
}
