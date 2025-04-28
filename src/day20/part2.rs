use crate::{
    day16::part1::{Cells, ReindeerOlympicMap},
    util::point::Point,
};

use super::part1::a_star_pathfinding;
use std::sync::LazyLock;

static MANHATTAN_OFFSETS_20: LazyLock<Vec<(Point, u8)>> = LazyLock::new(|| {
    let r = 20i32;
    let mut offsets = Vec::new();
    for dx in -r..=r {
        let max_dy = r - dx.abs();
        for dy in -max_dy..=max_dy {
            let cost = (dx.abs() + dy.abs()) as u8;
            offsets.push((Point::new(dx, dy), cost));
        }
    }
    offsets
});

pub fn solve_day_20_part_02(input: &str, threshold: u32) -> usize {
    let map = ReindeerOlympicMap::from(input);
    let costs = a_star_pathfinding(&map);

    assert_eq!(
        costs.len(),
        map.grid.iter().filter(|(_, v)| **v == Cells::Empty).count()
    );

    let mut counter = 0;
    for (cheat_start_point, cheat_start_cost) in &costs {
        for (offset, offset_cost) in &*MANHATTAN_OFFSETS_20 {
            let neighbor = Point::new(
                cheat_start_point.x + offset.x,
                cheat_start_point.y + offset.y,
            );

            if let Some(&cheat_end_cost) = costs.get(&neighbor) {
                let cheat_benefit = cheat_end_cost
                    .saturating_sub(*cheat_start_cost)
                    .saturating_sub(*offset_cost as u32);

                if cheat_benefit >= threshold {
                    counter += 1;
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_20_part_02() {
        let input = read_string("./src/day20/input.txt").unwrap();

        let solution = solve_day_20_part_02(&input, 100);

        assert_eq!(971737, solution);
    }

    #[test]
    fn should_solve_day_20_part_02_sample() {
        let input: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .trim();
        let expected = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
        assert_eq!(expected, solve_day_20_part_02(input, 50));
    }
}
