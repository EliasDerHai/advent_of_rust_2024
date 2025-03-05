use crate::day15::part1::*;
use crate::util::grid::Grid;
use crate::util::point::Point;
use std::collections::HashMap;

type DoubleWidthWarehouseGrid = Grid<DoubleWidthWarehouseCell>;

#[derive(Debug, PartialEq, Clone)]
enum DoubleWidthWarehouseCell {
    Wall,
    BoxEast,
    BoxWest,
    Empty,
}

impl From<&str> for DoubleWidthWarehouseGrid {
    fn from(value: &str) -> Self {
        DoubleWidthWarehouseGrid::from(
            Grid::<char>::from(value)
                .map(WarehouseCellParsing::from)
                .map(WarehouseCell::from),
        )
    }
}

impl From<WarehouseGrid> for DoubleWidthWarehouseGrid {
    fn from(value: WarehouseGrid) -> Self {
        let width_multiplier = (2, 1);
        let mut double_warehouse_grid: HashMap<Point, DoubleWidthWarehouseCell> = HashMap::new();

        value.into_iter().for_each(|(point, cell)| {
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

        DoubleWidthWarehouseGrid::new(double_warehouse_grid)
    }
}

pub fn solve_day_15_part_02(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

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
        let grid = DoubleWidthWarehouseGrid::from("#.O@");

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
    }
}
