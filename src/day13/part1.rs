use crate::util::point::Point;

#[derive(Debug)]
pub(crate) struct Arcade {
    pub(crate) machines: Vec<ClawMachine>,
}

#[derive(Debug, Default)]
pub(crate) struct ClawMachine {
    // fixed cost of 3
    pub(crate) a: ButtonBehavior,
    // fixed cost of 1
    pub(crate) b: ButtonBehavior,
    // must match exactly
    pub(crate) prize_location: Point,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ButtonBehavior {
    // add to x per button press
    pub(crate) x: i32,
    // add to y per button press
    pub(crate) y: i32,
}

impl From<&str> for Arcade {
    fn from(value: &str) -> Self {
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
                    .map(|n| n.parse::<i32>().expect("NaN"));
                a = ButtonBehavior {
                    x: numbers.next().expect("Should have X"),
                    y: numbers.next().expect("Should have Y"),
                }
            }
            if line.starts_with(B_PREFIX) {
                let mut numbers = line[B_PREFIX.len()..]
                    .split(BUTTON_SPLITTER)
                    .map(|n| n.parse::<i32>().expect("NaN"));
                b = ButtonBehavior {
                    x: numbers.next().expect("Should have X"),
                    y: numbers.next().expect("Should have Y"),
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
                    prize_location: price_location,
                })
            }
        }

        Arcade {
            machines
        }
    }
}

pub fn solve_day_13_part_01(input: &str) -> u128 {
    let arcade = Arcade::from(input);
    let mut tokens = 0;

    for machine in arcade.machines {
        if let Some((a_presses, b_presses)) = cramer_integer_solve(
            machine.a.x as i128,
            machine.a.y as i128,
            machine.b.x as i128,
            machine.b.y as i128,
            machine.prize_location.x as i128,
            machine.prize_location.y as i128,
        ) {
            if a_presses >= 0 && b_presses >= 0 {
                tokens += a_presses as u128 * 3 + b_presses as u128;
            }
        }
    }

    tokens
}

pub(crate) fn cramer_integer_solve(
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
) -> Option<(i128, i128)> {
    let det = ax * by - ay * bx;
    let det_sub_a = px * by - py * bx;
    let det_sub_b = ax * py - ay * px;

    if det == 0 || det_sub_a % det != 0 || det_sub_b % det != 0 {
        None
    } else {
        Some((det_sub_a / det, det_sub_b / det))
    }
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_13_part_01() {
        let input = read_string("./src/day13/input.txt").unwrap();

        let solution = solve_day_13_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_13_part_01_sample() {
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
        Prize: X=18641, Y=10279
        ";

        assert_eq!(480, solve_day_13_part_01(input));
    }
}
