#![allow(unused_variables, dead_code)]

use std::fmt::Display;

use crate::util::point::Point;

trait KeypadKey {
    fn get_pos(&self) -> Point;
}

/* DoorKey */

#[derive(Debug, PartialEq, Eq, Hash)]
enum DoorKey {
    A,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

impl From<char> for DoorKey {
    fn from(value: char) -> Self {
        match value {
            'A' => DoorKey::A,
            '0' => DoorKey::K0,
            '1' => DoorKey::K1,
            '2' => DoorKey::K2,
            '3' => DoorKey::K3,
            '4' => DoorKey::K4,
            '5' => DoorKey::K5,
            '6' => DoorKey::K6,
            '7' => DoorKey::K7,
            '8' => DoorKey::K8,
            '9' => DoorKey::K9,
            _ => panic!("unexpected value '{value}'"),
        }
    }
}

impl KeypadKey for DoorKey {
    fn get_pos(&self) -> Point {
        match self {
            DoorKey::K7 => Point::new(0, 0),
            DoorKey::K8 => Point::new(1, 0),
            DoorKey::K9 => Point::new(2, 0),
            DoorKey::K4 => Point::new(0, 1),
            DoorKey::K5 => Point::new(1, 1),
            DoorKey::K6 => Point::new(2, 1),
            DoorKey::K1 => Point::new(0, 2),
            DoorKey::K2 => Point::new(1, 2),
            DoorKey::K3 => Point::new(2, 2),
            DoorKey::K0 => Point::new(1, 3),
            DoorKey::A => Point::new(2, 3),
        }
    }
}

/* DoorCode */

#[derive(Debug, PartialEq, Eq, Hash)]
struct DoorCode([DoorKey; 4]);

impl From<&str> for DoorCode {
    fn from(value: &str) -> Self {
        DoorCode(
            value
                .chars()
                .map(DoorKey::from)
                .collect::<Vec<DoorKey>>()
                .try_into()
                .expect("incorrect length"),
        )
    }
}

fn parse_door_codes(input: &str) -> [DoorCode; 5] {
    input
        .trim()
        .lines()
        .map(DoorCode::from)
        .collect::<Vec<DoorCode>>()
        .try_into()
        .expect("incorrect length")
}

impl DoorCode {
    fn transpile(&self, mut current_pos: Point) -> DirectionKeySequence {
        DirectionKeySequence(
            self.0
                .iter()
                .flat_map(|key| {
                    let mut moves = Vec::new();
                    let target_pos = key.get_pos();

                    while current_pos.x < target_pos.x {
                        moves.push(DirectionalKey::Right);
                        current_pos = current_pos.right();
                    }
                    while current_pos.x > target_pos.x {
                        moves.push(DirectionalKey::Left);
                        current_pos = current_pos.left();
                    }
                    while current_pos.y > target_pos.y {
                        moves.push(DirectionalKey::Up);
                        current_pos = current_pos.up();
                    }
                    while current_pos.y < target_pos.y {
                        moves.push(DirectionalKey::Down);
                        current_pos = current_pos.down();
                    }
                    if current_pos == target_pos {
                        moves.push(DirectionalKey::A);
                    }

                    moves
                })
                .collect(),
        )
    }
}

/* DirectionalKey */

#[derive(Debug, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl From<char> for DirectionalKey {
    fn from(value: char) -> Self {
        match value {
            '^' => DirectionalKey::Up,
            'A' => DirectionalKey::A,
            '<' => DirectionalKey::Left,
            'v' => DirectionalKey::Down,
            '>' => DirectionalKey::Right,
            _ => panic!("unexpected value '{value}'"),
        }
    }
}

impl KeypadKey for DirectionalKey {
    fn get_pos(&self) -> Point {
        match self {
            DirectionalKey::Up => Point::new(1, 0),
            DirectionalKey::A => Point::new(2, 0),
            DirectionalKey::Left => Point::new(0, 1),
            DirectionalKey::Down => Point::new(1, 1),
            DirectionalKey::Right => Point::new(2, 1),
        }
    }
}

impl Display for DirectionalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKey::Up => write!(f, "^"),
            DirectionalKey::Down => write!(f, "v"),
            DirectionalKey::Left => write!(f, "<"),
            DirectionalKey::Right => write!(f, ">"),
            DirectionalKey::A => write!(f, "A"),
        }
    }
}

/* DirectionKeySequence */

struct DirectionKeySequence(Vec<DirectionalKey>);

impl From<&str> for DirectionKeySequence {
    fn from(value: &str) -> Self {
        DirectionKeySequence(
            value
                .chars()
                .map(DirectionalKey::from)
                .collect::<Vec<DirectionalKey>>(),
        )
    }
}

impl DirectionKeySequence {
    fn transpile(&self, mut current_pos: Point) -> DirectionKeySequence {
        DirectionKeySequence(
            self.0
                .iter()
                .flat_map(|key| {
                    let mut moves = Vec::new();
                    let target_pos = key.get_pos();

                    while current_pos.x < target_pos.x {
                        moves.push(DirectionalKey::Right);
                        current_pos = current_pos.right();
                    }
                    while current_pos.x > target_pos.x {
                        moves.push(DirectionalKey::Left);
                        current_pos = current_pos.left();
                    }
                    while current_pos.y > target_pos.y {
                        moves.push(DirectionalKey::Up);
                        current_pos = current_pos.up();
                    }
                    while current_pos.y < target_pos.y {
                        moves.push(DirectionalKey::Down);
                        current_pos = current_pos.down();
                    }
                    if current_pos == target_pos {
                        moves.push(DirectionalKey::A);
                    }

                    moves
                })
                .collect(),
        )
    }
}

pub fn solve_day_21_part_01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numeric_part = &line[..3];
            let numeric_part = numeric_part
                .parse::<u32>()
                .expect(&format!("Couln't parse '{numeric_part}'"));

            let code = DoorCode::from(line);
            let transpiled = code.transpile(DoorKey::A.get_pos());
            let transpiled = transpiled.transpile(DirectionalKey::A.get_pos());
            let transpiled = transpiled.transpile(DirectionalKey::A.get_pos());
            let transpiliation_length = transpiled.0.len() as u32;

            let solution =numeric_part * transpiliation_length ;
            println!("numeric_part = {numeric_part} + transpilation_length = {transpiliation_length} -> {solution}");
            solution
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::stringify::stringify;

    use crate::util::file::read_string;
    #[test]
    fn should_solve() {
        let input = read_string("./src/day21/input.txt").unwrap();

        let solution = solve_day_21_part_01(&input);

        println!("{solution}");
        assert_eq!(0, solution);
    }

    #[test]
    fn should_solve_example() {
        let input = "
029A
980A
179A
456A
379A"
            .trim();

        let solution = solve_day_21_part_01(&input);

        println!("{solution}");
        assert_eq!(126384, solution);
    }

    // TODO FIXME
    #[test]
    fn debug() {
        let code = DoorCode::from("179A");
        let transpiled = code.transpile(DoorKey::A.get_pos());
        let transpiled = transpiled.transpile(DirectionalKey::A.get_pos());
        let transpiled = transpiled.transpile(DirectionalKey::A.get_pos());

        assert_eq!(
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            stringify(transpiled.0)
        );
    }

    #[test]
    fn should_transpile_doorcode() {
        let code: DoorCode = "029A".into();
        let transpiled = code.transpile(DoorKey::A.get_pos());

        assert_eq!("<A^A>^^AvvvA", stringify(transpiled.0));
    }

    #[test]
    fn should_transpile_directioncode() {
        let sequence: DirectionKeySequence = "<A^A>^^AvvvA".into();
        let transpiled = sequence.transpile(DirectionalKey::A.get_pos());

        assert_eq!("<<vA>>^A<A>AvA<^AA>A<vAAA>^A", stringify(transpiled.0));
    }
}
