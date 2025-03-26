type OperandValue = u8;

#[derive(Debug, PartialEq, Eq, Default)]
struct TuringTape(Vec<(OpCode, OperandValue)>);

impl From<&str> for TuringTape {
    fn from(value: &str) -> Self {
        let raw: Vec<u8> = value
            .split(',')
            .into_iter()
            .map(|el| el.parse::<u8>().unwrap())
            .collect();
        let v = raw
            .chunks_exact(2)
            .map(|chunk| match chunk {
                [left, right] => (OpCode::from(left), *right),
                _ => panic!("Expected pairs, but found a leftover element."),
            })
            .collect();
        TuringTape(v)
    }
}

fn parse(input: &str) -> (TuringTape, TuringState) {
    let mut a_val = None;
    let mut b_val = None;
    let mut c_val = None;
    let mut tape: TuringTape = TuringTape::default();

    const REG_A_PREFIX: &str = "Register A: ";
    const REG_B_PREFIX: &str = "Register B: ";
    const REG_C_PREFIX: &str = "Register C: ";
    const REG_TAPE_PREFIX: &str = "Program: ";
    for line in input.trim().lines().into_iter() {
        if line.starts_with(REG_A_PREFIX) {
            a_val = Some(line[REG_A_PREFIX.len()..].parse::<u32>().unwrap())
        }
        if line.starts_with(REG_B_PREFIX) {
            b_val = Some(line[REG_B_PREFIX.len()..].parse::<u32>().unwrap())
        }
        if line.starts_with(REG_C_PREFIX) {
            c_val = Some(line[REG_C_PREFIX.len()..].parse::<u32>().unwrap())
        }
        if line.starts_with(REG_TAPE_PREFIX) {
            tape = TuringTape::from(&line[REG_TAPE_PREFIX.len()..]);
        }
    }

    (
        tape,
        TuringState::new(a_val.unwrap(), b_val.unwrap(), c_val.unwrap()),
    )
}

#[derive(Debug, PartialEq, Eq)]
struct TuringState {
    pointer: usize,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    output: Vec<u8>,
}

impl TuringState {
    fn new(reg_a: u32, reg_b: u32, reg_c: u32) -> TuringState {
        TuringState {
            pointer: 0,
            output: Vec::new(),
            reg_a,
            reg_b,
            reg_c,
        }
    }

    fn advance(mut self, opcode: &OpCode, operand: &u8) -> TuringState {
        let combo_operand = || ComboOperand.resolve(&self, *operand);
        let literal_operand = || LiteralOperand.resolve(&self, *operand);

        match opcode {
            OpCode::Adv => {
                self.reg_a = self.reg_a / 2u32.pow(combo_operand());
                self.pointer += 1;
            }
            OpCode::Bxl => {
                self.reg_b = self.reg_b ^ literal_operand();
                self.pointer += 1;
            }
            OpCode::Bst => {
                self.reg_b = combo_operand() % 8;
                self.pointer += 1;
            }
            OpCode::Jnz => {
                println!("jnz");
                if self.reg_a != 0 {
                    self.pointer = literal_operand() as usize;
                } else {
                    self.pointer += 1;
                }
            }
            OpCode::Bxc => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.pointer += 1;
            }
            OpCode::Out => {
                let out = combo_operand() % 8;
                println!("out: {out}");
                self.output.push(out as u8);
                self.pointer += 1;
            }
            OpCode::Bdv => {
                self.reg_b = self.reg_a / 2u32.pow(combo_operand());
                self.pointer += 1;
            }
            OpCode::Cdv => {
                self.reg_c = self.reg_a / 2u32.pow(combo_operand());
                self.pointer += 1;
            }
        }

        self
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<&u8> for OpCode {
    fn from(value: &u8) -> Self {
        match value {
            0u8 => OpCode::Adv,
            1u8 => OpCode::Bxl,
            2u8 => OpCode::Bst,
            3u8 => OpCode::Jnz,
            4u8 => OpCode::Bxc,
            5u8 => OpCode::Out,
            6u8 => OpCode::Bdv,
            7u8 => OpCode::Cdv,
            _ => panic!("bad opcode '{value}'"),
        }
    }
}

trait Operand: Copy {
    fn resolve(self, state: &TuringState, input: u8) -> u32;
}

#[derive(Debug, Clone, Copy)]
struct LiteralOperand;

impl Operand for LiteralOperand {
    fn resolve(self, _state: &TuringState, input: u8) -> u32 {
        input as u32
    }
}

#[derive(Debug, Clone, Copy)]
struct ComboOperand;

impl Operand for ComboOperand {
    fn resolve(self, state: &TuringState, input: u8) -> u32 {
        match input {
            0 | 1 | 2 | 3 => input as u32,
            4 => state.reg_a,
            5 => state.reg_b,
            6 => state.reg_c,
            _ => panic!("Reserved"),
        }
    }
}

fn advance(tape: &TuringTape, state: TuringState) -> TuringState {
    let p = state.pointer;
    match tape.0.get(p) {
        Some((ref opcode, operand)) => state.advance(opcode, operand),
        None => state,
    }
}

pub fn solve_day_17_part_01(input: &str) -> String {
    let (tape, mut state) = parse(input);

    //   println!("{:?}", tape);
    //   println!("{:?}", state);
    //   let mut i = 0;
    //   println!("{}", state.pointer);
    while state.pointer < tape.0.len() {
        //      println!("{}", state.pointer);
        state = advance(&tape, state);
        //       i += 1;
        //       if i > 50 {
        //           println!("{:?}", state);
        //           panic!("doesnt halt");
        //       }
    }

    state
        .output
        .iter()
        .map(|el| el.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    const EXAMPLE_INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    impl TuringState {
        fn new_all(
            reg_a: u32,
            reg_b: u32,
            reg_c: u32,
            pointer: usize,
            output: Vec<u8>,
        ) -> TuringState {
            TuringState {
                pointer,
                output,
                reg_a,
                reg_b,
                reg_c,
            }
        }
    }

    #[test]
    fn should_solve_day_17_part_01() {
        let input = read_string("./src/day17/input.txt").unwrap();

        let solution = solve_day_17_part_01(&input);

        assert_eq!("7,1,3,4,1,2,6,7,1", solution);
    }

    #[test]
    fn should_solve_day_17_part_01_sample() {
        let input = EXAMPLE_INPUT.trim();

        assert_eq!(
            "4,6,3,5,6,3,5,2,1,0".to_string(),
            solve_day_17_part_01(input)
        );
    }

    #[test]
    fn should_parse() {
        let input = EXAMPLE_INPUT.trim();
        let state = TuringState::new_all(729, 0, 0, 0, Vec::new());
        let tape: TuringTape = TuringTape::from("0,1,5,4,3,0");

        println!("{:?}", tape);
        assert_eq!((tape, state), parse(input));
    }

    mod advance {
        use super::*;

        #[test]
        fn should_match_example_1() {
            // arrange
            let state = TuringState::new(0, 0, 9);
            let tape: TuringTape = TuringTape::from("2,6");

            // act
            let state = advance(&tape, state);

            // assert
            let expected = TuringState::new_all(0, 1, 9, 1, Vec::new());
            assert_eq!(expected, state);
        }

        #[test]
        fn should_match_example_2() {
            let input = "
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

            assert_eq!("0,1,2".to_string(), solve_day_17_part_01(input));
        }
    }

    mod opcode {
        use super::*;

        #[test]
        fn should_adv() {
            // arrange
            let state = TuringState::new(4, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Adv, &2);

            // assert
            let expected = TuringState::new_all(1, 0, 0, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_bxl() {
            // arrange
            let state = TuringState::new(0, 5, 0);

            // act
            let final_state = state.advance(&OpCode::Bxl, &6);

            // assert
            let expected = TuringState::new_all(0, 3, 0, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_bst() {
            // arrange
            let state = TuringState::new(10, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Bst, &4);

            // assert
            let expected = TuringState::new_all(10, 2, 0, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_jnz_with_0_in_reg_a() {
            // arrange
            let state = TuringState::new(0, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Jnz, &123);

            // assert
            let expected = TuringState::new_all(0, 0, 0, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_jnz_with_5_in_reg_a() {
            // arrange
            let state = TuringState::new(5, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Jnz, &123);

            // assert
            let expected = TuringState::new_all(5, 0, 0, 123, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_bxc() {
            // arrange
            let state = TuringState::new(0, 5, 6);

            // act
            let final_state = state.advance(&OpCode::Bxc, &5);

            // assert
            let expected = TuringState::new_all(0, 3, 6, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_out() {
            // arrange
            let state = TuringState::new(0, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Out, &3);

            // assert
            let expected = TuringState::new_all(0, 0, 0, 1, vec![3]);
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_bdv() {
            // arrange
            let state = TuringState::new(4, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Bdv, &2);

            // assert
            let expected = TuringState::new_all(4, 1, 0, 1, Vec::new());
            assert_eq!(expected, final_state);
        }

        #[test]
        fn should_cdv() {
            // arrange
            let state = TuringState::new(4, 0, 0);

            // act
            let final_state = state.advance(&OpCode::Cdv, &2);

            // assert
            let expected = TuringState::new_all(4, 0, 1, 1, Vec::new());
            assert_eq!(expected, final_state);
        }
    }
}
