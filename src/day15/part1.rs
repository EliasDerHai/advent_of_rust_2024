use crate::util::grid::{Direction, Grid};
use crate::util::point::Point;
use std::collections::VecDeque;

type WarehouseGrid = Grid<WarehouseCell>;
type RobotMoveInstruction = Direction;

/// represents the input incl. Robot variant
#[derive(Debug, PartialEq)]
enum WarehouseCellParsing {
    Wall,
    Box,
    Empty,
    Robot,
}

/// represents the easier to handle Warehouse variants without Robot
/// (didn't fit the Grid based algo and is therefor replaced by robot_pos)
#[derive(Debug, PartialEq)]
enum WarehouseCell {
    Wall,
    Box,
    Empty,
}

impl From<char> for WarehouseCellParsing {
    fn from(c: char) -> Self {
        match c {
            '#' => WarehouseCellParsing::Wall,
            '.' => WarehouseCellParsing::Empty,
            '@' => WarehouseCellParsing::Robot,
            'O' => WarehouseCellParsing::Box,
            c => panic!("didn't expect '{c}' in the warehouse!"),
        }
    }
}

impl From<WarehouseCellParsing> for WarehouseCell {
    fn from(value: WarehouseCellParsing) -> Self {
        match value {
            WarehouseCellParsing::Wall => WarehouseCell::Wall,
            WarehouseCellParsing::Box => WarehouseCell::Box,
            WarehouseCellParsing::Empty => WarehouseCell::Empty,
            WarehouseCellParsing::Robot => WarehouseCell::Empty,
        }
    }
}

#[derive(Debug, PartialEq)]
struct WarehouseSituation {
    grid: WarehouseGrid,
    instructions: VecDeque<RobotMoveInstruction>,
    robot_pos: Point,
}

impl WarehouseSituation {
    fn new(grid: WarehouseGrid, instructions: Vec<RobotMoveInstruction>, start_pos: Point) -> Self {
        WarehouseSituation {
            grid,
            instructions: instructions.into(),
            robot_pos: start_pos,
        }
    }

    fn apply_all_instructions(self) -> WarehouseGrid {
        let mut next = self.next();

        while let InstructionResult::Next(situation) = next {
            next = situation.next();
        }

        match next {
            InstructionResult::Next(_) => panic!("should not have terminated while loop..."),
            InstructionResult::Final(grid) => grid,
        }
    }

    fn next(mut self) -> InstructionResult {
        match self.instructions.pop_front() {
            None => InstructionResult::Final(self.grid),
            Some(instruction) => InstructionResult::Next(self.apply(instruction)),
        }
    }

    fn apply(mut self, instruction: RobotMoveInstruction) -> Self {
        let next = self.robot_pos + instruction;

        match self.grid.get(&next).unwrap() {
            &WarehouseCell::Wall => self,
            &WarehouseCell::Empty => {
                self.robot_pos = next;
                self
            }
            &WarehouseCell::Box => self.attempt_to_move_box(next, instruction),
        }
    }

    fn attempt_to_move_box(mut self, first_box: Point, instruction: RobotMoveInstruction) -> Self {
        // represents the last element of the stack to shift
        // initialized as box and incrementally checked if "row" ends in wall or free space
        let mut end_of_stack = (first_box.clone(), &WarehouseCell::Box);

        while end_of_stack.1 == &WarehouseCell::Box {
            let next_point = end_of_stack.0 + instruction;
            let next_cell = self.grid.get(&next_point).unwrap();
            end_of_stack = (next_point, next_cell);
        }

        match end_of_stack {
            (_, WarehouseCell::Wall) => self,
            (p, WarehouseCell::Empty) => {
                self.grid.set(first_box, WarehouseCell::Empty);
                self.grid.set(p, WarehouseCell::Box);
                self.robot_pos = first_box;
                self
            }
            (_, WarehouseCell::Box) => panic!("should not have terminated while loop..."),
        }
    }
}

enum InstructionResult {
    Next(WarehouseSituation),
    Final(WarehouseGrid),
}

impl From<&str> for WarehouseSituation {
    fn from(value: &str) -> Self {
        let (grid, instructions) = value
            .split_once("\n\n")
            .unwrap_or_else(|| value.split_once("\r\n\r\n").unwrap());

        let grid = Grid::from(grid).map(WarehouseCellParsing::from);

        let start_pos = *grid
            .iter()
            .find(|(&_, cell)| cell == &&WarehouseCellParsing::Robot)
            .unwrap()
            .0;
        let grid = grid.map(WarehouseCell::from);

        let instructions = instructions
            .chars()
            .filter_map(|c| match c {
                '>' => Some(Direction::E),
                '<' => Some(Direction::W),
                '^' => Some(Direction::N),
                'v' => Some(Direction::S),
                _ => None,
            })
            .collect();

        WarehouseSituation::new(grid, instructions, start_pos)
    }
}

pub fn solve_day_15_part_01(input: &str) -> u32 {
    let initial_situation = WarehouseSituation::from(input);
    let final_warehouse = initial_situation.apply_all_instructions();

    final_warehouse
        .into_iter()
        .filter(|(_, cell)| cell == &WarehouseCell::Box)
        .map(|(p, _)| p.x as u32 + (p.y as u32 * 100))
        .sum()
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

    const INPUT_EXAMPLE: &'static str = "
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn should_solve_day_15_part_01_sample() {
        let input = INPUT_EXAMPLE.trim();

        assert_eq!(10092, solve_day_15_part_01(input));
    }

    #[test]
    fn should_parse() {
        let input = INPUT_EXAMPLE.trim();

        let actual = WarehouseSituation::from(input);

        // assert
        let first_instruction = actual.instructions.get(0).unwrap();
        let last_instruction = actual
            .instructions
            .get(actual.instructions.len() - 1)
            .unwrap();
        let origin = actual.grid.get(&Point::new(0, 0)).unwrap();
        let one_one = actual.grid.get(&Point::new(1, 1)).unwrap();
        let first_box = actual.grid.get(&Point::new(3, 1)).unwrap();

        assert_eq!(&Direction::W, first_instruction);
        assert_eq!(&Direction::N, last_instruction);
        assert_eq!(&WarehouseCell::Wall, origin);
        assert_eq!(&WarehouseCell::Empty, one_one);
        assert_eq!(&WarehouseCell::Box, first_box);
        assert_eq!(Point::new(4, 4), actual.robot_pos)
    }

    /// the instructions don't really matter here
    #[test]
    fn should_move_single_box() {
        let situation = WarehouseSituation::from("@O..#\n\n");

        let actual = situation.attempt_to_move_box(Point::new(1, 0), Direction::E);

        let expected = WarehouseSituation::from(".@O.#\n\n");

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_move_two_boxes() {
        let situation = WarehouseSituation::from("@OO.#\n\n");

        let actual = situation.attempt_to_move_box(Point::new(1, 0), Direction::E);

        let expected = WarehouseSituation::from(".@OO#\n\n");

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_not_move_boxes() {
        let situation = WarehouseSituation::from("@OO#\n\n");

        let actual = situation.attempt_to_move_box(Point::new(1, 0), Direction::E);

        let expected = WarehouseSituation::from("@OO#\n\n");

        assert_eq!(expected, actual);
    }

    /// instructions do matter now
    #[test]
    fn should_apply_all_instructions() {
        let situation = WarehouseSituation::from("@O..#\n\n>>");

        let actual = situation.apply_all_instructions();

        let expected = WarehouseSituation::from("..@O#\n\n");

        assert_eq!(expected.grid, actual);
    }
}
