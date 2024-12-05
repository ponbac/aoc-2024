use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let (rules_input, updates_input) = INPUT.trim().split_once("\n\n").unwrap();

    let rules: HashMap<String, Vec<String>> =
        rules_input.lines().fold(HashMap::new(), |mut acc, line| {
            let (id, after) = line.split_once("|").unwrap();
            acc.entry(id.to_string())
                .or_default()
                .push(after.to_string());
            acc
        });
    let (valid_updates, invalid_updates): (Vec<Vec<String>>, Vec<Vec<String>>) = updates_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut valid, mut invalid), update| {
                if is_valid_update(&rules, &update) {
                    valid.push(update);
                } else {
                    invalid.push(update);
                }
                (valid, invalid)
            },
        );

    let valid_updates_sum: u32 = valid_updates
        .iter()
        .map(|update| middle_value(update))
        .sum();
    let invalid_updates_sum: u32 = invalid_updates
        .into_iter()
        .map(|mut update| {
            sort_update(&rules, &mut update);
            middle_value(&update)
        })
        .sum();

    println!("Part 1: {}", valid_updates_sum);
    println!("Part 2: {}", invalid_updates_sum);
}

fn is_valid_update(rules: &HashMap<String, Vec<String>>, update: &[String]) -> bool {
    !update.iter().enumerate().any(|(i, current)| {
        rules
            .get(current)
            .map(|afters| update.iter().take(i).any(|prev| afters.contains(prev)))
            .unwrap_or(false)
    })
}

fn sort_update(rules: &HashMap<String, Vec<String>>, update: &mut [String]) {
    update.sort_by(|a, b| {
        if rules.get(a).map_or(false, |afters| afters.contains(b)) {
            Ordering::Less
        } else if rules.get(b).map_or(false, |afters| afters.contains(a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn middle_value(update: &[String]) -> u32 {
    update[update.len() / 2].parse::<u32>().unwrap()
}
