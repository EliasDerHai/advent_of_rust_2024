use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StripColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for StripColor {
    type Error = String;
    fn try_from(value: char) -> Result<Self, String> {
        match value {
            'w' => Ok(Self::White),
            'u' => Ok(Self::Blue),
            'b' => Ok(Self::Black),
            'r' => Ok(Self::Red),
            'g' => Ok(Self::Green),
            _ => Err(format!("unexpected '{}'", value)),
        }
    }
}

pub struct AvailableTowels(pub Vec<Vec<StripColor>>);
pub struct DesignGoals(pub Vec<Vec<StripColor>>);

pub fn parse(input: &str) -> (AvailableTowels, DesignGoals) {
    let mut lines = input.trim().lines();

    let first_line = lines.next().expect("empty input");
    let mut towels: Vec<Vec<StripColor>> = first_line
        .split(',')
        .map(|seq| {
            seq.trim()
                .chars()
                .map(|c| StripColor::try_from(c).unwrap())
                .collect()
        })
        .collect();

    // minor optimization
    towels.sort_by_key(|v| Reverse(v.len()));

    lines.next();

    let goals = lines
        .map(|line| {
            line.chars()
                .map(|c| StripColor::try_from(c).unwrap())
                .collect()
        })
        .collect();

    (AvailableTowels(towels), DesignGoals(goals))
}

pub fn solve_day_19_part_01(input: &str) -> usize {
    let (towels, goals) = parse(input);

    goals
        .0
        .into_iter()
        .filter(|goal| can_be_achieved(goal, &towels))
        .count()
}

fn can_be_achieved(goal: &[StripColor], towels: &AvailableTowels) -> bool {
    if goal.len() == 0 {
        return true;
    }

    for towel in &towels.0 {
        if goal.starts_with(&towel) {
            if can_be_achieved(&goal[towel.len()..], towels) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_19_part_01() {
        let input = read_string("./src/day19/input.txt").unwrap();

        let solution = solve_day_19_part_01(&input);

        assert_eq!(263, solution);
    }

    #[test]
    fn should_solve_day_19_part_01_sample() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .trim();

        assert_eq!(6, solve_day_19_part_01(input));
    }
}
