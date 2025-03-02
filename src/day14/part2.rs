pub fn solve_day_14_part_02(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;

    #[test]
    fn should_solve_day_14_part_02() {
        let input = read_string("./src/day14/input.txt").unwrap();

        let solution = solve_day_14_part_02(&input);

        println!("{solution}");
    }


    #[test]
    fn should_solve_day_14_part_02_sample() {
        let input = "".trim();

        assert_eq!(0, solve_day_14_part_02(input));
    }
}
