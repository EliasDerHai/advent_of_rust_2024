use crate::util::point::Point;

#[derive(Debug)]
struct Arcade {
    machines: Vec<ClawMachine>,
}

#[derive(Debug, Default)]
struct ClawMachine {
    // fixed cost of 3
    a: ButtonBehavior,
    // fixed cost of 1
    b: ButtonBehavior,
    // must match exactly
    prize_location: Point,
}

#[derive(Debug, Default, Clone)]
struct ButtonBehavior {
    // add to x per button press
    x: i32,
    // add to y per button press
    y: i32,
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
                    .map(|n| n.parse::<i32>().expect("NaN"));
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

#[derive(Debug, Clone)]
struct Step<'a> {
    a: &'a ButtonBehavior,
    a_press_count: u8,
    b: &'a ButtonBehavior,
    b_press_count: u8,
    current_position: Point,
}

impl Step<'_> {
    /// runtime explodes (O^2) in iterative approach
    /// needs to be done mathematically
    /// (Cramer's rule https://en.wikipedia.org/wiki/Cramer%27s_rule) - but fuck this shit I'm done with AOC
    fn traverse(&mut self, solutions: &mut Vec<u32>) {
        {
            let mut next_a = self.clone();
            next_a.press(self.a.x, self.a.y, true);
            if next_a.is_success() {
                solutions.push(next_a.a_press_count as u32 * 3 + next_a.b_press_count as u32);
            } else if !next_a.is_failed() {
                next_a.traverse(solutions);
            }
        }
        {
            let next_b = self;
            next_b.press(next_b.b.x, next_b.b.y, true);
            if next_b.is_success() {
                solutions.push(next_b.a_press_count as u32 * 3 + next_b.b_press_count as u32);
            } else if !next_b.is_failed() {
                next_b.traverse(solutions);
            }
        }
    }

    fn press(&mut self, x: i32, y: i32, is_a: bool) {
        self.current_position.x -= x;
        self.current_position.y -= y;
        if is_a {
            self.a_press_count += 1;
        } else {
            self.b_press_count += 1;
        }
    }

    fn is_success(&self) -> bool {
        self.current_position == Point::new(0, 0)
    }

    fn is_failed(&self) -> bool {
        self.current_position.x < 0 || self.current_position.y < 0 || self.a_press_count > 100 || self.b_press_count > 100
    }
}

fn attempt_solve(machine: ClawMachine) -> Option<u32> {
    let mut first_step = Step {
        a: &machine.a,
        a_press_count: 0,
        b: &machine.b,
        b_press_count: 0,
        current_position: machine.prize_location.clone(),
    };

    let mut solutions: Vec<u32> = Vec::new();
    first_step.traverse(&mut solutions);

    solutions
        .iter()
        .min()
        .map(|v| *v)
}

pub fn solve_day_13_part_01(input: &str) -> u32 {
    let arcade = Arcade::from(input);
    arcade.machines
        .into_iter()
        .filter_map(|machine| attempt_solve(machine))
        .sum()
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

        println!("{:?}", Arcade::from(input));

        assert_eq!(480, solve_day_13_part_01(input));
    }
}
