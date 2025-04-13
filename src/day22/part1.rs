#![allow(unused_variables, dead_code)]

use std::ops::{Div, Mul};

const PRUNE_VAL: u64 = 16777216;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Secret(u64);

impl Secret {
    fn new(v: u64) -> Self {
        Secret(v)
    }
    fn mix(&mut self, other: u64) -> Self {
        self.0 ^= other;
        *self
    }
    fn prune(&mut self) -> Self {
        self.0 %= PRUNE_VAL;
        *self
    }
    fn v(&self) -> u64 {
        self.0
    }
    fn evolve(&mut self) -> Self {
        self.mix(*self * 64);
        self.prune();
        self.mix(*self / 32);
        self.prune();
        self.mix(*self * 2048);
        self.prune()
    }

    fn evolve_n_times(&mut self, mut n: u32) -> Self {
        while n > 0 {
            self.evolve();
            n -= 1;
        }
        *self
    }
}

impl Mul<u64> for Secret {
    type Output = u64;

    fn mul(self, rhs: u64) -> Self::Output {
        self.0 * rhs
    }
}
impl Div<u64> for Secret {
    type Output = u64;

    fn div(self, rhs: u64) -> Self::Output {
        self.0 / rhs
    }
}

pub fn solve_day_22_part_01(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .flat_map(|line| line.parse::<u64>())
        .map(|v| Secret::new(v).evolve_n_times(2000).v())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_22_part_01() {
        let input = read_string("./src/day22/input.txt").unwrap();

        let solution = solve_day_22_part_01(&input);

        assert_eq!(13022553808, solution);
    }

    #[test]
    fn should_solve_day_22_part_01_sample() {
        #[rustfmt::skip]
        let expectations = vec![
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];

        expectations
            .into_iter()
            .enumerate()
            .fold(Secret::new(123), |mut sec, (gen, exp)| {
                assert_eq!(exp, sec.evolve().v(), "Generation {} broken", gen + 1);
                sec
            });

        assert_eq!(5908254, Secret::new(123).evolve_n_times(10).v());

        assert_eq!(8685429, Secret::new(1).evolve_n_times(2000).v());
        assert_eq!(4700978, Secret::new(10).evolve_n_times(2000).v());
        assert_eq!(15273692, Secret::new(100).evolve_n_times(2000).v());
        assert_eq!(8667524, Secret::new(2024).evolve_n_times(2000).v());
    }

    #[test]
    fn test() {
        assert_eq!(Secret::new(42).mix(15).v(), 37);
        assert_eq!(Secret::new(16777216).prune().v(), 0);
        assert_eq!(Secret::new(10) / 3, 3);
    }
}
