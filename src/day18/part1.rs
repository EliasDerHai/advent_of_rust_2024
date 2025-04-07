use crate::util::point::Point;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug)]
pub struct PushdownAutomatonMap {
    start: Point,
    goal: Point,
    corrupt: HashSet<Point>,
    map_length: u8,
}

impl PushdownAutomatonMap {
    pub fn new(input: &str, size: u8, n: u16) -> Self {
        let corrupt = input
            .trim()
            .lines()
            .take(n as usize)
            .map(|l| l.trim().split_once(",").expect("should have comma"))
            .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        let goal_x_y: i128 = size as i128 - 1;
        PushdownAutomatonMap {
            start: Point::new(0, 0),
            goal: Point::new(goal_x_y, goal_x_y),
            corrupt,
            map_length: size,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathNode {
    p: Point,
    g: u32,
    h: u32,
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_f = self.f();
        let other_f = other.f();

        let o = if self_f < other_f {
            Ordering::Less
        } else if self_f > other_f {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        Some(o)
    }
}
impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_f = self.f();
        let other_f = other.f();

        if self_f < other_f {
            Ordering::Less
        } else if self_f > other_f {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PathNode {
    fn new(p: Point, g: u32, h: u32) -> Self {
        PathNode { p, g, h }
    }

    fn f(&self) -> u32 {
        self.g + self.h
    }
}

fn g(parent: &PathNode) -> u32 {
    parent.g + 1
}

fn h(n: &Point, goal: &Point) -> u32 {
    let delta_x = (n.x - goal.x).abs() as u32;
    let delta_y = (n.y - goal.y).abs() as u32;
    delta_x + delta_y
}

pub fn a_star_pathfinding(map: PushdownAutomatonMap) -> Option<u32> {
    let start_node = PathNode::new(map.start, 0, h(&map.start, &map.goal));

    let mut open = BinaryHeap::new();
    open.push(Reverse(start_node));

    // best g cost found so far for each point.
    let mut best_cost: HashMap<Point, u32> = HashMap::new();
    best_cost.insert(map.start, 0);

    let mut closed: HashSet<Point> = HashSet::new();

    while let Some(Reverse(current)) = open.pop() {
        let curr = current;
        closed.insert(curr.p);

        if curr.p == map.goal {
            return Some(curr.f());
        }

        curr.p
            .neighbors()
            .into_iter()
            .filter(|p| {
                !map.corrupt.contains(p)
                    && p.x >= 0
                    && p.x < map.map_length as i128
                    && p.y >= 0
                    && p.y < map.map_length as i128
            })
            .filter(|p| !closed.contains(p))
            .filter_map(|p| {
                let new_g = g(&curr); // new cost (typically, curr.g + step_cost)
                if best_cost.get(&p).map_or(true, |&c| new_g < c) {
                    best_cost.insert(p, new_g);
                    let new_h = h(&p, &map.goal);
                    let child = PathNode::new(p, new_g, new_h);
                    Some(Reverse(child))
                } else {
                    None
                }
            })
            .for_each(|child| open.push(child));
    }
    None
}

pub fn solve_day_18_part_01(input: &str, map_size: u8, n: u16) -> u32 {
    let map = PushdownAutomatonMap::new(input, map_size, n);
    a_star_pathfinding(map).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_18_part_01() {
        let input = read_string("./src/day18/input.txt").unwrap();

        let solution = solve_day_18_part_01(&input, 71, 1024);

        assert_eq!(288, solution);
    }

    #[test]
    fn should_solve_day_18_part_01_sample() {
        let input = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1"
        .trim();

        assert_eq!(22, solve_day_18_part_01(input, 7, 12));
    }
}
