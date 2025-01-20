use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

struct MagicStones {
    stones: Vec<usize>,
    stone_cache: Arc<Mutex<HashMap<(usize, u8), usize>>>, // just an experiment, but actually the
    // single threaded variant (w.o. mutex, arc & rayon) is twice as fast, which makes sense
    // considering that probably most of the time the cache is locked anyways...
}

impl From<&str> for MagicStones {
    fn from(value: &str) -> Self {
        MagicStones {
            stones: value.split_whitespace().map(|n| n.parse::<usize>().expect("Nan")).collect(),
            stone_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MagicStones {
    pub fn simulate(self, iterations: u8) -> usize {
        self.stones
            .clone()
            .into_par_iter()
            .map(|s| self.blink(s, iterations))
            .sum()
    }

    fn blink(&self, element: usize, iterations: u8) -> usize {
        if iterations == 0 {
            return 1;
        }

        let hash_key = (element, iterations);
        {
            match Arc::clone(&self.stone_cache).lock().unwrap().get(&hash_key) {
                None => {}
                Some(c) => { return *c; }
            }
        }


        let count = match element {
            0 => {
                self.blink(1, iterations - 1)
            }
            x  if x.to_string().len() % 2 == 0 => {
                let x_str = x.to_string();
                let split_idx = x_str.len() / 2;
                let left = x_str[..split_idx].parse::<usize>().unwrap();
                let right = x_str[split_idx..].parse::<usize>().unwrap();
                self.blink(left, iterations - 1) + self.blink(right, iterations - 1)
            }
            _ => self.blink(element * 2024, iterations - 1)
        };

        {
            Arc::clone(&self.stone_cache).lock().unwrap().insert(hash_key, count);
        }

        count
    }
}

pub fn solve_day_11(input: &str, generations: u8) -> usize {
    MagicStones::from(input).simulate(generations)
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_11() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11(input.as_str(), 25);

        println!("{solution}");
        assert_eq!(199982, solution);
    }

    #[test]
    fn should_solve_day_11_part_02() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11(input.as_str(), 75);

        assert_eq!(237149922829154, solution);
        println!("{solution}");
    }
}
