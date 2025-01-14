pub fn solve_day_08_part_01(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::read_string;

    use super::*;

    #[test]
    fn should_solve_day_08_part_01() {
        let input = read_string("./src/day08/input.txt").unwrap();

        let solution = solve_day_08_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_08_part_01_sample() {
        let input = "".trim().to_string();

        assert_eq!(0, solve_day_08_part_01(input));
    }
}
