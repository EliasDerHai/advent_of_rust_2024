use std::iter::repeat;

use crate::day09::part1::MemoryType::{Free, Occupied};

#[derive(Debug, PartialEq, Default, Clone)]
enum MemoryType {
    #[default]
    Free,
    Occupied { file_id: usize },
}

#[derive(Debug, PartialEq, Default, Clone)]
struct MemoryUnit {
    pos: usize,
    memory_type: MemoryType,
}

#[derive(Default, Debug, PartialEq)]
struct Disk {
    units: Vec<MemoryUnit>,
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self { // can be called from outside
        let mut disk = Disk::default();
        value
            .chars()
            .enumerate()
            .for_each(|(idx, c)| {
                let length = c.to_digit(10).unwrap() as usize;
                let is_file = idx % 2 == 0;
                let next_unit = if is_file {
                    MemoryUnit { pos: idx, memory_type: Occupied { file_id: idx / 2 } }
                } else {
                    MemoryUnit { pos: idx, memory_type: Free }
                };
                let mut next_chunk: Vec<MemoryUnit> = repeat(next_unit).take(length).collect();
                disk.units.append(&mut next_chunk);
            });

        disk
    }
}

impl Disk {
    fn fragment(&mut self) -> &mut Self {
        let mut left_index = 0;
        let mut right_index = self.units.len() - 1;

        while left_index < right_index {
            let left = self.units.get(left_index).unwrap();
            match left.memory_type {
                Free => {}
                Occupied { .. } => {
                    left_index += 1;
                    continue;
                }
            }

            let right = self.units.get(right_index).unwrap();
            match right.memory_type {
                Free => {
                    right_index -= 1;
                    continue;
                }
                _ => {}
            }

            self.units.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }

        self
    }

   fn checksum(&self) -> usize {
        self.units.iter().enumerate()
            .map(|(index, unit)| {
                match unit.memory_type {
                    Occupied { file_id } => file_id * index,
                    Free => 0,
                }
            })
            .sum()
    }
}


pub fn solve_day_09_part_01(input: String) -> usize {
    Disk::from(input.as_str())
        .fragment()
        .checksum()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_09_part_01() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_01(input);

        println!("{solution}");
        assert_eq!(6398608069280, solution);
    }

    #[test]
    fn should_solve_day_09_part_01_sample() {
        let input = "2333133121414131402".to_string();

        assert_eq!(1928, solve_day_09_part_01(input));
    }
}
