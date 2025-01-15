use std::time::Instant;

pub fn solve_day_09_part_01(input: String) -> usize {
    let mut unzipped = String::new();

    let mut instant = Instant::now();
    input.chars().enumerate().for_each(|(idx, c)| {
        let length: usize = c.to_digit(10).unwrap() as usize;
        let is_file = idx % 2 == 0;
        let char_to_add = if is_file {
            let content = (idx / 2) % 10; // 0 - 9
            content.to_string().chars().next().unwrap()
        } else {
            '.'
        };
        let new: String = std::iter::repeat(char_to_add).take(length).collect();
        unzipped.push_str(&new);
    });

    println!("unzipped: {unzipped} - {}ms", instant.elapsed().as_millis());
    instant = Instant::now();

    let mut left_index = 0;
    let mut right_index = unzipped.len() - 1;
    let unsorted: Vec<char> = unzipped.chars().collect();
    while left_index < right_index {
        let left = unsorted[left_index];
        if left != '.' {
            left_index += 1;
            continue;
        }
        let right = unsorted[right_index];
        if right == '.' {
            right_index -= 1;
            continue;
        }

        let bytes = unsafe { unzipped.as_bytes_mut() };
        bytes.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;
    }

    println!("sorted: {unzipped} - {}ms", instant.elapsed().as_millis());

    unzipped
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(idx, c)| idx * (c.to_string().parse::<usize>().unwrap()))
        .sum()
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
        assert_eq!(5602033310, solution);
    }

    #[test]
    fn should_solve_day_09_part_01_sample() {
        let input = "2333133121414131402".to_string();

        assert_eq!(1928, solve_day_09_part_01(input));
    }
}
