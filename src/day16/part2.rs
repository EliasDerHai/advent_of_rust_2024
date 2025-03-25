use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::part1::*;
use crate::util::grid::Grid;
use crate::util::point::Point;

#[derive(Debug, Clone, PartialEq)]
struct PathNode {
    p: Point,
    cost: u32,
    parent: Option<Rc<PathNode>>,
    dir: TravelDirection,
}

impl PathNode {
    fn new(p: Point, cost: u32, parent: Rc<PathNode>, dir: TravelDirection) -> Self {
        PathNode {
            p,
            cost,
            parent: Some(parent),
            dir,
        }
    }

    fn default(p: Point) -> Self {
        PathNode {
            p,
            cost: 0,
            parent: None,
            dir: TravelDirection::E,
        }
    }

    fn collect_path(self: &Rc<Self>) -> Vec<Point> {
        let mut path = vec![self.p];
        let mut node: Rc<PathNode> = Rc::clone(self);

        while let Some(parent) = &node.parent {
            path.push(parent.p);
            node = Rc::clone(&parent);
        }

        path.reverse();
        path
    }

    fn dir_to(&self, other: &Point) -> TravelDirection {
        match other {
            x if x == &self.p.right() => TravelDirection::E,
            x if x == &self.p.left() => TravelDirection::W,
            x if x == &self.p.down() => TravelDirection::S,
            x if x == &self.p.up() => TravelDirection::N,
            _ => panic!("Not a neighbor"),
        }
    }
}

/// continuing the parents direction is cheap
fn calc_cost_and_dir(p: &Point, parent: &PathNode) -> (u32, TravelDirection) {
    let new_dir = parent.dir_to(p);
    if parent.dir == new_dir {
        (parent.cost + 1, new_dir)
    } else {
        (parent.cost + 1001, new_dir)
    }
}

fn dijkstra_pathfinding(map: ReindeerOlympicMap) -> Vec<Rc<PathNode>> {
    let start_node = PathNode::default(map.start);

    let mut open: Vec<PathNode> = vec![start_node];
    let mut closed: HashMap<(Point, TravelDirection), u32> = HashMap::new();
    let mut solutions: Vec<Rc<PathNode>> = Vec::new();

    while !open.is_empty() {
        let index = open
            .iter()
            .enumerate()
            .min_by_key(|&(_, node)| node.cost)
            .map(|(i, _)| i)
            .unwrap();
        let curr = Rc::new(open.swap_remove(index));

        let children: Vec<PathNode> = map
            .grid
            .neighbors(&curr.p)
            .filter_map(|(p, &c)| if c == Cells::Empty { Some(p) } else { None })
            .map(|p| {
                let (cost, dir) = calc_cost_and_dir(&p, &curr);
                PathNode::new(p, cost, Rc::clone(&curr), dir)
            })
            .filter(|n| {
                if let Some(&existing) = closed.get(&(n.p, n.dir)) {
                    n.cost <= existing
                } else {
                    true
                }
            })
            .filter(|n| {
                if let Some(existing) = open
                    .iter()
                    .find(|&existing| existing.p == n.p && existing.dir == n.dir)
                {
                    n.cost <= existing.cost
                } else {
                    true
                }
            })
            .collect();

        for n in children.clone() {
            if n.p == map.end {
                solutions.push(Rc::new(n));
            } else {
                open.push(n);
            }
        }

        closed.insert((curr.p, curr.dir), curr.cost);
    }

    solutions
}

pub fn solve_day_16_part_02(input: &str) -> u32 {
    let map = ReindeerOlympicMap::from(input);
    let start = map.start;
    let end = map.end;
    let solutions = dijkstra_pathfinding(map);

    if solutions.is_empty() {
        panic!("Cannot travel to {} from {}", end, start);
    } else {
        let min = solutions.iter().min_by_key(|s| s.cost).unwrap().cost;

        println!("Min cost: {min} - paths found: {}", solutions.len());
        let mut i = 0;
        for solution in &solutions {
            i += 1;
            println!("{i}: {} - cost: {}", solution.p, solution.cost);
        }

        let distinct_points = solutions
            .iter()
            .filter(|s| s.cost == min)
            .flat_map(|s| s.collect_path())
            .collect::<HashSet<Point>>()
            .iter()
            .count();

        distinct_points as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    const TEST_INPUT: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn should_solve_day_16_part_02() {
        let input = read_string("./src/day16/input.txt").unwrap();

        let solution = solve_day_16_part_02(&input);

        println!("{solution}");

        assert_eq!(538, solution);
    }

    #[test]
    fn should_solve_day_16_part_02_sample() {
        assert_eq!(45, solve_day_16_part_02(TEST_INPUT.trim()));
    }

    #[test]
    fn should_find_two_solutions() {
        let map = "
#######
##...E#
#S..###
#######
";
        let solutions = dijkstra_pathfinding(ReindeerOlympicMap::from(map.trim()));

        println!("{:?}", solutions);

        assert_eq!(2, solutions.len());
    }
}
