use std::thread::sleep;
use std::time::Duration;

use rayon::prelude::*;

struct MagicStones {
    stones: Vec<usize>,
}

impl From<&str> for MagicStones {
    fn from(value: &str) -> Self {
        MagicStones {
            stones: value.split_whitespace().map(|n| n.parse::<usize>().expect("Nan")).collect()
        }
    }
}

impl MagicStones {
    fn evolve(mut self) -> Self {
        self.stones = self.stones
            .par_iter()
            .flat_map(|s| {
                match s {
                    0 => vec![1],
                    x  if x.to_string().len() % 2 == 0 => {
                        let x_str = x.to_string();
                        let split_idx = x_str.len() / 2;
                        let left = &x_str[..split_idx];
                        let right = &x_str[split_idx..];
                        vec![left, right]
                            .into_iter()
                            .map(|str_val| str_val.parse::<usize>().unwrap())
                            .collect()
                    }
                    _ => vec![*s * 2024]
                }
            })
            .collect();
        self
    }
}

pub fn solve_day_11_part_01(input: &str, generations: u8) -> Vec<usize> {
    let mut stones = MagicStones::from(input);
    for i in 0..generations {
        stones = stones.evolve();
        println!("... gen{i} done ...");
        sleep(Duration::from_millis(50));
    }
    stones.stones
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_11_part_01() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11_part_01(input.as_str(), 25).len();

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_11_part_02() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11_part_01(input.as_str(), 75).len();

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_11_part_01_sample() {
        let input = "125 17".trim();
        let expected_gen1 = vec![253000, 1, 7];
        let expected_gen2 = vec![253, 0, 2024, 14168];
        let expected_gen3 = vec![512072, 1, 20, 24, 28676032];

        assert_eq!(expected_gen1, solve_day_11_part_01(input, 1));
        assert_eq!(expected_gen2, solve_day_11_part_01(input, 2));
        assert_eq!(expected_gen3, solve_day_11_part_01(input, 3));
    }
}
