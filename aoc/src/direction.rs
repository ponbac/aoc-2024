#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn as_step(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    pub const ALL: [Direction; 4] = [
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

impl std::ops::Sub<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn sub(self, rhs: Direction) -> Self::Output {
        let step = rhs.as_step();
        (self.0 - step.0, self.1 - step.1)
    }
}
