use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq)]
struct OrderRule {
    before: u8,
    after: u8,
}

fn update_complies_with_rules(update: &Vec<u8>, rules: &Vec<OrderRule>) -> bool {
    // actually relevant ones
    let rules: Vec<&OrderRule> = rules
        .iter()
        .filter(|&rule| update.contains(&rule.before) && update.contains(&rule.after))
        .collect();

    rules.iter().all(|rule| {
        let left_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.before).map(|(index, _)| index).unwrap();
        let right_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.after).map(|(index, _)| index).unwrap();

        left_idx < right_idx
    })
}

pub fn solve_day_05_part_01(input: Vec<String>) -> u32 {
    let order_rules: Vec<OrderRule> = input
        .iter()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let values: Vec<u8> = line.split('|').map(|number| number.parse::<u8>().unwrap()).collect();
            assert_eq!(2, values.len());
            OrderRule { before: values[0], after: values[1] }
        })
        .collect();
    let updates: Vec<Vec<u8>> = input
        .iter()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|number| number.parse::<u8>().unwrap()).collect())
        .collect();

    println!("Rules: {:?}", order_rules);
    println!("Updates: {:?}", updates);

    updates
        .into_iter()
        .filter(|update| update_complies_with_rules(update, &order_rules))
        .map(|update| update.iter().nth(update.len() / 2).map(|&v| v as u32).unwrap_or(0))
        .sum()
}

pub fn solve_day_05_part_02(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::read_lines;

    use super::*;

    #[test]
    fn should_solve_day_05_part_01_sample() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".lines().map(str::to_string).collect();

        let actual = solve_day_05_part_01(input);

        assert_eq!(143, actual);
    }


    #[test]
    fn should_solve_day_05_part_01() {
        let input = read_lines("./src/day05/input.txt").unwrap();

        let solution = solve_day_05_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_05_part_02() {
        todo!()
    }
}
