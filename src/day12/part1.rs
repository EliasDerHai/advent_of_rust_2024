use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use crate::util::grid::CharGrid;
use crate::util::point::Point;

#[derive(Debug)]
pub struct Region {
    pub(crate) token: char,
    pub(crate) area: HashSet<Point>,
    // pub(crate) edges: HashSet<Point>,
}

impl Region {
    fn new(c: char) -> Self {
        Region {
            token: c,
            area: HashSet::new(),
            // edges: HashSet::new(),
        }
    }

    fn get_perimeter(&self, grid: &CharGrid) -> usize {
        self.area
            .iter()
            .map(|p| {
                grid.neighbors_incl_outs(p)
                    .filter(|(_, nc)| *nc != self.token)
                    .map(|tuple| {
                        tuple
                    })
                    .count()
            })
            .sum()
    }

    pub(crate) fn get_area(&self) -> usize {
        self.area.len()
    }
}

impl CharGrid {
    pub fn neighbors_incl_outs<'a>(&'a self, p: &'a Point)
                                   -> impl Iterator<Item=(Point, char)> + 'a {
        [p.left(), p.right(), p.up(), p.down()]
            .into_iter()
            .map(|n| {
                let char_or_question_mark = self.get(&n).unwrap_or(&'?');
                (n, *char_or_question_mark)
            })
    }
}

pub fn get_regions(grid: &CharGrid) -> Vec<Region> {
    let mut indexed: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    for (p, c) in grid.iter() {
        if indexed.contains(p) {
            continue;
        }

        let mut next_region = Region::new(*c);
        let mut queue = VecDeque::new();
        queue.push_back(*p);

        while let Some(curr) = queue.pop_front() {
            next_region.area.insert(curr);
            if indexed.contains(&curr) { continue; }
            indexed.insert(curr);
            grid.neighbors_incl_outs(&curr)
                .for_each(|(p, c)| {
                    let is_part_of_region = c == next_region.token;

                    if !is_part_of_region {
                        // next_region.edges.insert(curr);
                    } else if !indexed.contains(&p) {
                        queue.push_back(p);
                    }
                });
        }

        regions.push(next_region);
    }
    regions
}

pub fn solve_day_12_part_01(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let i = Instant::now();
    let regions = get_regions(&grid);
    println!("bfs-flood-fill took {}ms", i.elapsed().as_millis());

    let i = Instant::now();
    let c = regions
        .iter()
        .map(|r| {
            let cost = r.get_area() * r.get_perimeter(&grid);
            cost
        })
        .sum();
    println!("edge-counting took {}ms", i.elapsed().as_millis());
    c
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn log_size_of_input() {
        let input = read_string("./src/day12/input.txt").unwrap();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        println!("len {}", input.len());
        println!("{} x {}", width, height);
    }

    #[test]
    fn should_solve_day_12_part_01() {
        let input = read_string("./src/day12/input.txt").unwrap();

        let solution = solve_day_12_part_01(&input);

        println!("{solution}");
        assert_eq!(1518548, solution);
    }

    #[test]
    fn should_solve_day_12_part_01_sample() {
        let input = "
AAAA
BBCD
BBCC
EEEC".trim();

        assert_eq!(140, solve_day_12_part_01(input));
    }


    #[test]
    fn should_solve_day_12_part_01_sample_2() {
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

        assert_eq!(1930, solve_day_12_part_01(input));
    }
}
