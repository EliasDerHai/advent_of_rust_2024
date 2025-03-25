use crate::util::grid::Grid;
use crate::util::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Cells {
    Wall,
    Empty,
}

pub(super) struct ReindeerOlympicMap {
    pub(super) start: Point,
    pub(super) end: Point,
    pub(super) grid: Grid<Cells>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum TravelDirection {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, PartialEq)]
struct PathNode {
    p: Point,
    g: u32,
    h: u32,
    dir: TravelDirection,
}

impl PathNode {
    fn new(p: Point, g: u32, h: u32, dir: TravelDirection) -> Self {
        PathNode { p, g, h, dir }
    }

    fn f(&self) -> u32 {
        self.g + self.h
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
fn g(p: &Point, parent: &PathNode) -> (u32, TravelDirection) {
    let new_dir = parent.dir_to(p);
    if parent.dir == new_dir {
        (parent.g + 1, new_dir)
    } else {
        (parent.g + 1001, new_dir)
    }
}

fn h(n: &Point, goal: &Point) -> u32 {
    let delta_x = (n.x - goal.x).abs() as u32;
    let delta_y = (n.y - goal.y).abs() as u32;
    delta_x + delta_y
}

fn a_star_pathfinding(map: ReindeerOlympicMap) -> Option<u32> {
    let start_node = PathNode::new(map.start, 0, h(&map.start, &map.end), TravelDirection::E);

    let mut open: Vec<PathNode> = vec![start_node];
    let mut closed: Vec<PathNode> = Vec::new();

    while !open.is_empty() {
        let index = open
            .iter()
            .enumerate()
            .min_by_key(|&(_, node)| node.f())
            .map(|(i, _)| i)
            .unwrap();
        let curr = open.swap_remove(index);
        closed.push(curr.clone());

        if curr.p == map.end {
            return Some(curr.f());
        }

        let children: Vec<PathNode> = map
            .grid
            .neighbors(&curr.p)
            .filter_map(|(p, &c)| if c == Cells::Empty { Some(p) } else { None })
            .filter(|&p| !closed.iter().any(|n| n.p == p))
            .map(|p| {
                let (g, dir) = g(&p, &curr);
                let h = h(&p, &map.end);
                PathNode::new(p, g, h, dir)
            })
            .filter(|n| {
                if let Some(existing) = open.iter().find(|&existing| existing.p == n.p) {
                    if existing.g < n.g {
                        return false;
                    }
                }
                true
            })
            .collect();

        for n in children {
            open.push(n.clone());
        }
    }

    eprintln!("Cannot travel to {} from {}", map.end, map.start);
    None
}

pub fn solve_day_16_part_01(input: &str) -> u32 {
    let map = ReindeerOlympicMap::from(input);
    a_star_pathfinding(map).unwrap()
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
    fn should_solve_day_16_part_01() {
        let input = read_string("./src/day16/input.txt").unwrap();

        let solution = solve_day_16_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_16_part_01_sample() {
        assert_eq!(7036, solve_day_16_part_01(TEST_INPUT.trim()));
    }

    #[test]
    fn should_parse() {
        let map = ReindeerOlympicMap::from(TEST_INPUT.trim());

        assert_eq!(Point::new(1, 13), map.start);
        assert_eq!(Point::new(13, 1), map.end);
    }

    #[test]
    fn should_solve_small_sample() {
        let input = "
#####
#S.E#
#####
"
        .trim();

        assert_eq!(2, solve_day_16_part_01(input));
    }

    #[test]
    fn should_solve_small_sample_with_corner() {
        let input = "
#####
#S..#
###E#
#####
"
        .trim();

        assert_eq!(1003, solve_day_16_part_01(input));
    }

    #[test]
    fn should_solve_small_sample_with_two_corners() {
        let input = "
#####
#S..#
###.#
#E..#
#####
"
        .trim();

        assert_eq!(2006, solve_day_16_part_01(input));
    }
}
