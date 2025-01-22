use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use crate::util::point::Point;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Altitude(u8);

impl Display for Altitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Altitude {
    fn is_trail_head(&self) -> bool {
        self.0 == 0
    }

    fn is_trail_goal(&self) -> bool {
        self.0 == 9
    }

    fn is_climbable(&self, from: &Altitude) -> bool {
        self.0 == from.0 + 1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Trail {
    start: Point,
    curr_pos: Point,
    curr_alt: Altitude,
}

impl Trail {
    pub(crate) fn new(start: Point) -> Self {
        Trail {
            start,
            curr_pos: start,
            curr_alt: Altitude(0),
        }
    }

    fn new_set(start: Point, curr_pos: Point, curr_alt: Altitude) -> Self {
        Trail {
            start,
            curr_pos,
            curr_alt,
        }
    }

    pub(crate) fn is_completed(&self) -> bool {
        self.curr_alt.is_trail_goal()
    }
}

pub struct TopographicMap {
    map: HashMap<Point, Altitude>,
}

impl From<&str> for TopographicMap {
    fn from(value: &str) -> Self {
        let map: HashMap<Point, Altitude> =
            value
                .lines()
                .enumerate()
                .into_iter()
                .flat_map(|(y, line)| line
                    .chars()
                    // .filter(|c| c.is_digit(10))
                    .enumerate()
                    .map(move |(x, c)|
                        {
                            let p = Point::new(x as i16, y as i16);
                            let height = c.to_digit(10).unwrap_or(11) as u8;
                            (p, Altitude(height))
                        }
                    )
                )
                .collect();
        TopographicMap { map }
    }
}

impl TopographicMap {
    pub fn starting_points(&self) -> Vec<(&Point, &Altitude)> {
        self.map.iter().filter(|(_p, a)| a.is_trail_head()).collect()
    }

    fn traverse(&self) -> usize {
        let mut open: HashSet<Trail> = self
            .starting_points()
            .into_iter()
            .map(|(&p, _)| Trail::new(p))
            .collect();

        while open.iter().any(|trail| !trail.is_completed()) {
            open = open
                .into_iter()
                .flat_map(|trail| self.travel_adjacent(trail))
                .collect();
        }

        open.len()
    }

    pub fn travel_adjacent(&self, trail: Trail) -> Vec<Trail> {
        let p = trail.curr_pos;
        vec![p.left(), p.right(), p.up(), p.down()]
            .into_iter()
            .flat_map(|next| self.map.get_key_value(&next))
            .filter(|(_, a2)| a2.is_climbable(&trail.curr_alt))
            .map(|(p2, a2)| Trail::new_set(trail.start, *p2, *a2))
            .collect()
    }
}

pub fn solve_day_10_part_01(input: String) -> usize {
    TopographicMap::from(input.as_str()).traverse()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_10_part_01() {
        let input = read_string("./src/day10/input.txt").unwrap();

        let solution = solve_day_10_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_10_part_01_sample() {
        assert_eq!(2, solve_day_10_part_01("
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9".trim().to_string()));

        assert_eq!(36, solve_day_10_part_01("
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732".trim().to_string()));
    }
}
