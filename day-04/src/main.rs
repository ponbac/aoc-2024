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
    search_pattern: Vec<char>,
    search_pattern_reverse: Vec<char>,
    search_length: usize,
}

impl Grid {
    fn new(input: &str, search_pattern: Vec<char>) -> Self {
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
            search_pattern: search_pattern.clone(),
            search_pattern_reverse: search_pattern.iter().rev().cloned().collect(),
            search_length: search_pattern.len(),
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.rows[y][x]
    }

    fn find_horizontal(&self, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
        if x + self.search_length > self.width {
            return (false, vec![]);
        }

        let chars: Vec<char> = (0..self.search_length)
            .map(|i| self.rows[y][x + i])
            .collect();
        let coords: Vec<(usize, usize)> = (0..self.search_length).map(|i| (x + i, y)).collect();

        if chars == self.search_pattern || chars == self.search_pattern_reverse {
            (true, coords)
        } else {
            (false, vec![])
        }
    }

    fn find_vertical(&self, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
        if y + self.search_length > self.height {
            return (false, vec![]);
        }

        let chars: Vec<char> = (0..self.search_length)
            .map(|i| self.rows[y + i][x])
            .collect();
        let coords: Vec<(usize, usize)> = (0..self.search_length).map(|i| (x, y + i)).collect();

        if chars == self.search_pattern || chars == self.search_pattern_reverse {
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
        if x + self.search_length - 1 < self.width && y + self.search_length - 1 < self.height {
            let chars: Vec<char> = (0..self.search_length)
                .map(|i| self.rows[y + i][x + i])
                .collect();
            let coords: Vec<(usize, usize)> =
                (0..self.search_length).map(|i| (x + i, y + i)).collect();

            if chars == self.search_pattern || chars == self.search_pattern_reverse {
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
        if x + self.search_length - 1 < self.width && y >= self.search_length - 1 {
            let chars: Vec<char> = (0..self.search_length)
                .map(|i| self.rows[y - i][x + i])
                .collect();
            let coords: Vec<(usize, usize)> =
                (0..self.search_length).map(|i| (x + i, y - i)).collect();

            if chars == self.search_pattern || chars == self.search_pattern_reverse {
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

    fn find_cross(&self, x: usize, y: usize) -> (bool, Vec<(usize, usize)>) {
        if x + 2 >= self.width || y + 2 >= self.height {
            return (false, vec![]);
        }

        let diagonal_down = self.find_diagonal_top_left_bottom_right(x, y);
        println!("{:?}", diagonal_down);
        let diagonal_up = self.find_diagonal_bottom_left_top_right(x, y + 2);
        println!("{:?}", diagonal_up);

        let count = diagonal_down.0 as usize + diagonal_up.0 as usize;
        (
            count == 2,
            diagonal_down.1.into_iter().chain(diagonal_up.1).collect(),
        )
    }
}

fn main() {
    let grid = Grid::new(INPUT, vec!['M', 'A', 'S']);
    // println!("{}", grid);

    let mut found_grid: Vec<Vec<bool>> = vec![vec![false; grid.width]; grid.height];

    let mut found = 0;
    let mut found_cross = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            // let (count, coords) = grid.find_all(x, y);
            // found += count;
            // for (cx, cy) in coords {
            //     found_grid[cy][cx] = true;
            // }
            let (cross, coords) = grid.find_cross(x, y);
            found_cross += cross as usize;
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
    println!("{}", found_cross);
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}
