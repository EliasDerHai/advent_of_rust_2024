use std::collections::HashMap;

use crate::day19::part1::*;

pub fn solve_day_19_part_02(input: &str) -> usize {
    let (towels, goals) = parse(input);

    goals
        .0
        .into_iter()
        .map(|goal| {
            let mut memo: HashMap<&[StripColor], usize> = HashMap::new();
            can_be_achieved(&goal, &towels, &mut memo)
        })
        .sum()
}

fn can_be_achieved<'a>(
    goal: &'a [StripColor],
    towels: &AvailableTowels,
    memo: &mut HashMap<&'a [StripColor], usize>,
) -> usize {
    if goal.len() == 0 {
        return 1;
    }
    if let Some(&cached) = memo.get(goal) {
        return cached;
    }

    let mut possibles = 0;
    for towel in &towels.0 {
        if goal.starts_with(&towel) {
            possibles += can_be_achieved(&goal[towel.len()..], towels, memo);
        }
    }

    memo.insert(goal, possibles);
    possibles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_19_part_02() {
        let input = read_string("./src/day19/input.txt").unwrap();

        let solution = solve_day_19_part_02(&input);

        assert_eq!(723524534506343, solution);
    }

    #[test]
    fn should_solve_day_19_part_02_sample() {
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

        assert_eq!(16, solve_day_19_part_02(input));
    }
}
