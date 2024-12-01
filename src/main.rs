use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim();
    let lines = input.lines();

    let (mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>) = lines
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .unzip();
    left_numbers.sort();
    right_numbers.sort();

    let total_distance = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>();
    println!("Total distance: {}", total_distance);

    let right_counts = right_numbers.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let total_similarity = left_numbers
        .iter()
        .map(|left| left * right_counts.get(left).unwrap_or(&0))
        .sum::<i32>();
    println!("Total similarity: {}", total_similarity);
}
