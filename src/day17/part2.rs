use super::part1::*;

// looks good doesnt work (see https://blog.jverkamp.com/2024/12/17/aoc-2024-day-17-virtual-machininator/)
pub fn solve_day_17_part_02(input: &str) -> u32 {
    fn recur(
        original_machine: &TuringState,
        tape: &TuringTape,
        a: u128,
        index: usize,
    ) -> Option<u128> {
        for tribble in 0..8 {
            let mut machine = original_machine.clone();
            let next_a = (a << 3) | tribble;
            machine.reg_a = next_a as u32;

            dbg!(machine.clone());

            while machine.pointer < tape.0.len() {
                machine = {
                    let p = machine.pointer;
                    match tape.0.get(p) {
                        Some((ref opcode, operand)) => machine.advance(opcode, operand),
                        None => machine,
                    }
                };
            }

            if machine.output[0] == tape.0[index].1 {
                // Recursive base case
                if index == 0 {
                    return Some(next_a);
                }

                if let Some(a) = recur(original_machine, tape, next_a, index - 1) {
                    return Some(a);
                }
            }
        }

        None
    }

    let (tape, original_state) = parse(input);
    recur(&original_state, &tape, 0u128, tape.0.len() - 1).unwrap() as u32
}

// works but too slow
pub fn solve_day_17_part_02_brute_force(input: &str) -> u32 {
    let (tape, original_state) = parse(input);

    for i in 0.. {
        let mut state = original_state.clone();
        state.reg_a = i;

        while state.pointer < tape.0.len() {
            state = {
                let tape: &TuringTape = &tape;
                let p = state.pointer;
                match tape.0.get(p) {
                    Some((ref opcode, operand)) => state.advance(opcode, operand),
                    None => state,
                }
            };
        }

        let output = state
            .output
            .iter()
            .map(|el| el.to_string())
            .collect::<Vec<_>>()
            .join(",");

        println!("{i} - out: {output}");

        if TuringTape::from(output.as_str()) == tape {
            return i;
        }
    }

    panic!("should terminate");
}

#[cfg(test)]
mod tests {
    use super::*;
    //    use crate::util::file::read_string;
    //   #[test]
    //   fn should_solve_day_17_part_02() {
    //       let input = read_string("./src/day17/input.txt").unwrap();
    //
    //       let solution = solve_day_17_part_02(&input);
    //
    //       assert_eq!(0, solution);
    //   }

    #[test]
    fn inspect_tape() {
        dbg!(TuringTape::from("2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0"));
    }

    #[test]
    fn should_match_example_3() {
        let input = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        .trim();

        assert_eq!(117440, solve_day_17_part_02_brute_force(input));
    }
}
