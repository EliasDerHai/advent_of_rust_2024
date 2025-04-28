use std::collections::HashMap;

enum LogicOperator {
    OR,
    XOR,
    AND,
}

impl LogicOperator {
    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            LogicOperator::OR => left | right,
            LogicOperator::XOR => left ^ right,
            LogicOperator::AND => left && right,
        }
    }
}

struct UnresolvedExpression {
    left: String,
    right: String,
    out: String,
    operator: LogicOperator,
}

impl From<&str> for UnresolvedExpression {
    fn from(s: &str) -> Self {
        let (left, tail) = s.trim().split_once(" ").expect(&format!("Invalid: '{s}'"));
        let (operator, tail) = tail.split_once(" ").expect(&format!("Invalid: '{s}'"));
        let (right, tail) = tail.split_once(" ").expect(&format!("Invalid: '{s}'"));
        let out = tail.replace("-> ", "");

        let operator = match operator {
            "AND" => LogicOperator::AND,
            "XOR" => LogicOperator::XOR,
            "OR" => LogicOperator::OR,
            _ => panic!("'{operator}' is not an operator"),
        };

        UnresolvedExpression {
            left: left.to_string(),
            right: right.to_string(),
            out,
            operator,
        }
    }
}

fn parse(input: &str) -> (HashMap<String, bool>, Vec<UnresolvedExpression>) {
    input
        .trim()
        .split_once("\n\n")
        .or_else(|| input.split_once("\n\r\n\r"))
        .map(|(initial_values, expressions)| {
            let initial_values = initial_values
                .lines()
                .map(|line| {
                    line.split_once(": ")
                        .map(|(key, value)| (key.to_string(), value.trim() == "1"))
                        .expect(&format!("{} should contain ': '", line))
                })
                .collect();

            let expressions = expressions
                .lines()
                .map(UnresolvedExpression::from)
                .collect();

            (initial_values, expressions)
        })
        .expect("Doesn't contain empty line")
}

pub fn solve_day_24_part_01(input: &str) -> u64 {
    let (mut values, mut expressions) = parse(input);

    while !expressions.is_empty() {
        let mut solveables = Vec::new();
        expressions = expressions
            .into_iter()
            .filter_map(|exp| {
                if values.contains_key(&exp.left) && values.contains_key(&exp.right) {
                    solveables.push(exp);
                    return None;
                }
                Some(exp)
            })
            .collect();

        for UnresolvedExpression {
            out,
            left,
            right,
            operator,
        } in solveables
        {
            let left = *values.get(&left).unwrap();
            let right = *values.get(&right).unwrap();
            values.insert(out, operator.apply(left, right));
        }
    }

    values
        .into_iter()
        .filter(|(key, _value)| key.starts_with('z'))
        .fold(0u64, |aggr, (next_key, next_value)| {
            if !next_value {
                return aggr;
            }
            let v: u32 = next_key[1..].parse().expect("can't parse");
            aggr + 2u64.pow(v)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_24_part_01() {
        let input = read_string("./src/day24/input.txt").unwrap();

        let solution = solve_day_24_part_01(&input);

        assert_eq!(46463754151024, solution);
    }

    #[test]
    fn should_solve_day_24_part_01_sample() {
        let input = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .trim();

        assert_eq!(4, solve_day_24_part_01(input));
    }
}
