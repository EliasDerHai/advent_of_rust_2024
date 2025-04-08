use crate::day16::part1::*;
use crate::util::point::Point;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::part1::{a_star_pathfinding, h};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CheatingStep {
    NotYet,
    CheatActive,
    Cheated,
}

#[derive(Debug, Clone, Copy, Hash)]
struct Cheating {
    start: Option<Point>,
    end: Option<Point>,
    cheating_seconds_spent: u8,
}

impl Cheating {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
            cheating_seconds_spent: 0,
        }
    }

    fn start(point_before_cheating: Point) -> Self {
        Self {
            start: Some(point_before_cheating),
            end: None,
            cheating_seconds_spent: 0,
        }
    }

    fn keep_cheating(&self, next_point: Point) -> Self {
        let next_cheating_seconds_spent = self.cheating_seconds_spent + 1;
        let next_end = if next_cheating_seconds_spent == 20 {
            Some(next_point)
        } else {
            None
        };
        Self {
            start: self.start,
            end: next_end,
            cheating_seconds_spent: next_cheating_seconds_spent,
        }
    }

    fn cheating_state(&self) -> CheatingStep {
        match self {
            Cheating {
                start: None,
                end: None,
                cheating_seconds_spent: 0,
            } => CheatingStep::NotYet,
            Cheating {
                start: Some(_),
                end: None,
                cheating_seconds_spent: _,
            } => CheatingStep::CheatActive,
            Cheating {
                start: Some(_),
                end: Some(_),
                cheating_seconds_spent: _,
            } => CheatingStep::Cheated,
            _ => panic!("Illegal state"),
        }
    }
}

impl PartialEq for Cheating {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}
impl Eq for Cheating {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathNode {
    p: Point,
    f: u32,
    g: u32,
    h: u32,
    cheating: Cheating,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| other.h.cmp(&self.h))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PathNode {
    fn new(p: Point, g: u32, h: u32, cheating: Cheating) -> Self {
        PathNode {
            p,
            g,
            h,
            f: g + h,
            cheating,
        }
    }
}

fn a_star_pathfinding_distribution(
    map: &ReindeerOlympicMap,
    cost_map: &HashMap<Point, u32>,
    offset: u32,
    lowest_cost_no_cheating: u32,
) -> HashMap<Cheating, u32> {
    let start_node = PathNode::new(map.start, 0, h(&map.start, &map.end), Cheating::new());

    let mut open = BinaryHeap::new();
    open.push(Reverse(start_node));

    let mut closed: HashSet<(Point, Cheating)> = HashSet::new();
    let mut cost_per_cheat: HashMap<Cheating, u32> = HashMap::new();

    while let Some(Reverse(current)) = open.pop() {
        closed.insert((current.p, current.cheating));

        if current.p == map.end {
            cost_per_cheat.insert(current.cheating, current.g);
        }

        map.grid
            .neighbors(&current.p)
            .into_iter()
            .filter_map(|(p, c)| match c {
                Cells::Wall => match current.cheating.cheating_state() {
                    CheatingStep::NotYet => Some((p, Cheating::start(current.p))),
                    CheatingStep::CheatActive => Some((p, current.cheating.keep_cheating(p))),
                    CheatingStep::Cheated => None,
                },
                Cells::Empty => match current.cheating.cheating_state() {
                    CheatingStep::CheatActive => Some((p, current.cheating.keep_cheating(p))),
                    _ => Some((p, current.cheating)),
                },
            })
            .filter(|(p, c)| !closed.contains(&(*p, *c)))
            .for_each(|(p, c)| {
                let new_g = current.g + 1;
                let new_h = h(&p, &map.end);
                let child = PathNode::new(p, new_g, new_h, c);

                if child.cheating.cheating_state() == CheatingStep::Cheated
                    && cost_map.contains_key(&p)
                {
                    let best_cost_without_cheating = *cost_map.get(&p).unwrap();
                    if child.g + offset <= best_cost_without_cheating {
                        let cost = child.g + lowest_cost_no_cheating - best_cost_without_cheating;
                        cost_per_cheat.insert(child.cheating, cost);
                    }
                } else {
                    open.push(Reverse(child))
                }
            });
    }

    cost_per_cheat
        .into_iter()
        .filter(|(_, v)| *v <= lowest_cost_no_cheating - offset)
        .collect()
}

pub fn solve_day_20_part_02(input: &str, threshold: u32) -> usize {
    let map = ReindeerOlympicMap::from(input);
    let costs = a_star_pathfinding(&map).collect_cost_map();
    let mut cheats: HashSet<(Point, Point)> = HashSet::new();

    assert_eq!(
        costs.len(),
        map.grid.iter().filter(|(_, v)| **v == Cells::Empty).count()
    );

    for (cheat_start_point, cheat_start_cost) in &costs {
        cheat_start_point
            .proximity_manhattan(20)
            .into_iter()
            //.filter(|cheat_end_point| !cheats.contains(&(*cheat_start_point, *cheat_end_point)))
            .filter_map(|p| costs.get(&p).map(|cost| (p, *cost)))
            .for_each(|(cheat_end_point, cheat_end_cost)| {
                let cheat_benefit = cheat_end_cost
                    .saturating_sub(*cheat_start_cost)
                    .saturating_sub(h(cheat_start_point, &cheat_end_point));

                if cheat_benefit >= threshold {
                    cheats.insert((*cheat_start_point, cheat_end_point));
                }
            });
    }

    cheats.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    const SAMPLE_MAZE: &str = "
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
###############";

    #[test]
    fn should_solve_day_20_part_02() {
        let input = read_string("./src/day20/input.txt").unwrap();

        let solution = solve_day_20_part_02(&input, 100);

        assert_eq!(1321, solution);
    }

    #[test]
    fn should_solve_day_20_part_02_sample() {
        let expected = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
        assert_eq!(expected, solve_day_20_part_02(SAMPLE_MAZE, 50));
    }

    //   #[test]
    //   fn should_find_best_solution_with_cheating() {
    //       let map = ReindeerOlympicMap::from(SAMPLE_MAZE.trim());
    //       let mut cost_map = HashMap::new();
    //       cost_map.insert(map.end, 84);
    //       let cost = a_star_pathfinding_distribution(&map, &cost_map, 50, 84);
    //
    //       // todo print costs grouped by value and sorted by max
    //       let mut counts: HashMap<u32, u32> = HashMap::new();
    //       for &value in cost.values() {
    //           *counts.entry(value).or_insert(0) += 1;
    //       }
    //
    //       let mut sorted_counts: Vec<(u32, u32)> = counts.into_iter().collect();
    //       sorted_counts.sort_by_key(|&(key, _)| key);
    //
    //       let lowest = *cost.iter().min_by_key(|(_, cost)| *cost).unwrap().1;
    //       assert_eq!(8, lowest);
    //   }
}
