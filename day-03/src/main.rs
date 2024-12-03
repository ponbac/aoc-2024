use regex::Regex;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_split = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let sum: i32 = re_split
        .split(INPUT)
        .zip(
            std::iter::once(true)
                .chain(re_split.find_iter(INPUT).map(|m| m.as_str() == "do()")),
        )
        .filter_map(|(part, enabled)| {
            if !enabled {
                return None;
            }
            
            Some(
                re_mul
                    .captures_iter(part)
                    .map(|cap| {
                        let x: i32 = cap[1].parse().unwrap();
                        let y: i32 = cap[2].parse().unwrap();
                        x * y
                    })
                    .sum::<i32>(),
            )
        })
        .sum();

    println!("Sum: {}", sum);
}
