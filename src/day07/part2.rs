#[cfg(test)]
mod tests {
    use crate::day07::part1::solve_day_07;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_07_part_02() {
        let input = read_string("./src/day07/input.txt").unwrap();

        let solution = solve_day_07(input, true);

        println!("{solution}");
        assert_eq!(337041851384440, solution);
    }


    #[test]
    fn should_solve_day_07_part_02_sample() {
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

        assert_eq!(11387, solve_day_07(input, true));
    }
}
