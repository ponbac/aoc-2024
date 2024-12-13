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
        let (x1, y1) = (self.buttons[0].x() as f64, self.buttons[0].y() as f64);
        let (x2, y2) = (self.buttons[1].x() as f64, self.buttons[1].y() as f64);
        let (target_x, target_y) = (self.prize.0 as f64, self.prize.1 as f64);

        let determinant = x1 * y2 - x2 * y1;
        if determinant == 0.0 {
            return None;
        }

        let a = (target_x * y2 - x2 * target_y) / determinant;
        let b = (x1 * target_y - target_x * y1) / determinant;

        const EPSILON: f64 = 1e-10;
        if a < 0.0 || b < 0.0 || a.fract().abs() > EPSILON || b.fract().abs() > EPSILON {
            return None;
        }

        let solution_x = a * x1 + b * x2;
        let solution_y = a * y1 + b * y2;
        if (solution_x - target_x).abs() > EPSILON || (solution_y - target_y).abs() > EPSILON {
            return None;
        }

        Some((a as u64, b as u64))
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
    for machine in &machines {
        if let Some(presses) = machine.solve() {
            winnable += 1;
            let cost = machine.calculate_cost(presses);
            total_cost += cost;
        }
    }

    println!("Part 1:");
    println!("Total prizes possible: {}", winnable);
    println!("Total tokens needed: {}", total_cost);

    let mut total_cost_2 = 0;
    let mut winnable_2 = 0;
    for machine in &machines {
        if let Some(presses) = machine.solve_part_2() {
            println!("Possible to win machine {:?}", machine);
            winnable_2 += 1;
            let cost = machine.calculate_cost(presses);
            total_cost_2 += cost;
        }
    }

    println!("\nPart 2:");
    println!("Total prizes possible: {}", winnable_2);
    println!("Total tokens needed: {}", total_cost_2);
}
