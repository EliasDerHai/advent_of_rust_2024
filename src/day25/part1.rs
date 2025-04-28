use std::{
    collections::HashSet,
    fmt::{write, Display},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct KeyLockSchema(String);

impl KeyLockSchema {
    fn invert(&self) -> Self {
        KeyLockSchema(
            self.0
                .chars()
                .map(|c| match c {
                    '#' => '.',
                    '.' => '#',
                    '\n' => '\n',
                    _ => panic!("expected '.' or '#' found '{c}'"),
                })
                .collect(),
        )
    }
}

impl Display for KeyLockSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct OrignalWithNegative(KeyLockSchema, KeyLockSchema);

pub fn solve_day_25_part_01(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|schema| KeyLockSchema(schema.to_string()))
        .fold((0u32, HashSet::new()), |(mut c, mut set), schema| {
            println!("o:\n{schema}");
            let negative = schema.invert();
            println!("n:\n{negative}");
            println!("set: {}", set.len());

            if set.contains(&schema) {
                c = c + 1;
            }
            set.insert(negative);

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

        println!("{solution}");
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
    fn should_match_negative() {
        let o = "
#####
##.##
.#.##
...##
...#.
...#.
.....
"
        .trim();

        let n = "
.....
.....
.....
#....
#.#..
#.#.#
#####"
            .trim();

        let n = KeyLockSchema(n.to_string());
        let o = KeyLockSchema(o.to_string()).invert();

        println!("{o}");
        println!("{n}");

        assert_eq!(n, o);
    }
}
