#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use crate::util::grid::Grid;
use crate::util::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cells {
    Wall,
    Empty,
}

struct ReindeerOlympicMap {
    start: Point,
    end: Point,
    grid: Grid<Cells>,
}

impl From<&str> for ReindeerOlympicMap {
    fn from(value: &str) -> Self {
        let grid = Grid::from(value.trim());
        let start: Point = *grid.iter().find(|(_, &c)| c == 'S').unwrap().0;
        let end: Point = *grid.iter().find(|(_, &c)| c == 'E').unwrap().0;

        let grid = grid.map(|c| match c {
            '.' | 'S' | 'E' => Cells::Empty,
            '#' => Cells::Wall,
            other => panic!("Did not expect '{other}' in map input"),
        });

        ReindeerOlympicMap { start, end, grid }
    }
}

/// we have to keep track of the traveldirection for each node in order to run the g-function
#[derive(Debug, Clone, Copy, PartialEq)]
enum TravelDirection {
    N,
    E,
    S,
    W,
}

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

    fn collect_path(&self) -> Vec<Point> {
        let mut path = Vec::new();
        let mut node = *self;

        loop {
            path.push(node.p);
            match &node.parent {
                Some(parent) => node = Rc::clone(parent),
                None => break,
            }
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

fn dijkstra_pathfinding(map: ReindeerOlympicMap) -> Option<u32> {
    let start_node = PathNode::default(map.start);

    let mut open: Vec<PathNode> = vec![start_node];
    let mut closed: HashMap<Point, u32> = HashMap::new();
    let mut solutions: Vec<Rc<PathNode>> = Vec::new();

    while !open.is_empty() {
        let index = open
            .iter()
            .enumerate()
            .min_by_key(|&(_, node)| node.cost)
            .map(|(i, _)| i)
            .unwrap();
        let curr = open.swap_remove(index);

        if curr.p == map.end {
            solutions.push(Rc::new(curr.clone()));
        }

        let children: Vec<PathNode> = map
            .grid
            .neighbors(&curr.p)
            .filter_map(|(p, &c)| if c == Cells::Empty { Some(p) } else { None })
            .filter(|&p| {
                if let Some(&existing) = closed.get(&p) {
                    existing > curr.cost
                } else {
                    true
                }
            })
            .map(|p| {
                let (cost, dir) = calc_cost_and_dir(&p, &curr);
                PathNode::new(p, cost, Rc::new(curr.clone()), dir)
            })
            .filter(|n| {
                if let Some(existing) = open.iter().find(|&existing| existing.p == n.p) {
                    if existing.cost < n.cost {
                        return false;
                    }
                }
                true
            })
            .collect();

        for n in children {
            if n.p == map.end {
                solutions.push(Rc::new(n.clone()));
            } else {
                open.push(n.clone());
            }
        }

        closed.insert(curr.p, curr.cost);
    } 

    if solutions.is_empty() {
        eprintln!("Cannot travel to {} from {}", map.end, map.start);
        None
    } else {
        println!("{:?}", solutions.len());
        let min = solutions.iter().min_by_key(|s| s.cost).unwrap().cost;
        Some(solutions.iter().filter(|s| s.cost == min).map(|s| **s.collect_path()).count() as u32
    }
}

pub fn solve_day_16_part_02(input: &str) -> u32 {
    let map = ReindeerOlympicMap::from(input);
    dijkstra_pathfinding(map).unwrap()
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
    }

    #[test]
    fn should_solve_day_16_part_02_sample() {
        assert_eq!(45, solve_day_16_part_02(TEST_INPUT.trim()));
    }
}
