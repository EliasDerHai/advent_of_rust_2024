fn get_lists_from_input(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    input
        .iter()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("Invalid number"))
                .collect();
            assert_eq!(numbers.len(), 2);
            (numbers[0], numbers[1])
        })
        .unzip()
}

pub fn solve_day_01_part_01(input: Vec<String>) -> u32 {
    let (mut l1, mut l2): (Vec<u32>, Vec<u32>) = get_lists_from_input(input);

    l1.sort();
    l2.sort();

    l1.iter()
        .zip(l2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn solve_day_01_part_02(input: Vec<String>) -> u32 {
    let (l1, l2): (Vec<u32>, Vec<u32>) = get_lists_from_input(input);

    l1.iter()
        .map(|l1_value| l2.iter().filter(|l2_value| **l2_value == *l1_value).count() as u32 * l1_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::read_lines;
    use super::*;

    #[test]
    fn should_solve_day_01_part_01() {
        match read_lines("./src/day01/input.txt") {
            Ok(lines) => println!("Solution: {}", solve_day_01_part_01(lines)),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    #[test]
    fn should_solve_day_01_part_02() {
        match read_lines("./src/day01/input.txt") {
            Ok(lines) => println!("Solution: {}", solve_day_01_part_02(lines)),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }
}

