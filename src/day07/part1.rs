struct Calibration {
    target: usize,
    parts: Vec<usize>,
}

enum Operator {
    Multiply,
    Add,
    Concat,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operator::Multiply => left * right,
            Operator::Add => left + right,
            Operator::Concat => format!("{left}{right}").parse().unwrap()
        }
    }
}

impl Calibration {
    fn from_line(line: &str) -> Self {
        let mut split1 = line.split(":");
        let target = split1.next().unwrap().parse::<usize>().unwrap();
        let parts = split1.next().unwrap().split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
        Calibration {
            target,
            parts,
        }
    }


    fn check_if_can_meet_target(&self, include_concat: bool) -> bool {
        let (&init, tail) = self.parts.split_first().expect("parts cannot be empty");
        let final_values = tail.iter().fold(vec![init], |acc, &next| {
            let mut results = Vec::with_capacity(acc.len() * 3);

            acc.iter()
                .map(|&a| {
                    if a <= self.target {
                        Operator::Add.apply(a, next)
                    } else {
                        self.target + 1
                    }
                })
                .for_each(|val| results.push(val));

            acc.iter()
                .map(|&a| {
                    if a <= self.target {
                        Operator::Multiply.apply(a, next)
                    } else {
                        self.target + 1
                    }
                })
                .for_each(|val| results.push(val));

            if include_concat {
                acc.iter()
                    .map(|&a| {
                        if a <= self.target {
                            Operator::Concat.apply(a, next)
                        } else {
                            self.target + 1
                        }
                    })
                    .for_each(|val| results.push(val));
            }

            results
        });

        // Check if we can meet the target
        final_values.contains(&self.target)
    }
}

pub fn solve_day_07(input: String, include_concat: bool) -> usize {
    input
        .lines()
        .map(|l| Calibration::from_line(l))
        .filter(|c| c.check_if_can_meet_target(include_concat))
        .map(|c| c.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_07_part_01() {
        let input = read_string("./src/day07/input.txt").unwrap();

        let solution = solve_day_07(input, false);

        println!("{solution}");
        assert_eq!(303766880536, solution);
    }

    #[test]
    fn should_solve_day_07_part_01_sample() {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20".trim().to_string();

        assert_eq!(3749, solve_day_07(input, false));

        assert_eq!(3, solve_day_07("3: 1 1 1".to_string(), false));

        assert_eq!(10, solve_day_07("10: 2 5 1".to_string(), false));
    }
}
