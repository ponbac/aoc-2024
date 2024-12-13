use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, i32, i32) {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    (antennas, width, height)
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Calculate vector between points
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                // Add points where one antenna is twice as far as the other
                let antinode1 = Point {
                    x: p2.x + dx,
                    y: p2.y + dy,
                };
                let antinode2 = Point {
                    x: p1.x - dx,
                    y: p1.y - dy,
                };

                if antinode1.x >= 0
                    && antinode1.x < width
                    && antinode1.y >= 0
                    && antinode1.y < height
                {
                    antinodes.insert(antinode1);
                }
                if antinode2.x >= 0
                    && antinode2.x < width
                    && antinode2.y >= 0
                    && antinode2.y < height
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

fn calculate_antinodes_part2(
    antennas: &HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
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
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let gcd = gcd(dx.abs(), dy.abs()).max(1);
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                // Add all points on the line
                let mut p = Point { x: p1.x, y: p1.y };
                while p.x >= 0 && p.x < width && p.y >= 0 && p.y < height {
                    antinodes.insert(p);
                    p.x += step_x;
                    p.y += step_y;
                }

                let mut p = Point {
                    x: p1.x - step_x,
                    y: p1.y - step_y,
                };
                while p.x >= 0 && p.x < width && p.y >= 0 && p.y < height {
                    antinodes.insert(p);
                    p.x -= step_x;
                    p.y -= step_y;
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
