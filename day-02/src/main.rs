const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let n_valid_reports = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            is_valid_report(report)
                || (0..report.len())
                    .map(|i| {
                        let mut nums = report.to_vec();
                        nums.remove(i);
                        nums
                    })
                    .any(|variation| is_valid_report(&variation))
        })
        .count();
    println!("Number of valid reports: {}", n_valid_reports);
}

fn is_valid_report(report: &[i32]) -> bool {
    let mut direction = None;
    for window in report.windows(2) {
        if !(1..=3).contains(&(window[1] - window[0]).abs()) {
            return false;
        }

        let is_increasing = window[1] > window[0];
        match direction {
            None => direction = Some(is_increasing),
            Some(dir) if dir != is_increasing => return false,
            _ => {}
        }
    }

    true
}
