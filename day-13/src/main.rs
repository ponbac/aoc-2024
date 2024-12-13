const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

#[derive(Debug)]
struct Button {
    id: char,
    delta: (u64, u64),
}

impl Button {
    fn x(&self) -> u64 {
        self.delta.0
    }

    fn y(&self) -> u64 {
        self.delta.1
    }

    fn cost(&self) -> u64 {
        match self.id {
            'A' => 3,
            'B' => 1,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Button>,
    prize: (u64, u64),
}

impl Machine {
    fn new(input: &str) -> Self {
        let buttons: Vec<Button> = input
            .lines()
            .rev()
            .skip(1)
            .map(|line| {
                let id = line.replace("Button ", "").chars().next().unwrap();
                let (x, y) = line.split_once("X+").unwrap().1.split_once(", Y+").unwrap();
                Button {
                    id,
                    delta: (x.parse().unwrap(), y.parse().unwrap()),
                }
            })
            .collect();

        let price = input.lines().last().unwrap();
        let (x, y) = price
            .split_once("X=")
            .unwrap()
            .1
            .split_once(", Y=")
            .unwrap();

        Machine {
            buttons: buttons.into_iter().rev().collect(),
            prize: (x.parse().unwrap(), y.parse().unwrap()),
        }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.buttons[0].x() + b * self.buttons[1].x();
                let y = a * self.buttons[0].y() + b * self.buttons[1].y();

                if x == self.prize.0 && y == self.prize.1 {
                    return Some((a, b));
                }
            }
        }
        None
    }

    fn solve_part_2(&self) -> Option<(u64, u64)> {
        let prize = (self.prize.0 * 10000000000000, self.prize.1 * 10000000000000);

        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.buttons[0].x() + b * self.buttons[1].x();
                let y = a * self.buttons[0].y() + b * self.buttons[1].y();

                if x == prize.0 && y == prize.1 {
                    return Some((a, b));
                }
            }
        }
        None
    }

    fn calculate_cost(&self, presses: (u64, u64)) -> u64 {
        presses.0 * self.buttons[0].cost() + presses.1 * self.buttons[1].cost()
    }
}

fn main() {
    let machines = INPUT
        .trim()
        .split("\n\n")
        .map(Machine::new)
        .collect::<Vec<_>>();

    let mut total_cost = 0;
    let mut winnable = 0;
    for machine in machines {
        if let Some(presses) = machine.solve() {
            winnable += 1;
            let cost = machine.calculate_cost(presses);
            total_cost += cost;
        }
    }

    println!("Total prizes possible: {}", winnable);
    println!("Total tokens needed: {}", total_cost);
}
