pub mod part1;
pub mod part2_state_machine;
pub mod part2_starts_with;




#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::day03::part2_starts_with::solve_day_03_part_02_starts_with;
    use crate::day03::part2_state_machine::solve_day_03_part_02_state_machine;
    use crate::util::file::{read_chars, read_string};

    /// used for debugging an issue in my state-machine
    #[test]
    fn should_be_equal() {
        let input = "do()mul(135,70)";

        let lazy = solve_day_03_part_02_state_machine(input.bytes().map(|b| b as char));
        let eager = solve_day_03_part_02_starts_with(input);

        assert_eq!(lazy, eager);
    }

    /// performance test - turn out eagerly loading String into mem and processing it with 'starts-with' is faster - than the char-by-char state-machine
    #[test]
    fn should_be_fast() {
        let start2 = Instant::now();
        let input2 = read_string("./src/day03/input.txt").unwrap();
        let solution2 = solve_day_03_part_02_starts_with(input2.as_str());
        let elapsed2 = start2.elapsed();
        println!("Eager result: {solution2}, time: {elapsed2:?}");

        let start1 = Instant::now();
        let input1 = read_chars("./src/day03/input.txt").unwrap();
        let solution1 = solve_day_03_part_02_state_machine(input1);
        let elapsed1 = start1.elapsed();
        println!("Lazy result: {solution1}, time: {elapsed1:?}");
    }
}
