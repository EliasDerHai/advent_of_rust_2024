use super::part1::*;
use std::fmt;

// wasted too much time with stupid reverse engineering (see https://blog.jverkamp.com/2024/12/17/aoc-2024-day-17-virtual-machininator/)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Instruction {
    // True if the operand is always a literal value, false if it's a combo operand (below)
    fn is_literally_literal(&self) -> bool {
        match self {
            Self::Adv => false,
            Self::Bxl => true,
            Self::Bst => false,
            Self::Jnz => true,
            Self::Bxc => true, // Takes one but ignores it
            Self::Out => false,
            Self::Bdv => false,
            Self::Cdv => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    A,
    B,
    C,
}

impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => Self::Literal(value),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid combo operand"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{}", value),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Machine {
    pub a: u128,
    pub b: u128,
    pub c: u128,
    pub ip: usize,
    pub ram: Vec<u8>,
    pub halted: bool,
    pub output: Vec<u8>,
}

impl Machine {
    pub fn decompile(&self) -> String {
        let mut output = String::new();

        for (i, &byte) in self.ram.iter().enumerate().step_by(2) {
            let instruction = Instruction::from(byte);
            let operand = if instruction.is_literally_literal() {
                Operand::Literal(self.ram[i + 1])
            } else {
                Operand::from(self.ram[i + 1])
            };

            output.push_str(&format!("{instruction} {operand}\n"));
        }

        output
    }

    fn value_of(&self, operand: Operand) -> u128 {
        match operand {
            Operand::Literal(value) => value as u128,
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        // Always read an instruction + operand, out of bounds is an error
        if self.ip >= self.ram.len() - 1 {
            self.halted = true;
            return;
        }

        let instruction = Instruction::from(self.ram[self.ip]);
        let operand = if instruction.is_literally_literal() {
            Operand::Literal(self.ram[self.ip + 1])
        } else {
            Operand::from(self.ram[self.ip + 1])
        };

        match instruction {
            // Division (actually a right shift)
            Instruction::Adv => {
                self.a >>= self.value_of(operand);
            }
            // Bitwise XOR
            Instruction::Bxl => {
                self.b ^= self.value_of(operand);
            }
            // Bitwise set
            Instruction::Bst => {
                self.b = self.value_of(operand) & 0b111;
            }
            // Jump (if not zero)
            Instruction::Jnz => {
                if self.a != 0 {
                    self.ip = self.value_of(operand) as usize;
                    return; // Don't increment the IP
                }
            }
            // Bitwise XOR between b and c (ignores operand)
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            // Output
            Instruction::Out => {
                self.output.push((self.value_of(operand) as u8) & 0b111);
            }
            // Division (actually a right shift) to b, still reads from a
            Instruction::Bdv => {
                self.b = self.a >> self.value_of(operand);
            }
            // Division (actually a right shift) to c, still reads from a
            Instruction::Cdv => {
                self.c = self.a >> self.value_of(operand);
            }
        }

        self.ip += 2;
    }
}

pub fn parse(input: &str) -> Machine {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let b = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let c = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    lines.next(); // Skip the empty line

    let ram = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    Machine {
        a,
        b,
        c,
        ip: 0,
        ram,
        halted: false,
        output: Vec::new(),
    }
}

fn part2_backtrack(input: &Machine) -> u128 {
    fn recur(original_machine: &Machine, a: u128, index: usize) -> Option<u128> {
        for tribble in 0..8 {
            let mut machine = original_machine.clone();
            let next_a = (a << 3) | tribble;
            machine.a = next_a;
            machine.run();

            if machine.output[0] == machine.ram[index] {
                if index == 0 {
                    return Some(next_a);
                }

                if let Some(a) = recur(original_machine, next_a, index - 1) {
                    return Some(a);
                }
            }
        }

        None
    }

    recur(input, 0, input.ram.len() - 1).unwrap()
}

/*
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
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_17_part_02() {
        let input = read_string("./src/day17/input.txt").unwrap();

        let machine = parse(&input);

        assert_eq!(0, part2_backtrack(&machine));
    }
    /*
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
        */
}
