use crate::day15::part1::*;
use crate::util::grid::Grid;
use crate::util::point::Point;
use std::collections::HashMap;

type DoubleWidthWarehouseGrid = Warehouse<DoubleWidthWarehouseCell>;

#[derive(Debug, PartialEq, Clone)]
enum DoubleWidthWarehouseCell {
    Wall,
    BoxEast,
    BoxWest,
    Empty,
}

impl DoubleWidthWarehouseCell {
    /// horizontally flip cells (only really makes sense for boxes)
    fn mirror(&self) -> Self {
        match self {
            DoubleWidthWarehouseCell::BoxEast => DoubleWidthWarehouseCell::BoxWest,
            DoubleWidthWarehouseCell::BoxWest => DoubleWidthWarehouseCell::BoxEast,
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
            * (2, 0); // up-scaling

        DoubleWidthWarehouseGrid::from((char_grid.map(WarehouseCell::from), robot_pos))
    }
}

impl From<(Grid<WarehouseCell>, Point)> for DoubleWidthWarehouseGrid {
    fn from((grid, robot_pos): (Grid<WarehouseCell>, Point)) -> Self {
        let width_multiplier = (2, 1);
        let mut double_warehouse_grid: HashMap<Point, DoubleWidthWarehouseCell> = HashMap::new();

        grid.into_iter().for_each(|(point, cell)| {
            let left = point * width_multiplier; // upscaled cell
            let right = left.right(); // upscaled cell's right neighbor
            match cell {
                WarehouseCell::Wall => {
                    double_warehouse_grid.insert(left, DoubleWidthWarehouseCell::Wall);
                    double_warehouse_grid.insert(right, DoubleWidthWarehouseCell::Wall);
                }
                WarehouseCell::Box => {
                    double_warehouse_grid.insert(left, DoubleWidthWarehouseCell::BoxWest);
                    double_warehouse_grid.insert(right, DoubleWidthWarehouseCell::BoxEast);
                }
                WarehouseCell::Empty => {
                    double_warehouse_grid.insert(left, DoubleWidthWarehouseCell::Empty);
                    double_warehouse_grid.insert(right, DoubleWidthWarehouseCell::Empty);
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
            DoubleWidthWarehouseCell::Wall => self,
            DoubleWidthWarehouseCell::Empty => {
                self.robot_pos = next;
                self
            }
            DoubleWidthWarehouseCell::BoxEast | DoubleWidthWarehouseCell::BoxWest => {
                self.attempt_to_move_box((next, next_cell.clone()), instruction)
            }
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

    fn attempt_to_move_box_vertically(
        self,
        _first_box: (Point, DoubleWidthWarehouseCell),
        _instruction: RobotMoveInstruction,
    ) -> Self {
        todo!()
    }

    fn attempt_to_move_box_horizontally(
        self,
        first_box: (Point, DoubleWidthWarehouseCell),
        instruction: RobotMoveInstruction,
    ) -> Self {
        // represents the last element of the stack to shift
        // initialized as box and incrementally checked if "row" ends in wall or free space
        let mut end_of_stack = first_box.clone();

        while end_of_stack.1 == DoubleWidthWarehouseCell::BoxEast
            || end_of_stack.1 == DoubleWidthWarehouseCell::BoxWest
        {
            let next_point = end_of_stack.0 + instruction;
            let next_cell = self.grid.get(&next_point).unwrap().clone();
            end_of_stack = (next_point, next_cell);
        }

        match end_of_stack {
            (_, DoubleWidthWarehouseCell::Wall) => self,
            (_, DoubleWidthWarehouseCell::Empty) => {
                self.move_box_horizontally(first_box, end_of_stack)
            }
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
        println!("{start_x} - {end_x}");
        for x in start_x + 1..end_x {
            let p = Point::new(x, first_point.y);
            let original = self.grid.get(&p).unwrap();
            let flipped = original.mirror();
            println!("flipped {:?} ({:?}->{:?}", p, original, flipped);
            self.grid.set(p, flipped);
        }

        self.grid.set(first_point, DoubleWidthWarehouseCell::Empty);
        self.robot_pos = first_point;
        self.grid.set(end_point, first_cell.mirror());
        self
    }
}

pub fn solve_day_15_part_02(_input: &str) -> u32 {
    todo!()
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
        let input = "".trim();

        assert_eq!(0, solve_day_15_part_02(input));
    }

    #[test]
    fn should_extract_grid() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.O@");
        let grid = warehouse.grid;

        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(0, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(1, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(2, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(3, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(4, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(5, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(6, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(7, 0)).unwrap()
        );
        assert_eq!(Point::new(6, 0), warehouse.robot_pos);
    }

    #[test]
    fn should_move_one_box_horizontally() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.O@").apply_instruction(Direction::W);
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(0, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(1, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(2, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(3, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(4, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(5, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(6, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(7, 0)).unwrap()
        );
        assert_eq!(Point::new(5, 0), robot_pos);
    }

    #[test]
    fn should_move_two_boxes_horizontally_left() {
        let warehouse = DoubleWidthWarehouseGrid::from("#.OO@").apply_instruction(Direction::W);
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(0, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(1, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(2, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(3, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(4, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(5, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(6, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(7, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(8, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(9, 0)).unwrap()
        );
        assert_eq!(Point::new(7, 0), robot_pos);
    }

    #[test]
    fn should_move_two_boxes_horizontally_right() {
        let warehouse = DoubleWidthWarehouseGrid::from("#@OO.")
            .apply_instruction(Direction::E)
            .apply_instruction(Direction::E); // robot spawns on the left cell so we have to shift twice
        let grid = warehouse.grid;
        let robot_pos = warehouse.robot_pos;

        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(0, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Wall,
            grid.get(&Point::new(1, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(2, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(3, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(4, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(5, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(6, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxWest,
            grid.get(&Point::new(7, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::BoxEast,
            grid.get(&Point::new(8, 0)).unwrap()
        );
        assert_eq!(
            &DoubleWidthWarehouseCell::Empty,
            grid.get(&Point::new(9, 0)).unwrap()
        );
        assert_eq!(Point::new(4, 0), robot_pos);
    }
}
