use crate::day10::part1::{TopographicMap, Trail};

impl TopographicMap {
    fn traverse_ratings(&self) -> usize {
        let mut open: Vec<Trail> = self // only one word difference to part one 😁
            .starting_points()
            .into_iter()
            .map(|(&p, _)| Trail::new(p))
            .collect();

        while open.iter().any(|trail| !trail.is_completed()) {
            open = open
                .into_iter()
                .flat_map(|trail| self.travel_adjacent(trail))
                .collect();
        }

        open.len()
    }
}

pub fn solve_day_10_part_02(input: String) -> usize {
    TopographicMap::from(input.as_str()).traverse_ratings()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_10_part_02() {
        let input = read_string("./src/day10/input.txt").unwrap();

        let solution = solve_day_10_part_02(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_10_part_01_sample() {
        assert_eq!(81, solve_day_10_part_02("
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732".trim().to_string()));
    }
}
