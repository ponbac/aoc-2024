use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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

    // For each frequency
    for positions in antennas.values() {
        // Compare each pair of antennas with the same frequency
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Calculate the vector between the two points
                let diff = p2.sub(&p1);

                // First antinode: extend from p2 in the direction of the vector
                let antinode1 = p2.add(&diff);
                if is_within_bounds(&antinode1, width, height) {
                    antinodes.insert(antinode1);
                }

                // Second antinode: extend from p1 in the opposite direction
                let antinode2 = p1.sub(&diff);
                if is_within_bounds(&antinode2, width, height) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

fn is_within_bounds(p: &Point, width: i32, height: i32) -> bool {
    p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
}

fn calculate_antinodes_part2(
    antennas: &HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    // For each frequency
    for positions in antennas.values() {
        // Skip if there's only one antenna of this frequency
        if positions.len() < 2 {
            continue;
        }

        // Compare each pair of antennas with the same frequency
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Add both antenna positions as antinodes
                if is_within_bounds(&p1, width, height) {
                    antinodes.insert(p1);
                }
                if is_within_bounds(&p2, width, height) {
                    antinodes.insert(p2);
                }

                // Calculate the vector between the two points
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                // Check all points on the line between and beyond the antennas
                // First, normalize the vector to get the smallest step
                let gcd = gcd(dx.abs(), dy.abs());
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                // Check points in both directions
                let mut current = Point {
                    x: p1.x - step_x,
                    y: p1.y - step_y,
                };
                // Check points before p1
                while is_within_bounds(&current, width, height) {
                    antinodes.insert(current);
                    current.x -= step_x;
                    current.y -= step_y;
                }

                let mut current = Point {
                    x: p2.x + step_x,
                    y: p2.y + step_y,
                };
                // Check points after p2
                while is_within_bounds(&current, width, height) {
                    antinodes.insert(current);
                    current.x += step_x;
                    current.y += step_y;
                }

                // Check points between p1 and p2
                let mut current = Point {
                    x: p1.x + step_x,
                    y: p1.y + step_y,
                };
                while current.x != p2.x || current.y != p2.y {
                    if is_within_bounds(&current, width, height) {
                        antinodes.insert(current);
                    }
                    current.x += step_x;
                    current.y += step_y;
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

    // Part 1
    let antinodes = calculate_antinodes(&antennas, width, height);
    println!("Part 1 - Number of unique antinodes: {}", antinodes.len());

    // Part 2
    let antinodes2 = calculate_antinodes_part2(&antennas, width, height);
    println!("Part 2 - Number of unique antinodes: {}", antinodes2.len());
}
