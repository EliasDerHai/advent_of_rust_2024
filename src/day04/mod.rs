use std::collections::HashMap;

use crate::day04::Directions::*;

/* represents read-direction (eg. NE means from left bottom to right top) */
#[derive(Debug)]
enum Directions {
    N,
    E,
    S,
    W,
    NW,
    SW,
    NE,
    SE,
}

/// takes a string and indexes every letter to a tuple (x, y)
/// where x is the index of the horizontal position in the line
/// and y is the index of the vertical position (~ line)
fn parse_to_map(input: String) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()//char_indices()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<(isize, isize), char>>()
}

fn explore<F>(entry_pos: (isize, isize), search: &str, strategy: F, lookup: &HashMap<(isize, isize), char>) -> u32
    where F: Fn((isize, isize)) -> (isize, isize)
{
    search.chars()
        .into_iter()
        .fold(Some(entry_pos), |pos, char| {
            if pos.is_none() { return None; }
            let next_pos = strategy(pos.unwrap());
            let next_letter = lookup.get(&next_pos);
            return match next_letter {
                Some(&letter) => {
                    return if letter == char {
                        Some(next_pos)
                    } else {
                        None
                    };
                }
                None => None
            };
        })
        .map(|_| 1)
        .unwrap_or(0)
}

pub fn solve_day_04_part_01(input: String) -> u32 {
    let letters = parse_to_map(input);
    let search_word = &"XMAS"[1..]; // X is the entry point so we essentially search for "MAS" from there on
    let directions = vec![
        N,
        E,
        S,
        W,
        NW,
        SW,
        NE,
        SE,
    ];

    letters
        .iter()
        .filter(|(_, &v)| v.to_ascii_uppercase() == 'X')
        .map(|(&(x, y), _)| {
            directions.iter().map(|dir| {
                let strategy: fn((isize, isize)) -> (isize, isize) = match dir {
                    N => |(x, y)| (x, y - 1),
                    E => |(x, y)| (x + 1, y),
                    S => |(x, y)| (x, y + 1),
                    W => |(x, y)| (x - 1, y),
                    NW => |(x, y)| (x - 1, y - 1),
                    SW => |(x, y)| (x - 1, y + 1),
                    NE => |(x, y)| (x + 1, y - 1),
                    SE => |(x, y)| (x + 1, y + 1)
                };
                explore((x, y), search_word, strategy, &letters)
            })
                .sum::<u32>()
        })
        .sum()
}


pub fn solve_day_04_part_02(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day04::solve_day_04_part_01;
    use crate::util::read_string;

    #[test]
    fn should_solve_day_04_part_01_sample_0() {
        let actual = solve_day_04_part_01("XMAS".to_string());
        assert_eq!(1, actual);
    }

    #[test]
    fn should_solve_day_04_part_01_sample_1() {
        let input = "
WWXWWW
WSAMXW
WAWWAW
XMASWS
WXWWWW".trim().to_string();

        let actual = solve_day_04_part_01(input);

        assert_eq!(4, actual);
    }

    #[test]
    fn should_solve_day_04_part_01_sample_2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();

        assert_eq!(18, solve_day_04_part_01(input));
    }

    #[test]
    fn should_solve_day_04_part_01_sample_3() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".trim().to_string();

        assert_eq!(true, 2 * 18 <= solve_day_04_part_01(input));
    }

    #[test]
    fn should_solve_day_04_part_01() {
        let input = read_string("./src/day04/input.txt")
            .expect("Should have")
            .trim()
            .to_string();


        let solution = solve_day_04_part_01(input);

        println!("Solution: {solution}");
    }

    #[test]
    fn should_solve_day_04_part_02() {
        todo!()
    }
}
