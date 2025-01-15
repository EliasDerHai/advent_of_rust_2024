use std::collections::HashSet;

use crate::day08::part1::parse_antennas;
use crate::util::algebra::{apply_vec, cartesian_product_refs, get_vector_between};

fn is_within_map(pos: (i32, i32), width: i32, height: i32) -> bool {
    let x = pos.0;
    let y = pos.1;
    x >= 0 && y >= 0 && x < width && y < height
}

/// traverses into a vec direction from a given start point and collects every visited point into a vec
/// (vec length matters)
fn collect_locations(from: (i32, i32), vec: (i32, i32), width: usize, height: usize) -> Vec<(i32, i32)> {
    let mut result = vec![from];
    let mut next = from;
    while is_within_map(next, width as i32, height as i32) {
        result.push(next);
        next = apply_vec(next, vec);
    }

    result
}

pub fn solve_day_08_part_02(input: String) -> usize {
    let width = input.lines().next().unwrap().trim().chars().count();
    let height = input.lines().count();
    let antenna_groups = parse_antennas(&input);
    let locations: HashSet<(i32, i32)> = antenna_groups
        .into_iter()
        .flat_map(|antennas| cartesian_product_refs(&antennas, &antennas)
            .filter(|(left, right)| left != right)
            .flat_map(|(&left, &right)| {
                let left_signed = (left.0 as i32, left.1 as i32);
                let right_signed = (right.0 as i32, right.1 as i32);
                let vec = get_vector_between(left_signed, right_signed);
                collect_locations(right_signed, vec, width, height)
            })
            .collect::<Vec<(i32, i32)>>()
        )
        .collect();

    locations.len()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_08_part_02() {
        let input = read_string("./src/day08/input.txt").unwrap();

        let solution = solve_day_08_part_02(input);
        println!("{solution}");
        assert_eq!(1030, solution);
    }

    #[test]
    fn should_solve_day_08_part_02_sample() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".trim().to_string();

        assert_eq!(34, solve_day_08_part_02(input));
    }
}
