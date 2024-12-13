const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug)]
struct Button {
    id: char,
    dx: u64,
    dy: u64,
}

impl Button {
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
    price_pos: (u64, u64),
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
                    dx: x.parse().unwrap(),
                    dy: y.parse().unwrap(),
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
            price_pos: (x.parse().unwrap(), y.parse().unwrap()),
        }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.buttons[0].dx + b * self.buttons[1].dx;
                let y = a * self.buttons[0].dy + b * self.buttons[1].dy;

                if x == self.price_pos.0 && y == self.price_pos.1 {
                    return Some((a, b));
                }
            }
        }
        None
    }

    // Cramer's rule 2x2
    fn solve_part_2(&self) -> Option<(u64, u64)> {
        let (x1, y1) = (self.buttons[0].dx as f64, self.buttons[0].dy as f64);
        let (x2, y2) = (self.buttons[1].dx as f64, self.buttons[1].dy as f64);

        let n_to_add = 10_000_000_000_000.0;
        let (target_x, target_y) = (
            self.price_pos.0 as f64 + n_to_add,
            self.price_pos.1 as f64 + n_to_add,
        );

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

    let (winnable, total_cost) = solve(&machines, false);
    println!("Part 1: {} winnable with {} tokens", winnable, total_cost);
    let (winnable_2, total_cost_2) = solve(&machines, true);
    println!(
        "Part 2: {} winnable with {} tokens",
        winnable_2, total_cost_2
    );
}

/// Returns `(winnable_count, total_cost)`
fn solve(machines: &[Machine], part2: bool) -> (u64, u64) {
    machines
        .iter()
        .filter_map(|machine| {
            let solver = if part2 {
                Machine::solve_part_2
            } else {
                Machine::solve
            };

            solver(machine).map(|presses| machine.calculate_cost(presses))
        })
        .fold((0, 0), |acc, cost| (acc.0 + 1, acc.1 + cost))
}
