use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::time::Instant;

use regex::Regex;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let start = Instant::now();
    println!("Regex: {}, time: {:?}", regex_solution(), start.elapsed());

    let start = Instant::now();
    println!("Parser: {}, time: {:?}", nom_solution(), start.elapsed());
}

fn regex_solution() -> u32 {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_split = Regex::new(r"do\(\)|don't\(\)").unwrap();

    re_split
        .split(INPUT)
        .zip(std::iter::once(true).chain(re_split.find_iter(INPUT).map(|m| m.as_str() == "do()")))
        .filter_map(|(part, enabled)| {
            if !enabled {
                return None;
            }

            Some(
                re_mul
                    .captures_iter(part)
                    .map(|cap| {
                        let x: u32 = cap[1].parse().unwrap();
                        let y: u32 = cap[2].parse().unwrap();
                        x * y
                    })
                    .sum::<u32>(),
            )
        })
        .sum()
}

fn nom_solution() -> u32 {
    let (_, instructions) = parse_instructions(INPUT).unwrap();

    instructions
        .iter()
        .fold((true, 0), |(enabled, acc), instruction| match instruction {
            Instruction::Multiply(a, b) => (enabled, if enabled { acc + a * b } else { acc }),
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
        })
        .1
}

#[derive(Debug, Clone)]
enum Instruction {
    Multiply(u32, u32),
    Do,
    Dont,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Multiply(pair.0, pair.1)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_, instruction)| instruction))(input)
}
