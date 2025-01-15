use std::collections::{HashMap, HashSet};

use crate::util::algebra::{cartesian_product_refs, mirror};

fn parse_antennas(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphanumeric())
            .map(move |(x, c)| {
                (x, y, c)
            }))
        .fold(HashMap::<char, Vec<(usize, usize)>>::new(), |mut map, (x, y, c)| {
            map.entry(c).or_insert_with(Vec::new).push((x, y));
            map
        })
        .into_values()
        .collect()
}

pub fn solve_day_08_part_01(input: String) -> usize {
    let width = input.lines().next().unwrap().trim().chars().count();
    let height = input.lines().count();
    let antenna_groups = parse_antennas(&input);
    let locations: HashSet<(i32, i32)> = antenna_groups
        .into_iter()
        .flat_map(|antennas| cartesian_product_refs(&antennas, &antennas)
            .filter(|(left, right)| left != right)
            .map(|(&left, &right)| {
                let left_signed = (left.0 as i32, left.1 as i32);
                let right_signed = (right.0 as i32, right.1 as i32);
                mirror(left_signed, right_signed)
            })
            .filter(|&(x, y)|
                x >= 0 && y >= 0 && x < width as i32 && y < height as i32
            )
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
    fn should_solve_day_08_part_01() {
        let input = read_string("./src/day08/input.txt").unwrap();

        let solution = solve_day_08_part_01(input);
        assert_eq!(289, solution);
    }

    #[test]
    fn should_solve_day_08_part_01_sample() {
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

        assert_eq!(14, solve_day_08_part_01(input));
    }
}
