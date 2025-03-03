use crate::util::grid::{CharGrid, Direction};

struct WarehouseMap {

}

type RobotMoveInstruction = Direction;

fn parse(input: &str) -> (CharGrid, Vec<Direction>) {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();

    let warehouse = CharGrid::from(warehouse);
    let instructions = instructions.chars().filter_map(|c| match c {
        '>' => Some(Direction::E),
        '<' => Some(Direction::W),
        '^' => Some(Direction::N),
        'v' => Some(Direction::S),
        _ => None,
    }).collect();

    (warehouse, instructions)
}

pub fn solve_day_15_part_01(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_15_part_01() {
        let input = read_string("./src/day15/input.txt").unwrap();

        let solution = solve_day_15_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_15_part_01_sample() {
        let input = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "
        .trim();

        assert_eq!(10092, solve_day_15_part_01(input));
    }
}
