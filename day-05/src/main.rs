use std::collections::HashMap;

const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

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

    let updates: Vec<Vec<String>> = updates_input
        .lines()
        .map(|line| line.split(',').map(|s| s.to_string()).collect())
        .collect();

    let valid_updates_sum: u32 = updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(|update| update[update.len() / 2].parse::<u32>().unwrap())
        .sum();
    println!("{}", valid_updates_sum);

    let invalid_updates_sum: u32 = updates
        .iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|update| {
            let mut sorted = update.clone();
            sort_update(&rules, &mut sorted);
            sorted[sorted.len() / 2].parse::<u32>().unwrap()
        })
        .sum();
    println!("{}", invalid_updates_sum);
}

fn is_valid_update(rules: &HashMap<String, Vec<String>>, update: &[String]) -> bool {
    let reversed_update = update.iter().rev().collect::<Vec<&String>>();
    !reversed_update.iter().enumerate().any(|(i, current)| {
        rules
            .get(*current)
            .map(|afters| {
                reversed_update
                    .iter()
                    .skip(i + 1)
                    .any(|next| afters.contains(next))
            })
            .unwrap_or(false)
    })
}

fn sort_update(rules: &HashMap<String, Vec<String>>, update: &mut [String]) {
    update.sort_by(|a, b| {
        if rules.get(a).map_or(false, |afters| afters.contains(b)) {
            std::cmp::Ordering::Less
        } else if rules.get(b).map_or(false, |afters| afters.contains(a)) {
            std::cmp::Ordering::Greater
        } else {
            b.cmp(a)
        }
    });
}
