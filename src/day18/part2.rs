use super::part1::{a_star_pathfinding, PushdownAutomatonMap};

pub fn solve_day_18_part_02(input: &str, map_size: u8) -> &str {
    let mut lower_boundary = 0; // always points at solvable maze
    let mut upper_boundary = input.lines().count() - 1; // always points at unsolvable maze
    let next_index = |l, u| (l + u) / 2;

    while lower_boundary != upper_boundary - 1 {
        let n = next_index(lower_boundary, upper_boundary);
        let map = PushdownAutomatonMap::new(input, map_size, n as u16);
        match a_star_pathfinding(map) {
            Some(_) => lower_boundary = n,
            None => upper_boundary = n,
        }
    }

    input.lines().nth(upper_boundary - 1).expect("must exist")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_18_part_02() {
        let input = read_string("./src/day18/input.txt").unwrap();

        let solution = solve_day_18_part_02(&input, 71);

        assert_eq!("52,5", solution);
    }

    #[test]
    fn should_solve_day_18_part_02_sample() {
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
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .trim();

        assert_eq!("6,1", solve_day_18_part_02(input, 7));
    }
}
