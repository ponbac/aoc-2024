use std::collections::HashMap;

use aoc::Point;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: (isize, isize),
}

struct Arena {
    robots: Vec<Robot>,
    width: isize,
    height: isize,
}

impl Arena {
    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.position += robot.velocity;
            robot.position.wrap_around(self.width, self.height);
        }
    }

    fn safety_factor(&self) -> usize {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;

        let mut quadrants = [0; 4];
        for robot in &self.robots {
            if robot.position.x == mid_x || robot.position.y == mid_y {
                continue;
            }

            let quadrant = match (robot.position.x < mid_x, robot.position.y < mid_y) {
                (true, true) => 0,   // Top-left
                (false, true) => 1,  // Top-right
                (true, false) => 2,  // Bottom-left
                (false, false) => 3, // Bottom-right
            };
            quadrants[quadrant] += 1;
        }

        quadrants.iter().product()
    }

    fn is_christmas_tree(&self) -> bool {
        (0..self.height).into_par_iter().any(|y| {
            let mut current_streak = 0;
            let mut max_streak = 0;

            for x in 0..self.width {
                let pos = Point::new(x, y);
                if self.robots.iter().any(|r| r.position == pos) {
                    current_streak += 1;
                    max_streak = max_streak.max(current_streak);
                } else {
                    current_streak = 0;
                }
            }

            max_streak >= 8
        })
    }
}

fn main() {
    let robots = INPUT
        .trim()
        .lines()
        .map(|line| {
            let (position, velocity) = line.strip_prefix("p=").unwrap().split_once("v=").unwrap();
            let position = position.parse().unwrap();
            let velocity = velocity.split_once(',').unwrap();
            Robot {
                position,
                velocity: (velocity.0.parse().unwrap(), velocity.1.parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let mut arena = Arena {
        robots,
        width: 101,
        height: 103,
    };

    let mut steps = 0;
    let mut safety_factor = 0;
    loop {
        arena.step();
        steps += 1;

        if steps == 100 {
            safety_factor = arena.safety_factor();
        }

        if arena.is_christmas_tree() {
            println!("{}", arena);
            println!("Safety factor after 100 steps: {}", safety_factor);
            println!("Steps to form christmas tree: {}", steps);
            break;
        }
    }
}

impl std::fmt::Display for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let robots_per_position: HashMap<_, _> =
            self.robots
                .iter()
                .map(|r| &r.position)
                .fold(HashMap::new(), |mut counts, pos| {
                    *counts.entry(pos).or_insert(0) += 1;
                    counts
                });

        for y in 0..self.height {
            for x in 0..self.width {
                match robots_per_position.get(&Point::new(x, y)) {
                    Some(&count) => write!(f, "{}", count)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
