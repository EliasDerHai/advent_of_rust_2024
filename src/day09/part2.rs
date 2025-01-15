pub fn solve_day_09_part_02(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_09_part_02() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_02(input);

        println!("{solution}");
    }


    #[test]
    fn should_solve_day_09_part_02_sample() {
        let input = "".trim().to_string();

        assert_eq!(0, solve_day_09_part_02(input));
    }
}
