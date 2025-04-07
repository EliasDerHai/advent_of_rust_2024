use super::part1::{a_star_pathfinding, PushdownAutomatonMap};

#[allow(unused_variables, dead_code)]

pub fn solve_day_18_part_02(input: &str, map_size: u8) -> &str {
    for n in 0.. {
        let map = PushdownAutomatonMap::new(input, map_size, n);
        if a_star_pathfinding(map).is_none() {
            return input
                .lines()
                .nth((n - 1) as usize)
                .expect("must exist")
                .trim();
        } else {
            println!("{n} ... still solveable");
        }
    }

    panic!("PushdownAutomatonMap never blocks for all bytes");
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
