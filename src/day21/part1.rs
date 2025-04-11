use crate::util::grid::Grid;
use crate::util::point::Point;
use crate::util::stringify::stringify;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::LazyLock;

pub trait KeypadKey {
    fn get_pos(&self) -> Point;
}

trait Transpileable {
    fn transpile(&self) -> DirectionKeySequence;
}

/* DoorKey */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DoorKey {
    A,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

impl From<char> for DoorKey {
    fn from(value: char) -> Self {
        match value {
            'A' => DoorKey::A,
            '0' => DoorKey::K0,
            '1' => DoorKey::K1,
            '2' => DoorKey::K2,
            '3' => DoorKey::K3,
            '4' => DoorKey::K4,
            '5' => DoorKey::K5,
            '6' => DoorKey::K6,
            '7' => DoorKey::K7,
            '8' => DoorKey::K8,
            '9' => DoorKey::K9,
            _ => panic!("unexpected value '{value}'"),
        }
    }
}

impl KeypadKey for DoorKey {
    fn get_pos(&self) -> Point {
        match self {
            DoorKey::K7 => Point::new(0, 0),
            DoorKey::K8 => Point::new(1, 0),
            DoorKey::K9 => Point::new(2, 0),
            DoorKey::K4 => Point::new(0, 1),
            DoorKey::K5 => Point::new(1, 1),
            DoorKey::K6 => Point::new(2, 1),
            DoorKey::K1 => Point::new(0, 2),
            DoorKey::K2 => Point::new(1, 2),
            DoorKey::K3 => Point::new(2, 2),
            DoorKey::K0 => Point::new(1, 3),
            DoorKey::A => Point::new(2, 3),
        }
    }
}

/* DoorCode */

#[derive(Debug, PartialEq, Eq, Hash)]
// struct DoorCode([DoorKey; 4]);
struct DoorCode(Vec<DoorKey>);

impl From<&str> for DoorCode {
    fn from(value: &str) -> Self {
        DoorCode(
            value.chars().map(DoorKey::from).collect::<Vec<DoorKey>>(),
            //             .try_into()
            //             .expect("incorrect length"),
        )
    }
}

/* Pathfinding */

const NUMBER_KEYPAD: LazyLock<HashMap<(DoorKey, DoorKey), Vec<DirectionKey>>> =
    LazyLock::new(|| {
        let grid: Grid<char> = Grid::from(
            "
789
456
123
.0A"
            .trim(),
        );

        let grid = grid.filter_map(|c| match c {
            '.' => None,
            _ => Some(DoorKey::from(c)),
        });

        for (point, key) in grid.iter() {
            assert_eq!(*point, key.get_pos());
        }
        assert_eq!(grid.iter().count(), 11);

        get_best_paths(&grid)
    });

const DIRECTION_KEYPAD: LazyLock<HashMap<(DirectionKey, DirectionKey), Vec<DirectionKey>>> =
    LazyLock::new(|| {
        let grid: Grid<char> = Grid::from(
            "
.^A
<v>"
            .trim(),
        );

        let grid = grid.filter_map(|c| match c {
            '.' => None,
            _ => Some(DirectionKey::from(c)),
        });

        for (point, key) in grid.iter() {
            assert_eq!(*point, key.get_pos(), "{:?}", key);
        }
        assert_eq!(grid.iter().count(), 5);

        get_best_paths(&grid)
    });

/* Memoization */

impl<K: KeypadKey> Grid<K> {
    fn neighbors_with_direction_key(
        &self,
        p: &Point,
    ) -> impl Iterator<Item = (Point, &K, DirectionKey)> {
        [
            (p.left(), DirectionKey::Left),
            (p.down(), DirectionKey::Down),
            (p.up(), DirectionKey::Up),
            (p.right(), DirectionKey::Right),
        ]
        .into_iter()
        .map(|(n, key)| (n, self.get(&n), key))
        .filter_map(|(n, c, key)| c.map(|inner_c| (n, inner_c, key)))
    }
}

fn get_best_paths<T>(grid: &Grid<T>) -> HashMap<(T, T), Vec<DirectionKey>>
where
    T: KeypadKey + Clone + Eq + Hash,
{
    #[derive(Clone)]
    struct State {
        cost: u32,
        pos: Point,
        last: Option<DirectionKey>,
        path: Vec<DirectionKey>,
    }

    impl Eq for State {}

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn transition_cost(prev: Option<DirectionKey>, new: DirectionKey, traveled: usize) -> u32 {
        if Some(new) == prev {
            1
        } else {
            let dir = match new {
                DirectionKey::Left => 4,
                DirectionKey::Down => 3,
                DirectionKey::Up => 2,
                DirectionKey::Right => 1,
                DirectionKey::A => panic!("Invalid state"),
            };
            10 + dir * traveled as u32
        }
    }

    let mut best_paths = HashMap::new();

    for (start_point, start_key_ref) in grid.iter() {
        let start_key = start_key_ref.clone();
        let init_point = *start_point;
        let mut best_cost: HashMap<(Point, Option<DirectionKey>), u32> = HashMap::new();
        let mut best_at_point: HashMap<Point, (u32, Vec<DirectionKey>)> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let init_state = State {
            cost: 0,
            pos: init_point,
            last: None,
            path: Vec::new(),
        };
        best_cost.insert((init_point, None), 0);
        best_at_point.insert(init_point, (0, Vec::new()));
        heap.push(Reverse(init_state));

        while let Some(Reverse(state)) = heap.pop() {
            if let Some(&recorded) = best_cost.get(&(state.pos, state.last)) {
                if state.cost > recorded {
                    continue;
                }
            }
            // Record this state as the best for its position, if it beats any previous cost.
            best_at_point
                .entry(state.pos)
                .and_modify(|(cost, path)| {
                    if state.cost < *cost {
                        *cost = state.cost;
                        *path = state.path.clone();
                    }
                })
                .or_insert((state.cost, state.path.clone()));

            for (n_pos, _, direction) in grid.neighbors_with_direction_key(&state.pos) {
                let new_cost =
                    state.cost + transition_cost(state.last, direction, state.path.len());
                let mut new_path = state.path.clone();
                new_path.push(direction);
                let new_state = State {
                    cost: new_cost,
                    pos: n_pos,
                    last: Some(direction),
                    path: new_path,
                };
                let key = (n_pos, Some(direction));
                if best_cost
                    .get(&key)
                    .map_or(true, |&existing| new_cost < existing)
                {
                    best_cost.insert(key, new_cost);
                    heap.push(Reverse(new_state));
                }
            }
        }

        for (p, (_cost, path)) in best_at_point.iter() {
            if let Some(dest_key) = grid.get(p) {
                best_paths.insert((start_key.clone(), dest_key.clone()), path.clone());
            }
        }
    }
    best_paths
}

impl Transpileable for DoorCode {
    fn transpile(&self) -> DirectionKeySequence {
        let mut pos = DoorKey::A;
        let map = &NUMBER_KEYPAD;
        DirectionKeySequence(
            self.0
                .iter()
                .flat_map(|&key| {
                    let mut moves = map.get(&(pos, key)).unwrap().clone();
                    pos = key;
                    moves.push(DirectionKey::A);
                    moves
                })
                .collect(),
        )
    }
}

/* DirectionalKey */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl From<char> for DirectionKey {
    fn from(value: char) -> Self {
        match value {
            '^' => DirectionKey::Up,
            'A' => DirectionKey::A,
            '<' => DirectionKey::Left,
            'v' => DirectionKey::Down,
            '>' => DirectionKey::Right,
            _ => panic!("unexpected value '{value}'"),
        }
    }
}

impl KeypadKey for DirectionKey {
    fn get_pos(&self) -> Point {
        match self {
            DirectionKey::Up => Point::new(1, 0),
            DirectionKey::A => Point::new(2, 0),
            DirectionKey::Left => Point::new(0, 1),
            DirectionKey::Down => Point::new(1, 1),
            DirectionKey::Right => Point::new(2, 1),
        }
    }
}

impl Display for DirectionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionKey::Up => write!(f, "^"),
            DirectionKey::Down => write!(f, "v"),
            DirectionKey::Left => write!(f, "<"),
            DirectionKey::Right => write!(f, ">"),
            DirectionKey::A => write!(f, "A"),
        }
    }
}

/* DirectionKeySequence */

struct DirectionKeySequence(Vec<DirectionKey>);

impl From<&str> for DirectionKeySequence {
    fn from(value: &str) -> Self {
        DirectionKeySequence(
            value
                .chars()
                .map(DirectionKey::from)
                .collect::<Vec<DirectionKey>>(),
        )
    }
}

impl Transpileable for DirectionKeySequence {
    fn transpile(&self) -> DirectionKeySequence {
        let map = &DIRECTION_KEYPAD;
        let mut pos = DirectionKey::A;
        DirectionKeySequence(
            self.0
                .iter()
                .flat_map(|&key| {
                    let mut moves = map.get(&(pos, key)).unwrap().clone();
                    pos = key;
                    moves.push(DirectionKey::A);
                    moves
                })
                .collect(),
        )
    }
}

fn compile(line: &str, intermediate_robots: u8) -> DirectionKeySequence {
    let mut code: Box<dyn Transpileable> = Box::new(DoorCode::from(line));

    for _ in 0..intermediate_robots {
        code = Box::new(code.transpile());
    }

    code.transpile()
}

fn quick_compile(line: &str, intermediate_robots: u8) -> u32 {
    let code = DoorCode::from(line);

    let transpile_once = code.transpile();

    todo!()
}

pub fn solve_day_21(input: &str, intermediate_robots: u8) -> u32 {
    input
        .lines()
        .map(|line| {
            let numeric_part = &line[..3];
            let numeric_part = numeric_part
                .parse::<u32>()
                .expect(&format!("Couln't parse '{numeric_part}'"));

            let transpiliation_length = compile(line, intermediate_robots).0.len() as u32;

            let solution = numeric_part * transpiliation_length;
            println!("numeric_part = {numeric_part} + transpilation_length = {transpiliation_length} -> {solution}");
            solution
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::stringify::stringify;

    use crate::util::file::read_string;
    #[test]
    fn should_solve_part_1() {
        let input = read_string("./src/day21/input.txt").unwrap();

        let solution = solve_day_21(&input.trim(), 2);

        assert_eq!(169390, solution);
    }

    #[test]
    fn try_single_letter() {
        let solution = compile("0", 25).0.len();

        println!("{solution}");
        assert_eq!(0, solution);
    }

    #[test]
    fn should_solve_part_2() {
        let input = read_string("./src/day21/input.txt").unwrap();

        let solution = solve_day_21(&input.trim(), 17);

        println!("{solution}");
        assert_ne!(0, solution);
    }

    #[test]
    fn should_example() {
        let input = "
029A
980A
179A
456A
379A"
            .trim();

        let solution = solve_day_21(&input, 2);

        assert_eq!(126384, solution);
    }

    #[test]
    fn debug() {
        assert_eq!(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
            compile("029A", 2).0.len()
        );
        assert_eq!(
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len(),
            compile("980A", 2).0.len()
        );
        assert_eq!(
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            compile("179A", 2).0.len()
        );
        assert_eq!(
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len(),
            compile("456A", 2).0.len()
        );
        assert_eq!(
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            compile("379A", 2).0.len(),
        );
    }

    #[test]
    fn should_transpile_doorcode() {
        let code: DoorCode = "029A".into();
        let transpiled = code.transpile();

        assert_eq!("<A^A>^^AvvvA", stringify(transpiled.0));
    }

    #[test]
    fn should_transpile_directioncode() {
        let sequence: DirectionKeySequence = "<A^A>^^AvvvA".into();
        let transpiled = sequence.transpile();

        assert_eq!("v<<A>>^A<A>AvA<^AA>A<vAAA^>A", stringify(transpiled.0));
    }

    #[test]
    fn best_paths() {
        let map = &DIRECTION_KEYPAD;

        assert_eq!(
            ">>^",
            stringify(
                map.get(&(DirectionKey::Left, DirectionKey::A))
                    .unwrap()
                    .clone()
            )
        );
        assert_eq!(
            "<v",
            stringify(
                map.get(&(DirectionKey::A, DirectionKey::Down))
                    .unwrap()
                    .clone()
            )
        );
        assert_eq!(
            "^>",
            stringify(
                map.get(&(DirectionKey::Down, DirectionKey::A))
                    .unwrap()
                    .clone()
            )
        );

        let map = &NUMBER_KEYPAD;
        assert_eq!(
            "^^^",
            stringify(map.get(&(DoorKey::A, DoorKey::K9)).unwrap().clone())
        );
    }
}
