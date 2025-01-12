use std::collections::{HashMap, HashSet};

use crate::day06::Direction::*;
use crate::day06::FieldType::Out;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum FieldType {
    // no obstacle can move on - true if visited - false otherwise
    #[default]
    Free,
    // obstacle can not move on
    Occupied,
    // out of map
    Out,
}

/// Direction into which the guard faces
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

const DIRECTIONS: [Direction; 4] = [
    N,
    W,
    S,
    E,
];

type GameMap = HashMap<(isize, isize), FieldType>;
type GamePosition = (isize, isize);

/// takes a string and indexes every letter to a tuple (x, y)
/// where x is the index of the horizontal position in the line
/// and y is the index of the vertical position (~ line)
fn parse_to_map(input: String) -> (GameMap, GamePosition) {
    let mut map: GameMap = HashMap::new();
    let mut start_pos = (isize::MAX, isize::MAX);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            match c {
                '#' => {
                    map.insert((x, y), FieldType::Occupied);
                }
                '.' => {
                    map.insert((x, y), FieldType::Free);
                }
                '^' => {
                    start_pos = (x, y);
                    map.insert((x, y), FieldType::Free);
                }
                _ => panic!("Unexpected token '{}' in input data", c),
            }
        }
    }

    if start_pos == (isize::MAX, isize::MAX) {
        panic!("No start position found!")
    }
    (map, start_pos)
}

fn travel_one_unit(map: &GameMap, pos: &GamePosition, dir: &Direction) -> (GamePosition, FieldType) {
    let x = pos.0;
    let y = pos.1;
    let next: GamePosition = match dir {
        N => {
            (x, y - 1)
        }
        E => {
            (x - 1, y)
        }
        S => {
            (x, y + 1)
        }
        W => {
            (x + 1, y)
        }
    };
    map.get(&next).map(|&field_type| (next, field_type)).unwrap_or((next, Out))
}

/// traverses the map from a given start position and returns the count of fields that have been visited (only first visit counts) until the map's end is reached
/// navigation strategy: walk straight till obstacle or end of map / turn right if obstacle
fn traverse_map(map: GameMap, start_pos: GamePosition) -> isize {
    let mut pos = Some(start_pos);
    let mut dir = N;
    let mut visited = HashSet::from([start_pos]);

    while let Some((x, y)) = pos {
        let (mut next_pos, mut next_field) = travel_one_unit(&map, &(x, y), &dir);
        while next_field == FieldType::Occupied {
            dir = DIRECTIONS[(DIRECTIONS.iter().position(|&d| d == dir).unwrap() + 1) % 4];
            (next_pos, next_field) = travel_one_unit(&map, &(x, y), &dir);
            // println!("Hit obstacle at ({x}, {y}) - changing dir to {:?}", dir);
        }
        if next_field == Out {
            // println!("Leaving map on {:?}", next_pos);
            pos = None; // terminates while loop
        } else {
            pos = Some(next_pos);
            visited.insert(next_pos);
        }
    }

    visited.len() as isize
}

pub fn solve_day_06_part_01(input: String) -> isize {
    let (map, start_pos) = parse_to_map(input);
    traverse_map(map, start_pos)
}

pub fn solve_day_06_part_02(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day06::solve_day_06_part_01;
    use crate::util::read_string;

    #[test]
    fn should_solve_day_06_part_01() {
        let input = read_string("./src/day06/input.txt").unwrap();

        let solution = solve_day_06_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_06_part_01_sample() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".trim().to_string();

        assert_eq!(41, solve_day_06_part_01(input));
    }

    #[test]
    fn should_solve_day_06_part_02() {
        todo!()
    }
}
