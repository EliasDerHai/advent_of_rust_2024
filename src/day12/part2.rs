use std::collections::HashSet;
use std::time::Instant;

use crate::day12::part1::get_regions;
use crate::util::grid::Grid;
use crate::util::point::Point;

pub fn solve_day_12_part_02(input: &str) -> usize {
    let grid = Grid::from(input);

    let i = Instant::now();
    let regions = get_regions(&grid);
    println!("bfs-flood-fill took {}ms", i.elapsed().as_millis());

    let i = Instant::now();
    let c = regions
        .iter()
        .map(|r| {
            let edge_count = get_edge_count(r.token, &r.area, &grid);

            let cost = r.get_area() * edge_count;
            // println!("{} - size: {} - edges: {} -> {}", r.token, r.get_area(), edge_count, cost);
            cost
        })
        .sum();
    println!("edge-counting took {}ms", i.elapsed().as_millis());
    c
}

fn get_edge_count(own: char, edge_points: &HashSet<Point>, grid: &Grid<char>) -> usize {
    edge_points
        .iter()
        .map(|e| {
            let mut count = 0;
            // couldn't figure out edge detection on my own so I took inspiration from:
            // https://blog.jverkamp.com/2024/12/12/aoc-2024-day-12-edginator/
            for xd in [-1, 1].iter() {
                for yd in [-1, 1].iter() {
                    // *x
                    // xC case

                    // Along the x and y directions match
                    let neighbor_xd = *grid.get(&Point::new(e.x + *xd, e.y)).unwrap_or(&'?');
                    let neighbor_yd = *grid.get(&Point::new(e.x, e.y + *yd)).unwrap_or(&'?');

                    if own != neighbor_xd && own != neighbor_yd {
                        count += 1;
                    }

                    // xC
                    // CC case

                    let neighbor_both = *grid.get(&Point::new(e.x + *xd, e.y + *yd)).unwrap_or(&'?');

                    if own == neighbor_xd && own == neighbor_yd && own != neighbor_both {
                        count += 1;
                    }
                }
            }

            count
        }).sum()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_12_part_02() {
        let input = read_string("./src/day12/input.txt").unwrap();

        let solution = solve_day_12_part_02(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_12_part_02_sample() {
        let input = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE".trim();

        assert_eq!(236, solve_day_12_part_02(input));
    }

    #[test]
    fn should_solve_day_12_part_02_sample_2() {
        let input = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA".trim();

        assert_eq!(368, solve_day_12_part_02(input));
    }

    #[test]
    fn should_solve_day_12_part_02_sample_3() {
        let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE".trim();

        assert_eq!(1206, solve_day_12_part_02(input));
    }
}
