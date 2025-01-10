#[derive(Debug, Copy, Clone, PartialEq)]
struct OrderRule {
    before: u8,
    after: u8,
}

fn parse_rules(input: &Vec<String>) -> Vec<OrderRule> {
    input
        .iter()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let values: Vec<u8> = line.split('|').map(|number| number.parse::<u8>().unwrap()).collect();
            assert_eq!(2, values.len());
            OrderRule { before: values[0], after: values[1] }
        })
        .collect()
}

fn parse_updates(input: Vec<String>) -> Vec<Vec<u8>> {
    input
        .iter()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|number| number.parse::<u8>().unwrap()).collect())
        .collect()
}


fn get_relevant_rules(update: &Vec<u8>, rules: &[OrderRule]) -> Vec<OrderRule> {
    rules
        .iter()
        .filter(|&&rule| update.contains(&rule.before) && update.contains(&rule.after))
        .cloned()
        .collect()
}

fn check_if_compliant(update: &Vec<u8>, rules: &[OrderRule]) -> bool {
    // actually relevant ones
    let rules = get_relevant_rules(update, rules);

    rules.into_iter().all(|rule| {
        let left_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.before).map(|(index, _)| index).unwrap();
        let right_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.after).map(|(index, _)| index).unwrap();

        left_idx < right_idx
    })
}

pub fn solve_day_05_part_01(input: Vec<String>) -> u32 {
    let order_rules = parse_rules(&input);
    let updates = parse_updates(input);

    updates
        .into_iter()
        .filter(|update| check_if_compliant(update, &order_rules[..]))
        .map(|update| update.iter().nth(update.len() / 2).map(|&v| v as u32).unwrap_or(0))
        .sum()
}

fn sort_update(mut update: Vec<u8>, rules: &Vec<OrderRule>) -> Vec<u8> {
    rules.into_iter().for_each(|rule| {
        let left_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.before).map(|(index, _)| index).unwrap();
        let right_idx = update.iter().enumerate().find(|&(_, &value)| value == rule.after).map(|(index, _)| index).unwrap();

        if left_idx > right_idx {
            update.swap(left_idx, right_idx);
        }
    });

    if check_if_compliant(&update, rules) {
        update
    } else {
        sort_update(update, rules)
    }
}

pub fn solve_day_05_part_02(input: Vec<String>) -> u32 {
    let order_rules: Vec<OrderRule> = parse_rules(&input);
    let updates: Vec<Vec<u8>> = parse_updates(input);

    updates
        .into_iter()
        .filter_map(|update| {
            let relevant_rules = get_relevant_rules(&update, &order_rules);

            if !check_if_compliant(&update, &relevant_rules[..]) {
                Some((update, relevant_rules))
            } else {
                None
            }
        })
        .map(|(update, relevant_rules)| sort_update(update, &relevant_rules))
        .map(|sorted_update| sorted_update.iter().nth(sorted_update.len() / 2).map(|&v| v as u32).unwrap_or(0))
        .sum()
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
    fn should_solve_day_05_part_02_sample() {
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

        let actual = solve_day_05_part_02(input);

        assert_eq!(123, actual);
    }

    #[test]
    fn should_solve_day_05_part_02() {
        let input = read_lines("./src/day05/input.txt").unwrap();

        let solution = solve_day_05_part_02(input);

        println!("{solution}");
    }
}
