use super::part1::{DirectionKey, DoorCode, Transpileable, DIRECTION_KEYPAD};
use memoize::memoize;

fn prepend(dirs: &[DirectionKey], elt: DirectionKey) -> Vec<DirectionKey> {
    let mut v = Vec::with_capacity(dirs.len() + 1);
    v.push(elt);
    v.extend_from_slice(dirs);
    v
}

#[memoize]
fn sequence_length(dirs: Vec<DirectionKey>, remaining: u8) -> u128 {
    if remaining == 0 {
        return dirs.len() as u128;
    }
    let with_start = prepend(&dirs, DirectionKey::A);
    with_start
        .windows(2)
        .map(|pair| {
            let mut replacement = DIRECTION_KEYPAD.get(&(pair[0], pair[1])).unwrap().clone();
            replacement.push(DirectionKey::A);
            sequence_length(replacement, remaining - 1)
        })
        .sum()
}

pub(super) fn compile_fast(line: &str, intermediate_robots: u8) -> u128 {
    let code = DoorCode::from(line);
    let transpiled = code.transpile();
    sequence_length(transpiled.0, intermediate_robots)
}

pub fn solve_day_21_fast(input: &str, intermediate_robots: u8) -> u128 {
    input
        .lines()
        .map(|line| {
            let numeric_part = &line[..3];
            let numeric_part = numeric_part
                .parse::<u128>()
                .expect(&format!("Couln't parse '{numeric_part}'"));

            let transpiliation_length = compile_fast(line, intermediate_robots);

            numeric_part * transpiliation_length
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day21::part1::compile;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_part_2() {
        let input = read_string("./src/day21/input.txt").unwrap();

        let solution = solve_day_21_fast(&input.trim(), 25);

        assert_eq!(210686850124870, solution);
    }

    #[test]
    fn compile_slow_should_equal_compile_fast() {
        let input = "029A";
        assert_eq!(compile(input, 2), compile_fast(input, 2));
    }
}
