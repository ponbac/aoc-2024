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

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

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

    antennas
}

fn calculate_antinodes(antennas: &HashMap<char, Vec<Point>>) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    let bounds = Point { x: 12, y: 12 }; // Based on example input size

    for (_, positions) in antennas {
        // For each pair of antennas with the same frequency
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let a1 = positions[i];
                let a2 = positions[j];

                // Vector from a1 to a2
                let dx = a2.x - a1.x;
                let dy = a2.y - a1.y;

                // First antinode: distance to a1 is half the distance to a2
                // This means it's 1/3 of the way from a1 to a2
                let antinode1 = Point {
                    x: a1.x + dx / 3,
                    y: a1.y + dy / 3,
                };

                // Second antinode: distance to a2 is half the distance to a1
                // This means it's 1/3 of the way from a2 back towards a1
                let antinode2 = Point {
                    x: a2.x - dx / 3,
                    y: a2.y - dy / 3,
                };

                // Only add antinodes that are within bounds
                if antinode1.x >= 0
                    && antinode1.x < bounds.x
                    && antinode1.y >= 0
                    && antinode1.y < bounds.y
                {
                    antinodes.insert(antinode1);
                }
                if antinode2.x >= 0
                    && antinode2.x < bounds.x
                    && antinode2.y >= 0
                    && antinode2.y < bounds.y
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    // Add antinodes that occur at antenna positions
    for (freq1, positions1) in antennas.iter() {
        for (freq2, positions2) in antennas.iter() {
            if freq1 != freq2 {
                // Different frequencies
                for pos1 in positions1 {
                    // Check if any antenna position is also an antinode
                    if antinodes.contains(pos1) {
                        antinodes.insert(*pos1);
                    }
                }
            }
        }
    }

    antinodes
}

fn main() {
    let antennas = parse_input(EXAMPLE);
    let antinodes = calculate_antinodes(&antennas);

    // Debug print
    println!("Antennas:");
    for (freq, positions) in &antennas {
        println!("{}: {:?}", freq, positions);
    }
    println!("\nAntinodes:");
    for antinode in &antinodes {
        println!("{:?}", antinode);
    }

    println!("\nNumber of unique antinodes: {}", antinodes.len());
}
