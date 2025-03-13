use crate::day15::part1::*;
use crate::day15::part2::DoubleWidthWarehouseCell::*;
use crate::util::grid::{Direction, Grid};
use crate::util::point::Point;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};

type DoubleWidthWarehouseGrid = Warehouse<DoubleWidthWarehouseCell>;
const DOUBLE_WIDTH_UPSCALE_FACTOR: (usize, usize) = (2, 1);

#[derive(Debug, PartialEq, Clone)]
enum DoubleWidthWarehouseCell {
    Wall,
    BoxEast,
    BoxWest,
    Empty,
}

impl Display for DoubleWidthWarehouseCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Wall => write!(f, "#"),
            BoxEast => write!(f, "]"),
            BoxWest => write!(f, "["),
            Empty => write!(f, "."),
        }
    }
}

impl Display for DoubleWidthWarehouseGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = self.grid.iter().map(|(point, _)| point.x).max().unwrap();
        let max_y = self.grid.iter().map(|(point, _)| point.y).max().unwrap();

        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                let target = Point::new(x, y);
                if self.robot_pos == target {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", self.grid.get(&target).unwrap())?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl DoubleWidthWarehouseCell {
    /// horizontally flip cells (only really makes sense for boxes)
    fn mirror(&self) -> Self {
        match self {
            BoxEast => BoxWest,
            BoxWest => BoxEast,
            _ => self.clone(),
        }
    }
}

impl From<&str> for DoubleWidthWarehouseGrid {
    fn from(value: &str) -> Self {
        let char_grid = Grid::<char>::from(value).map(WarehouseCellParsing::from);
        let robot_pos = *char_grid
            .iter()
            .find(|(_, &ref c)| c == &WarehouseCellParsing::Robot)
            .unwrap()
            .0
            * DOUBLE_WIDTH_UPSCALE_FACTOR;

        DoubleWidthWarehouseGrid::from((char_grid.map(WarehouseCell::from), robot_pos))
    }
}

impl From<(Grid<WarehouseCell>, Point)> for DoubleWidthWarehouseGrid {
    fn from((grid, robot_pos): (Grid<WarehouseCell>, Point)) -> Self {
        let width_multiplier = DOUBLE_WIDTH_UPSCALE_FACTOR;
        let mut double_warehouse_grid: HashMap<Point, DoubleWidthWarehouseCell> = HashMap::new();

        grid.into_iter().for_each(|(point, cell)| {
            let left = point * width_multiplier; // upscaled cell
            let right = left.right(); // upscaled cell's right neighbor
            match cell {
                WarehouseCell::Wall => {
                    double_warehouse_grid.insert(left, Wall);
                    double_warehouse_grid.insert(right, Wall);
                }
                WarehouseCell::Box => {
                    double_warehouse_grid.insert(left, BoxWest);
                    double_warehouse_grid.insert(right, BoxEast);
                }
                WarehouseCell::Empty => {
                    double_warehouse_grid.insert(left, Empty);
                    double_warehouse_grid.insert(right, Empty);
                }
            };
        });

        let double_warehouse_grid = Grid::new(double_warehouse_grid);
        DoubleWidthWarehouseGrid::new(double_warehouse_grid, robot_pos)
    }
}

impl Warehouse<DoubleWidthWarehouseCell> {
    fn apply_instruction(mut self, instruction: RobotMoveInstruction) -> Self {
        let next = self.robot_pos + instruction;
        let next_cell = self.grid.get(&next).unwrap().clone();

        match next_cell {
            Wall => self,
            Empty => {
                self.robot_pos = next;
                self
            }
            BoxEast | BoxWest => self.attempt_to_move_box((next, next_cell.clone()), instruction),
        }
    }

    fn attempt_to_move_box(
        self,
        first_box: (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Self {
        match instruction {
            RobotMoveInstruction::N | RobotMoveInstruction::S => {
                self.attempt_to_move_box_vertically(first_box, instruction)
            }
            RobotMoveInstruction::E | RobotMoveInstruction::W => {
                self.attempt_to_move_box_horizontally(first_box, instruction)
            }
            _ => panic!("only N, E, S, W expected but found {:?}", instruction),
        }
    }

    fn attempt_to_move_box_horizontally(
        self,
        first_box: (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Self {
        let mut end_of_stack = first_box.clone();

        while end_of_stack.1 == BoxEast || end_of_stack.1 == BoxWest {
            let next_point = end_of_stack.0 + instruction;
            let next_cell = self.grid.get(&next_point).unwrap().clone();
            end_of_stack = (next_point, next_cell);
        }

        match end_of_stack {
            (_, Wall) => self,
            (_, Empty) => self.move_box_horizontally(first_box, end_of_stack),
            (_, _) => {
                panic!("should not have terminated while loop...")
            }
        }
    }

    fn move_box_horizontally(
        mut self,
        (first_point, first_cell): (Point, DoubleWidthWarehouseCell),
        (end_point, _end_cell): (Point, DoubleWidthWarehouseCell),
    ) -> Warehouse<DoubleWidthWarehouseCell> {
        let first_point = first_point;

        let mut start_x = first_point.x;
        let mut end_x = end_point.x;
        if start_x > end_x {
            let tmp = start_x;
            start_x = end_x;
            end_x = tmp;
        }
        for x in start_x + 1..end_x {
            let p = Point::new(x, first_point.y);
            let original = self.grid.get(&p).unwrap();
            let flipped = original.mirror();
            self.grid.set(p, flipped);
        }

        self.grid.set(first_point, Empty);
        self.robot_pos = first_point;
        self.grid.set(end_point, first_cell.mirror());
        self
    }

    fn attempt_to_move_box_vertically(
        mut self,
        first_box: (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Self {
        match self.select_vertical_tree_rec(first_box.clone(), instruction) {
            None => self,
            Some(tree_items) => {
                self.robot_pos = first_box.0;
                // clear whole tree
                tree_items
                    .iter()
                    .for_each(|(p, _)| self.grid.set(*p, Empty));
                // shift whole tree into instruction direction
                tree_items
                    .iter()
                    .for_each(|(p, c)| self.grid.set(*p + instruction, c.clone()));

                self
            }
        }
    }

    /// select the tree of moveable nodes
    /// returns Some points if moveable
    /// or None if move forbidden (one of the boxes would collide with a wall)
    fn select_vertical_tree_rec(
        &self,
        curr: (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Option<HashMap<Point, DoubleWidthWarehouseCell>> {
        match curr {
            (_, Wall) => None,
            (_, Empty) => Some(HashMap::new()),
            (p, BoxWest) => self.collect((p, BoxWest), (p.right(), BoxEast), instruction),
            (p, BoxEast) => self.collect((p, BoxEast), (p.left(), BoxWest), instruction),
        }
    }

    fn collect(
        &self,
        (p, p_cell): (Point, DoubleWidthWarehouseCell),
        (neighbor, neighbor_cell): (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Option<HashMap<Point, DoubleWidthWarehouseCell>> {
        let mut collection = HashMap::new();
        collection.insert(p, p_cell);
        collection.insert(neighbor, neighbor_cell);

        {
            let next = p + instruction;
            let successor = self.select_vertical_tree_rec(
                (next, self.grid.get(&next).unwrap().clone()),
                instruction,
            );
            match successor {
                None => return None,
                Some(items) => items.into_iter().for_each(|(key, value)| {
                    collection.insert(key, value);
                }),
            }
        }
        {
            let next_neighbor = neighbor + instruction;
            let successor_neighbor = self.select_vertical_tree_rec(
                (
                    next_neighbor,
                    self.grid.get(&next_neighbor).unwrap().clone(),
                ),
                instruction,
            );
            match successor_neighbor {
                None => return None,
                Some(items) => items.into_iter().for_each(|(key, value)| {
                    collection.insert(key, value);
                }),
            }
        }
        Some(collection)
    }
}

fn get_warehouse_and_instructions(
    input: &str,
) -> (
    Warehouse<DoubleWidthWarehouseCell>,
    Vec<RobotMoveInstruction>,
) {
    let (grid, instructions) = input
        .split_once("\n\n")
        .unwrap_or_else(|| input.split_once("\r\n\r\n").unwrap());

    let warehouse = DoubleWidthWarehouseGrid::from(grid);

    let instructions: Vec<RobotMoveInstruction> = instructions
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Direction::E),
            '<' => Some(Direction::W),
            '^' => Some(Direction::N),
            'v' => Some(Direction::S),
            _ => None,
        })
        .collect();
    (warehouse, instructions)
}

pub fn solve_day_15_part_02(input: &str) -> u32 {
    let (warehouse, instructions) = get_warehouse_and_instructions(input);

    println!("{warehouse}");

    let final_warehouse = instructions
        .into_iter()
        .fold(warehouse, |w, instruction| w.apply_instruction(instruction));

    println!("{final_warehouse}");

    final_warehouse
        .grid
        .into_iter()
        .filter(|(_, cell)| cell == &BoxWest)
        .map(|(p, _)| p.x as u32 + (p.y as u32 * 100))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use crate::util::grid::Direction;

    #[test]
    fn should_solve_day_15_part_02() {
        let input = read_string("./src/day15/input.txt").unwrap();

        let solution = solve_day_15_part_02(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_15_part_02_sample() {
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .trim();

        assert_eq!(9021, solve_day_15_part_02(input));
    }

    #[test]
    fn should_extract_grid() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.O@");
        let grid = warehouse.grid;

        assert_eq!(&Wall, grid.get(&Point::new(0, 0)).unwrap());
        assert_eq!(&Wall, grid.get(&Point::new(1, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(2, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(3, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(4, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(5, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(6, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(7, 0)).unwrap());
        assert_eq!(Point::new(6, 0), warehouse.robot_pos);
    }

    #[test]
    fn should_move_one_box_horizontally() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.O@").apply_instruction(Direction::W);
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(&Wall, grid.get(&Point::new(0, 0)).unwrap());
        assert_eq!(&Wall, grid.get(&Point::new(1, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(2, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(3, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(4, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(5, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(6, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(7, 0)).unwrap());
        assert_eq!(Point::new(5, 0), robot_pos);
    }

    #[test]
    fn should_move_two_boxes_horizontally_left() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.OO@").apply_instruction(Direction::W);
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(&Wall, grid.get(&Point::new(0, 0)).unwrap());
        assert_eq!(&Wall, grid.get(&Point::new(1, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(2, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(3, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(4, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(5, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(6, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(7, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(8, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(9, 0)).unwrap());
        assert_eq!(Point::new(7, 0), robot_pos);
    }

    #[test]
    fn should_move_two_boxes_horizontally_right() {
        let warehouse = DoubleWidthWarehouseGrid::from("#@OO.")
            .apply_instruction(Direction::E)
            .apply_instruction(Direction::E); // robot spawns on the left cell so we have to shift twice
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(&Wall, grid.get(&Point::new(0, 0)).unwrap());
        assert_eq!(&Wall, grid.get(&Point::new(1, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(2, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(3, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(4, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(5, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(6, 0)).unwrap());
        assert_eq!(&BoxWest, grid.get(&Point::new(7, 0)).unwrap());
        assert_eq!(&BoxEast, grid.get(&Point::new(8, 0)).unwrap());
        assert_eq!(&Empty, grid.get(&Point::new(9, 0)).unwrap());
        assert_eq!(Point::new(4, 0), robot_pos);
    }

    #[test]
    fn should_select_tree() {
        let input = "
######
#.@..#
#.O..#
#.O..#
#....#
######"
            .trim();
        let warehouse = DoubleWidthWarehouseGrid::from(input);
        let actual = warehouse.select_vertical_tree_rec((Point::new(4, 2), BoxWest), Direction::S);

        assert_eq!(4, actual.unwrap().len());
    }

    #[test]
    fn should_do_step_by_step() {
        let input = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .trim();

        let (mut warehouse, _instructions) = get_warehouse_and_instructions(input);

        println!("0:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::W);
        println!("1:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::S);
        println!("2:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::S);
        println!("3:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::W);
        println!("4:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::W);
        println!("5:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::N);
        println!("6:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::N);
        println!("7:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::W);
        println!("8:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::W);
        println!("9:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::N);
        println!("10:\n{warehouse}");
        warehouse = warehouse.apply_instruction(Direction::N);
        println!("11:\n{warehouse}");
    }
}
