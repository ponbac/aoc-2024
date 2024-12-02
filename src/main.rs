use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");
const INPUT_2: &str = include_str!("./input2.txt");

const EXAMPLE_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

fn main() {
    // day1();
    day2();
}

fn day2() {
    let lines = INPUT_2.trim().lines();

    let reports: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let n_valid_reports = reports
        .iter()
        .filter(|report| {
            if is_valid_report(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut variation = report.to_vec();
                variation.remove(i);
                if is_valid_report(&variation) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("Number of valid reports: {}", n_valid_reports);
}

fn is_valid_report(report: &[i32]) -> bool {
    let first_level = report[0];

    report
        .iter()
        .skip(1)
        .try_fold(
            (first_level, None),
            |(prev_level, is_increasing), level| -> Result<(i32, Option<bool>), ()> {
                if prev_level == *level {
                    return Err(());
                }

                let diff = (level - prev_level).abs();
                if !(1..=3).contains(&diff) {
                    return Err(());
                }

                let new_is_increasing = *level > prev_level;
                if is_increasing.is_some() && is_increasing.unwrap() != new_is_increasing {
                    return Err(());
                }

                Ok((*level, Some(new_is_increasing)))
            },
        )
        .is_ok()
}

fn day1() {
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
