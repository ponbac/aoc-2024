use std::collections::{HashMap, HashSet};

use aoc::Point;

const INPUT: &str = include_str!("../input1.txt");

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, isize, isize) {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point::new(
                    x as isize,
                    y as isize,
                ));
            }
        }
    }

    (antennas, width, height)
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    width: isize,
    height: isize,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Calculate vector between points
                let diff = p2 - p1;

                // Add points where one antenna is twice as far as the other
                let antinode1 = p2 + (diff.x, diff.y);
                let antinode2 = p1 - (diff.x, diff.y);

                if antinode1.in_bounds(width, height) {
                    antinodes.insert(antinode1);
                }
                if antinode2.in_bounds(width, height) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

fn calculate_antinodes_part2(
    antennas: &HashMap<char, Vec<Point>>,
    width: isize,
    height: isize,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        if positions.len() < 2 {
            continue;
        }

        // Add antenna positions as antinodes
        for &p in positions {
            antinodes.insert(p);
        }

        // Add all collinear points
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Get smallest step size using GCD
                let diff = p2 - p1;
                let gcd = gcd(diff.x.abs() as i32, diff.y.abs() as i32).max(1);
                let step = (diff.x / gcd as isize, diff.y / gcd as isize);

                // Add all points on the line
                let mut p = p1;
                while p.in_bounds(width, height) {
                    antinodes.insert(p);
                    p += step;
                }

                let mut p = Point::new(p1.x - step.0, p1.y - step.1);
                while p.in_bounds(width, height) {
                    antinodes.insert(p);
                    p -= step;
                }
            }
        }
    }

    antinodes
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let (antennas, width, height) = parse_input(INPUT);
    println!(
        "Part 1 - Number of unique antinodes: {}",
        calculate_antinodes(&antennas, width, height).len()
    );
    println!(
        "Part 2 - Number of unique antinodes: {}",
        calculate_antinodes_part2(&antennas, width, height).len()
    );
}
