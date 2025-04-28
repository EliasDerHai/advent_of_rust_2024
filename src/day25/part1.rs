use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash)]
struct KeyLockSchema {
    raw: String,
    pins: [u8; 5],
    is_key: bool,
}

impl From<&str> for KeyLockSchema {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let reference: char = lines.next().unwrap().trim().chars().next().unwrap();
        let is_key = reference == '#';
        // 1 bc of first line and also 6 (7-1) to accredit for "line 0"
        let mut pins = if is_key { [1u8; 5] } else { [6u8; 5] };

        for line in lines {
            for (idx, c) in line.trim().chars().enumerate() {
                if c == reference {
                    if is_key {
                        pins[idx] += 1;
                    } else {
                        pins[idx] -= 1;
                    }
                }
            }
        }

        KeyLockSchema {
            raw: value.to_string(),
            pins,
            is_key,
        }
    }
}

impl KeyLockSchema {
    fn fits(&self, o: &KeyLockSchema) -> bool {
        if self.is_key == o.is_key {
            return false;
        }
        self.pins.iter().enumerate().all(|(idx, &p)| {
            let other_pin = o.pins.get(idx).unwrap();
            let together = p + other_pin;
            together <= 7u8
        })
    }
}

impl Display for KeyLockSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

pub fn solve_day_25_part_01(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(KeyLockSchema::from)
        .fold((0u32, HashSet::new()), |(mut c, mut set), schema| {
            c += set
                .iter()
                .filter(|&prev_schema| schema.fits(prev_schema))
                .count() as u32;
            set.insert(schema);
            (c, set)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_25_part_01() {
        let input = read_string("./src/day25/input.txt").unwrap();

        let solution = solve_day_25_part_01(&input);

        assert_eq!(3508, solution);
    }

    #[test]
    fn should_solve_day_25_part_01_sample() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
            .trim();

        assert_eq!(3, solve_day_25_part_01(input));
    }

    #[test]
    fn should_pins() {
        let s = KeyLockSchema::from(
            "
    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....",
        );

        assert_eq!([2, 3, 1, 6, 4], s.pins);
    }

    #[test]
    fn should_fit() {
        let s1 = KeyLockSchema::from(
            "
#####
#####
.#.##
...##
...#.
...#.
.....
",
        );

        let s2 = KeyLockSchema::from(
            "
.....
.....
#.#..
###..
###.#
###.#
#####
",
        );

        assert!(s1.fits(&s2));
    }

    #[test]
    fn shouldnt_fit() {
        let s1 = KeyLockSchema::from(
            "
#####
#####
.#.##
...##
...##
...#.
.....
",
        );

        let s2 = KeyLockSchema::from(
            "
.....
.....
#.#..
###..
###.#
###.#
#####
",
        );

        assert!(!s1.fits(&s2));
    }
}
