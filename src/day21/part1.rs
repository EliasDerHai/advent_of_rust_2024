#![allow(unused_variables, dead_code)]

use std::fmt::Display;

use crate::util::point::Point;

trait KeypadKey {
    fn get_pos(&self) -> Point;
}

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

#[derive(Debug, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    A,
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

fn transpile_door_key_code(mut current_pos: Point, code: &DoorCode) -> Vec<DirectionalKey> {
    code.0
        .iter()
        .flat_map(|key| {
            let mut moves = Vec::new();
            let target_pos = key.get_pos();

            while current_pos.x < target_pos.x {
                println!("right");
                moves.push(DirectionalKey::Right);
                current_pos = current_pos.right();
            }
            while current_pos.x > target_pos.x {
                println!("left");
                moves.push(DirectionalKey::Left);
                current_pos = current_pos.left();
            }
            while current_pos.y > target_pos.y {
                println!("up");
                moves.push(DirectionalKey::Up);
                current_pos = current_pos.up();
            }
            while current_pos.y < target_pos.y {
                println!("down");
                moves.push(DirectionalKey::Down);
                current_pos = current_pos.down();
            }
            if current_pos == target_pos {
                println!("A");
                moves.push(DirectionalKey::A);
            }

            moves
        })
        .collect()
}

pub fn solve_day_21_part_01(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    //   #[test]
    //   fn should_solve_day_21_part_01() {
    //       let input = read_string("./src/day21/input.txt").unwrap();
    //
    //       let solution = solve_day_21_part_01(&input);
    //
    //       println!("{solution}");
    //   }

    fn stringify(displayable: Vec<impl Display>) -> String {
        displayable
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn should_transpile_doorcode() {
        let code: DoorCode = "029A".into();
        let transpiled = transpile_door_key_code(DoorKey::A.get_pos(), &code);

        assert_eq!("<A^A>^^AvvvA", stringify(transpiled));
    }
}
