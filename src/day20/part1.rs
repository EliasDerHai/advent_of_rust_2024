use crate::day16::part1::*;
use crate::util::point::Point;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CheatingStep {
    NotYet,
    FirstCheatingMove,
    Cheated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cheating {
    start: Option<Point>,
    end: Option<Point>,
}

impl Cheating {
    fn new(start: Option<Point>, end: Option<Point>) -> Self {
        Self { start, end }
    }

    fn cheating_state(&self) -> CheatingStep {
        match self {
            Cheating {
                start: None,
                end: None,
            } => CheatingStep::NotYet,
            Cheating {
                start: Some(_),
                end: None,
            } => CheatingStep::FirstCheatingMove,
            Cheating {
                start: Some(_),
                end: Some(_),
            } => CheatingStep::Cheated,
            Cheating {
                start: None,
                end: Some(_),
            } => panic!("Illegal state"),
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathNodeWithParent {
    p: Point,
    f: u32,
    g: u32,
    h: u32,
    parent: Option<Rc<PathNodeWithParent>>,
}

impl Ord for PathNodeWithParent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| other.h.cmp(&self.h))
    }
}

impl PartialOrd for PathNodeWithParent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PathNodeWithParent {
    fn new_start(p: Point, g: u32, h: u32) -> Self {
        PathNodeWithParent {
            p,
            g,
            h,
            f: g + h,
            parent: None,
        }
    }

    fn new(p: Point, g: u32, h: u32, parent: Rc<PathNodeWithParent>) -> Self {
        PathNodeWithParent {
            p,
            g,
            h,
            f: g + h,
            parent: Some(parent),
        }
    }

    fn collect_cost_map(self: &Rc<Self>) -> HashMap<Point, u32> {
        let mut node: Rc<PathNodeWithParent> = Rc::clone(self);
        let mut map: HashMap<Point, u32> = HashMap::new();
        map.insert(self.p, self.g);

        while let Some(parent) = &node.parent {
            map.insert(parent.p, parent.g);
            node = Rc::clone(&parent);
        }

        map
    }
}

fn h(n: &Point, goal: &Point) -> u32 {
    let delta_x = (n.x - goal.x).abs() as u32;
    let delta_y = (n.y - goal.y).abs() as u32;
    delta_x + delta_y
}

fn a_star_pathfinding(map: &ReindeerOlympicMap) -> Rc<PathNodeWithParent> {
    let start_node = Rc::new(PathNodeWithParent::new_start(
        map.start,
        0,
        h(&map.start, &map.end),
    ));

    let mut open: BinaryHeap<Reverse<Rc<PathNodeWithParent>>> = BinaryHeap::new();
    open.push(Reverse(start_node.clone()));

    // best g cost found so far for each point.
    let mut best_cost: HashMap<Point, u32> = HashMap::new();
    best_cost.insert(map.start, 0);

    let mut closed: HashSet<Point> = HashSet::new();

    while let Some(Reverse(current)) = open.pop() {
        closed.insert(current.p);

        if current.p == map.end {
            return current;
        }

        map.grid
            .neighbors(&current.p)
            .into_iter()
            .filter_map(|(p, c)| match c {
                Cells::Wall => None,
                Cells::Empty => Some(p),
            })
            .filter(|p| !closed.contains(p))
            .filter_map(|p| {
                let new_g = current.g + 1;
                if best_cost.get(&p).map_or(true, |&c| new_g < c) {
                    best_cost.insert(p, new_g);
                    let new_h = h(&p, &map.end);
                    let child = PathNodeWithParent::new(p, new_g, new_h, current.clone());
                    Some(Reverse(Rc::new(child)))
                } else {
                    None
                }
            })
            .for_each(|child| open.push(child));
    }

    panic!("No solution found for the maze");
}

fn a_star_pathfinding_distribution(
    map: &ReindeerOlympicMap,
    cost_map: &HashMap<Point, u32>,
    offset: u32,
    lowest_cost_no_cheating: u32,
) -> HashMap<Cheating, u32> {
    let start_node = PathNode::new(
        map.start,
        0,
        h(&map.start, &map.end),
        Cheating::new(None, None),
    );

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
                    CheatingStep::NotYet => Some((p, Cheating::new(Some(current.p), None))),
                    CheatingStep::FirstCheatingMove => None,
                    CheatingStep::Cheated => None,
                },
                Cells::Empty => match current.cheating.cheating_state() {
                    CheatingStep::FirstCheatingMove => {
                        Some((p, Cheating::new(current.cheating.start, Some(p))))
                    }
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

pub fn solve_day_20_part_01(input: &str, offset: u32) -> usize {
    let map = ReindeerOlympicMap::from(input);
    let cost_map = a_star_pathfinding(&map).collect_cost_map();
    let lowest_cost_no_cheating = *cost_map.values().max().unwrap();
    let cost_per_cheat =
        a_star_pathfinding_distribution(&map, &cost_map, offset, lowest_cost_no_cheating);
    let cheats: HashSet<Cheating> = cost_per_cheat.keys().map(|c| *c).collect();
    cheats.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_minimal_example() {
        let map = "
#####
#S..#
###.#
#E..#
#####"
            .trim();
        let actual = solve_day_20_part_01(&map, 2);

        assert_eq!(2, actual);
    }

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
    fn should_solve_day_20_part_01() {
        let input = read_string("./src/day20/input.txt").unwrap();

        let solution = solve_day_20_part_01(&input, 100);

        assert_eq!(1321, solution);
    }

    #[test]
    fn should_solve_example() {
        let solution = solve_day_20_part_01(SAMPLE_MAZE, 1);

        assert_eq!(44, solution);
    }

    #[test]
    fn should_find_best_solution_without_cheating() {
        let map = ReindeerOlympicMap::from(SAMPLE_MAZE.trim());
        let cost = a_star_pathfinding(&map);
        assert_eq!(84, cost.f);
    }

    #[test]
    fn should_find_best_solution_without_cheating_variation() {
        let map = ReindeerOlympicMap::from(
            "
###############
#...#.........#
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
                .trim(),
        );
        let cost = a_star_pathfinding(&map);
        assert_eq!(72, cost.f);
    }

    #[test]
    fn should_solve_serpentines() {
        let map = "
###############
#S............#
#############.#
#.............#
#.#############
#.............#
#############.#
#.............#
#.#############
#.............#
#############.#
#.............#
#.#############
#............E#
###############"
            .trim();
        let actual = solve_day_20_part_01(&map, 1);

        assert_eq!(12 * 6, actual);
    }

    #[test]
    fn should_find_best_solution_with_cheating() {
        let map = ReindeerOlympicMap::from(SAMPLE_MAZE.trim());
        let mut cost_map = HashMap::new();
        cost_map.insert(map.end, 84);
        let cost = a_star_pathfinding_distribution(&map, &cost_map, 0, 84);

        // todo print costs grouped by value and sorted by max
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for &value in cost.values() {
            *counts.entry(value).or_insert(0) += 1;
        }

        let mut sorted_counts: Vec<(u32, u32)> = counts.into_iter().collect();
        sorted_counts.sort_by_key(|&(key, _)| key);

        let lowest = *cost.iter().min_by_key(|(_, cost)| *cost).unwrap().1;
        assert_eq!(20, lowest);
    }
}
