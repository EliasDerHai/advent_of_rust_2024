use crate::day13::part1::{cramer_integer_solve, Arcade, ButtonBehavior, ClawMachine};
use crate::util::point::Point;

impl Arcade {
    fn from_with_added_billion(value: &str) -> Self {
        let mut machines = Vec::new();

        let mut a: ButtonBehavior = ButtonBehavior::default();
        let mut b: ButtonBehavior = ButtonBehavior::default();

        const A_PREFIX: &'static str = "Button A: X+";
        const B_PREFIX: &'static str = "Button B: X+";
        const PRICE_PREFIX: &'static str = "Prize: X=";
        const BUTTON_SPLITTER: &'static str = ", Y+";
        const PRICE_SPLITTER: &'static str = ", Y=";

        for line in value.lines().map(|l| l.trim()) {
            if line.starts_with(A_PREFIX) {
                let mut numbers = line[A_PREFIX.len()..]
                    .split(BUTTON_SPLITTER)
                    .map(|n| n.parse::<u32>().expect("NaN"));
                a = ButtonBehavior {
                    x: numbers.next().expect("Should have X") as i128,
                    y: numbers.next().expect("Should have Y") as i128,
                }
            }
            if line.starts_with(B_PREFIX) {
                let mut numbers = line[B_PREFIX.len()..]
                    .split(BUTTON_SPLITTER)
                    .map(|n| n.parse::<u32>().expect("NaN"));
                b = ButtonBehavior {
                    x: numbers.next().expect("Should have X") as i128,
                    y: numbers.next().expect("Should have Y") as i128,
                }
            }
            if line.starts_with(PRICE_PREFIX) {
                let mut numbers = line[PRICE_PREFIX.len()..]
                    .split(PRICE_SPLITTER)
                    .map(|n| n.parse::<i128>().expect("NaN"));
                let price_location = Point::new(
                    numbers.next().expect("Should have X"),
                    numbers.next().expect("Should have Y"),
                );

                machines.push(ClawMachine {
                    a: a.clone(),
                    b: b.clone(),
                    prize_location: price_location + (10000000000000i128, 10000000000000i128),
                })
            }
        }

        Arcade { machines }
    }
}

pub fn solve_day_13_part_02(input: &str) -> u128 {
    let arcade = Arcade::from_with_added_billion(input);
    let mut tokens = 0;

    for machine in arcade.machines {
        if let Some((a_presses, b_presses)) = cramer_integer_solve(
            machine.a.x as i128,
            machine.a.y as i128,
            machine.b.x as i128,
            machine.b.y as i128,
            machine.prize_location.x,
            machine.prize_location.y,
        ) {
            if a_presses >= 0 && b_presses >= 0 {
                tokens += a_presses as u128 * 3 + b_presses as u128;
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_13_part_02() {
        let input = read_string("./src/day13/input.txt").unwrap();

        let solution = solve_day_13_part_02(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_13_part_02_sample() {
        let input = "
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
        Prize: X=18641, Y=10279";

        assert_eq!(875318608908, solve_day_13_part_02(input));
    }
}
