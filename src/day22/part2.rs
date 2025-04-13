use std::collections::HashMap;

use super::part1::Secret;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Window {
    data: [i8; 4],
    len: usize,
}

impl Window {
    fn new() -> Self {
        Self {
            data: [0; 4],
            len: 0,
        }
    }

    fn push(&mut self, value: i8) {
        if self.len < 4 {
            self.data[self.len] = value;
            self.len += 1;
        } else {
            self.data.copy_within(1.., 0);
            self.data[3] = value;
        }
    }

    fn as_array(&self) -> [i8; 4] {
        self.data
    }
}

impl Secret {
    fn digit(&self) -> i8 {
        let d = self.v() % 10;
        d.try_into().expect(&format!("{d} should fit in i8"))
    }
}

pub fn solve_day_22_part_02(input: &str) -> u32 {
    *input
        .trim()
        .lines()
        .flat_map(|line| line.parse::<u64>())
        .fold(
            HashMap::new(),
            |mut global_map: HashMap<[i8; 4], u32>, v| {
                let mut secret = Secret::new(v);
                let mut window = Window::new();
                let mut local_map: HashMap<[i8; 4], u8> = HashMap::new();

                for _ in 0..2000 {
                    let last_digit = secret.digit();
                    secret.evolve();
                    let new_digit = secret.digit();

                    window.push(last_digit - new_digit);

                    if window.len >= 4 && !local_map.contains_key(&window.as_array()) {
                        local_map.insert(window.as_array(), new_digit as u8);
                    }
                }

                local_map.into_iter().for_each(|(key, value)| {
                    let value = value as u32;
                    global_map
                        .entry(key)
                        .and_modify(|v| *v += value)
                        .or_insert(value);
                });

                global_map
            },
        )
        .values()
        .max()
        .unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_22_part_02() {
        let input = read_string("./src/day22/input.txt").unwrap();

        let solution = solve_day_22_part_02(&input);

        assert_eq!(1555, solution);
    }

    #[test]
    fn should_solve_day_22_part_02_sample() {
        let input = "1
2
3
2024"
            .trim();

        assert_eq!(23, solve_day_22_part_02(input));
    }

    #[test]
    fn should_fifo() {
        let mut window = Window::new();

        assert_eq!(window.len, 0);
        window.push(-2);
        window.push(-1);
        window.push(0);
        window.push(1);
        window.push(2);
        assert_eq!(window.as_array(), [-1, 0, 1, 2]);
        assert_eq!(window.len, 4);
    }
}
