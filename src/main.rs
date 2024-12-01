use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let lines = INPUT.lines();

    let (mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>) = lines
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .unzip();
    left_numbers.sort();
    right_numbers.sort();

    let right_counts = right_numbers.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let (total_distance, total_similarity) = left_numbers.iter().zip(right_numbers.iter()).fold(
        (0, 0),
        |(dist_acc, sim_acc), (left, right)| {
            let distance = (left - right).abs();
            let similarity = left * right_counts.get(left).unwrap_or(&0);
            (dist_acc + distance, sim_acc + similarity)
        },
    );

    println!("Total distance: {}", total_distance);
    println!("Total similarity: {}", total_similarity);
}
