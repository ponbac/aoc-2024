use std::time::Instant;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let start_time = Instant::now();
    day2();
    let end_time = Instant::now();
    println!("Time taken: {:?}", end_time.duration_since(start_time));
}

fn day2() {
    let lines = INPUT.trim().lines();

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
                let diff = (level - prev_level).abs();
                if !(1..=3).contains(&diff) {
                    return Err(());
                }

                let curr_is_increasing = *level > prev_level;
                if is_increasing.is_some() && is_increasing.unwrap() != curr_is_increasing {
                    return Err(());
                }

                Ok((*level, Some(curr_is_increasing)))
            },
        )
        .is_ok()
}
