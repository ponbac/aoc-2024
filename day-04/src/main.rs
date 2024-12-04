use std::fmt;

const INPUT: &str = include_str!("../input1.txt");

const EXAMPLE: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

struct Grid {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let width = rows[0].len();
        let height = rows.len();

        Self {
            rows,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.rows[y][x]
    }

    fn find_horizontal(&self, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
        if x + 3 >= self.width {
            return (false, vec![]);
        }

        let chars: Vec<char> = (0..4).map(|i| self.rows[y][x + i]).collect();
        let coords: Vec<(usize, usize)> = (0..4).map(|i| (x + i, y)).collect();

        if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
            (true, coords)
        } else {
            (false, vec![])
        }
    }

    fn find_vertical(&self, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
        if y + 3 >= self.height {
            return (false, vec![]);
        }

        let chars: Vec<char> = (0..4).map(|i| self.rows[y + i][x]).collect();
        let coords: Vec<(usize, usize)> = (0..4).map(|i| (x, y + i)).collect();

        if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
            (true, coords)
        } else {
            (false, vec![])
        }
    }

    fn find_diagonal_top_left_bottom_right(
        &self,
        x: usize,
        y: usize,
    ) -> (bool, Vec<(usize, usize)>) {
        if x + 3 < self.width && y + 3 < self.height {
            let chars: Vec<char> = (0..4).map(|i| self.rows[y + i][x + i]).collect();
            let coords: Vec<(usize, usize)> = (0..4).map(|i| (x + i, y + i)).collect();

            if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
                return (true, coords);
            }
        }
        (false, vec![])
    }

    fn find_diagonal_bottom_left_top_right(
        &self,
        x: usize,
        y: usize,
    ) -> (bool, Vec<(usize, usize)>) {
        if x + 3 < self.width && y >= 3 {
            let chars: Vec<char> = (0..4).map(|i| self.rows[y - i][x + i]).collect();
            let coords: Vec<(usize, usize)> = (0..4).map(|i| (x + i, y - i)).collect();

            if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
                return (true, coords);
            }
        }
        (false, vec![])
    }

    fn find_all(&self, x: usize, y: usize) -> (usize, Vec<(usize, usize)>) {
        let horizontal = self.find_horizontal(x, y);
        let vertical = self.find_vertical(x, y);
        let diagonal_down = self.find_diagonal_top_left_bottom_right(x, y);
        let diagonal_up = self.find_diagonal_bottom_left_top_right(x, y);

        let count = horizontal.0 as usize
            + vertical.0 as usize
            + diagonal_down.0 as usize
            + diagonal_up.0 as usize;

        (
            count,
            horizontal
                .1
                .into_iter()
                .chain(vertical.1)
                .chain(diagonal_down.1)
                .chain(diagonal_up.1)
                .collect(),
        )
    }
}

fn main() {
    let grid = Grid::new(INPUT);
    println!("{}", grid);

    let mut found_grid: Vec<Vec<bool>> = vec![vec![false; grid.width]; grid.height];

    let mut found = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let (count, coords) = grid.find_all(x, y);
            found += count;
            for (cx, cy) in coords {
                found_grid[cy][cx] = true;
            }
        }
    }

    println!(
        "{}",
        found_grid
            .iter()
            .enumerate()
            .map(|(y, row)| row
                .iter()
                .enumerate()
                .map(|(x, b)| if *b { grid.get(x, y) } else { '.' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );

    println!("{}", found);
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}
