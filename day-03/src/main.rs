use regex::Regex;

const EXAMPLE_INPUT: &str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_split = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let parts: Vec<_> = re_split.split(INPUT).collect();

    let mut enabled = true;
    let mut sum = 0;

    let delimiters: Vec<_> = re_split.find_iter(INPUT).map(|m| m.as_str()).collect();
    let mut delimiter_idx = 0;

    for part in parts {
        if enabled {
            for cap in re_mul.captures_iter(part) {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                println!("mul({},{}) = {}", x, y, x * y);
                sum += x * y;
            }
        }

        if delimiter_idx < delimiters.len() {
            enabled = match delimiters[delimiter_idx] {
                "do()" => true,
                "don't()" => false,
                _ => enabled,
            };
            delimiter_idx += 1;
        }
    }

    println!("Sum: {}", sum);
}
