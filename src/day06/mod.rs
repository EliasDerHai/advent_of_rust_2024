use std::collections::{HashMap, HashSet};

use crate::day06::Direction::*;
use crate::day06::FieldType::*;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Default)]
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
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn next(&self) -> Direction {
        match *self {
            N => E,
            E => S,
            S => W,
            W => N
        }
    }
}

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
                    map.insert((x, y), Occupied);
                }
                '.' => {
                    map.insert((x, y), Free);
                }
                '^' => {
                    start_pos = (x, y);
                    map.insert((x, y), Free);
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

/// traverses the map from a given start position and returns the count of fields that have been visited (only first visit counts) until the map's end is reached
/// navigation strategy: walk straight till obstacle or end of map / turn right if obstacle
fn traverse_map(map: GameMap, start_pos: GamePosition) -> isize {
    let mut pos = Some(start_pos);
    let mut dir = N;
    let mut visited = HashSet::from([start_pos]);

    while let Some((x, y)) = pos {
        let (mut next_pos, mut next_field) = travel_one_unit(&map, &(x, y), &dir);
        while next_field == Occupied {
            dir = dir.next();
            (next_pos, next_field) = travel_one_unit(&map, &(x, y), &dir);
            // println!("Hit obstacle at ({x}, {y}) - changing dir to {:?}", dir);
        }
        if next_field == Out {
            // println!("Leaving map on {:?}", next_pos);
            pos = None; // terminates while loop
        } else { // next_field must be free
            pos = Some(next_pos);
            visited.insert(next_pos);
        }
    }

    visited.len() as isize
}

/// general strategy: read the map into a suitable data structure, walk the maze according to the
/// navigation rules and count each visited field (no duplicates = hashset)
pub fn solve_day_06_part_01(input: String) -> isize {
    let (map, start_pos) = parse_to_map(input);
    traverse_map(map, start_pos)
}

type Game2DArray = [[FieldType; MAX_MAP_SIZE]; MAX_MAP_SIZE];

type LowMemGamePosition = (u8, u8);


/// map is actually square and 130 fields wide/tall
const MAX_MAP_SIZE: usize = 130;

fn parse_to_2d_arr(input: String) -> (Game2DArray, LowMemGamePosition) {
    let mut map: Game2DArray = [[Out; MAX_MAP_SIZE]; MAX_MAP_SIZE];
    let mut start_pos = (u8::MAX, u8::MAX);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map[x][y] = Occupied;
                }
                '.' => {
                    map[x][y] = Free;
                }
                '^' => {
                    map[x][y] = Free;
                    start_pos = (x as u8, y as u8);
                }
                _ => panic!("Unexpected token '{}' in input data", c),
            }
        }
    }

    if start_pos == (u8::MAX, u8::MAX) {
        panic!("No start position found!")
    }
    (map, start_pos)
}

fn travel_one_unit_fast(map: &Game2DArray, pos: &LowMemGamePosition, dir: &Direction) -> (LowMemGamePosition, FieldType) {
    let mut x = pos.0 as i16;
    let mut y = pos.1 as i16;
    match dir {
        N => {
            y -= 1;
        }
        E => {
            x += 1;
        }
        S => {
            y += 1;
        }
        W => {
            x -= 1;
        }
    };

    if x < 0 || x >= MAX_MAP_SIZE as i16 || y < 0 || y >= MAX_MAP_SIZE as i16 {
        ((u8::MAX, u8::MAX), Out)
    } else {
        ((x as u8, y as u8), map[x as usize][y as usize])
    }
}


fn travel_one_unit(map: &GameMap, pos: &GamePosition, dir: &Direction) -> (GamePosition, FieldType) {
    let x = pos.0;
    let y = pos.1;
    let next: GamePosition = match dir {
        N => {
            (x, y - 1)
        }
        E => {
            (x + 1, y)
        }
        S => {
            (x, y + 1)
        }
        W => {
            (x - 1, y)
        }
    };
    map.get(&next).map(|&field_type| (next, field_type)).unwrap_or((next, Out))
}

/// checks if a map loops or not
fn check_for_loop_fast(
    map: &Game2DArray,
    mut pos: LowMemGamePosition,
    mut dir: Direction,
    mut visited: HashSet<(LowMemGamePosition, Direction)>,
) -> bool {
    loop {
        let (mut next_pos, mut next_field) = travel_one_unit_fast(&map, &pos, &dir);
        while next_field == Occupied {
            dir = dir.next();
            (next_pos, next_field) = travel_one_unit_fast(&map, &pos, &dir);
        }
        match next_field {
            Free => {
                if visited.contains(&(next_pos, dir)) {
                    return true;
                } else {
                    visited.insert((next_pos, dir));
                    pos = next_pos;
                }
            }
            Out => return false,
            Occupied => panic!("Should not be possible..."),
        }
    }
}

/// checks if a map loops or not
fn check_for_loop(
    map: &GameMap,
    mut pos: GamePosition,
    mut dir: Direction,
    mut visited: HashSet<(GamePosition, Direction)>,
) -> bool {
    // println!("checking - loop at entry {:?}", pos);
    loop {
        let (mut next_pos, mut next_field) = travel_one_unit(&map, &pos, &dir);
        while next_field == Occupied {
            dir = dir.next();
            (next_pos, next_field) = travel_one_unit(&map, &pos, &dir);
            // println!("Hit obstacle at ({x}, {y}) - changing dir to {:?}", dir);
        }
        if next_field == Out {
            // println!("doesnt loop");
            return false;
        } else { // next_field must be free
            pos = next_pos;

            if visited.contains(&(next_pos, dir)) {
                // println!("does loop");
                return true;
            } else {
                visited.insert((next_pos, dir));
            }
        }
    }
}


/// traverses the map while also evaluating if a new obstacle in front of the current pos would
/// introduce a loop
/// these obstacles can only be put on not traveled fields, since otherwise the traveled path would
/// not be possible
fn traverse_map_with_obstacle_loops_fast(
    mut map: Game2DArray,
    start_pos: LowMemGamePosition,
    mut visited: HashSet<(LowMemGamePosition, Direction)>,
) -> usize {
    let mut pos = Some(start_pos);
    let mut dir = N;
    let mut obstacles_for_loop = 0;
    while let Some((x, y)) = pos {
        let (mut next_pos, mut next_field) = travel_one_unit_fast(&map, &(x, y), &dir);
        while next_field == Occupied {
            dir = dir.next();
            (next_pos, next_field) = travel_one_unit_fast(&map, &(x, y), &dir);
            // println!("Hit obstacle at ({x}, {y}) - changing dir to {:?}", dir);
        }
        if next_field == Out {
            // println!("Leaving map on {:?}", next_pos);
            pos = None; // terminates while loop
        } else { // must be free
            if !visited.iter().any(|f| f.0 == next_pos) {
                // let mut modified_map = map.clone();
                map[next_pos.0 as usize][next_pos.1 as usize] =  Occupied; // mutate instead of clone - perf. optimization
                let visited_copy = visited.clone();
                if check_for_loop_fast(&map, pos.unwrap(), dir.next(), visited_copy) {
                    obstacles_for_loop += 1;
                }
                map[next_pos.0 as usize][next_pos.1 as usize] =  Free; // revert to original state - perf. optimization
            }
            // println!("at {:?}", next_pos);
            pos = Some(next_pos);
            visited.insert((next_pos, dir));
        }
    }
    // println!("obstacles: {:?}", obstacles_for_loop);
    obstacles_for_loop
}

/// traverses the map while also evaluating if a new obstacle in front of the current pos would
/// introduce a loop
/// these obstacles can only be put on not traveled fields, since otherwise the traveled path would
/// not be possible
fn traverse_map_with_obstacle_loops(
    mut map: GameMap,
    start_pos: GamePosition,
    mut visited: HashSet<(GamePosition, Direction)>,
) -> usize {
    let mut pos = Some(start_pos);
    let mut dir = N;
    let mut obstacles_for_loop = 0;
    while let Some((x, y)) = pos {
        let (mut next_pos, mut next_field) = travel_one_unit(&map, &(x, y), &dir);
        while next_field == Occupied {
            dir = dir.next();
            (next_pos, next_field) = travel_one_unit(&map, &(x, y), &dir);
        }
        if next_field == Out {
            pos = None; // terminates while loop
        } else { // must be free
            if !visited.iter().any(|f| f.0 == next_pos) {
                // let mut modified_map = map.clone();
                map.insert(next_pos, Occupied); // mutate instead of clone - perf. optimization
                let visited_copy = visited.clone();
                if check_for_loop(&map, pos.unwrap(), dir.next(), visited_copy) {
                    obstacles_for_loop += 1;
                }
                map.insert(next_pos, Free); // revert to original state - perf. optimization
            }
            // println!("at {:?}", next_pos);
            pos = Some(next_pos);
            visited.insert((next_pos, dir));
        }
    }
    // println!("obstacles: {:?}", obstacles_for_loop);
    obstacles_for_loop
}

/// general strategy: read the map into a suitable data structure, before walking it create a save-point
/// now put an obstacle in front of you walk the modified map like in part 1 but record your every position
/// incl. direction. Terminate the modified walk once you end up on a position with the same direction
/// as you have already been on or if you leave the map. Count the loops (aka meeting your own path),
/// jump back to the save-point walk one field forward and repeat the obstacle walk.
/// do this until your non-obstacle walk leaves the map
pub fn solve_day_06_part_02(input: String) -> usize {
    let (map, start_pos) = parse_to_map(input);
    let visited: HashSet<(GamePosition, Direction)> = HashSet::from([(start_pos, N)]);
    traverse_map_with_obstacle_loops(map, start_pos, visited)
}

pub fn solve_day_06_part_02_fast(input: String) -> usize {
    let (map, start_pos) = parse_to_2d_arr(input);
    // println!("Map: {:?}", map);
    let visited: HashSet<(LowMemGamePosition, Direction)> = HashSet::from([(start_pos, N)]);
    traverse_map_with_obstacle_loops_fast(map, start_pos, visited)
}

#[cfg(test)]
mod tests {
    use crate::day06::{solve_day_06_part_01, solve_day_06_part_02_fast};
    use crate::day06::Direction::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_06_part_01() {
        let input = read_string("./src/day06/input.txt").unwrap();

        let solution = solve_day_06_part_01(input);

        println!("{solution}");
        assert_eq!(5534, solution);
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
    fn should_turn_90_degrees() {
        assert_eq!(E, N.next());
        assert_eq!(S, E.next());
        assert_eq!(W, S.next());
        assert_eq!(N, W.next());
    }

    #[test]
    fn should_solve_day_06_part_02() {
        let input = read_string("./src/day06/input.txt").unwrap();

        let solution = solve_day_06_part_02_fast(input);

        println!("{solution}");
        assert_eq!(2262, solution);
        // on avg ~ 6.7s before optimization
        // on avg ~ 5.4s after switching to 2d-arr instead of hashmap
    }


    #[test]
    fn should_find_dimensions_of_input() {
        let input = read_string("./src/day06/input.txt").unwrap();
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        println!("{cols} - {rows}");
    }

    #[test]
    fn should_solve_day_06_part_02_sample() {
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

        assert_eq!(6, solve_day_06_part_02_fast(input));
    }
}
