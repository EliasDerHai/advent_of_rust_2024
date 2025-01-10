use std::collections::HashMap;

use crate::day04::Direction::*;

/* represents read-direction (eg. NE means from left bottom to right top) */
#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
    NW,
    SW,
    NE,
    SE,
}

const DIRECTIONS: [Direction; 8] = [
    N,
    E,
    S,
    W,
    NW,
    SW,
    NE,
    SE,
];


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

type NavigationStrategy = fn((isize, isize)) -> (isize, isize);

/// searches for a sequence in a lookup hashmap with a given strategy
/// returns true if sequence was found
/// return false if sequence was not found
fn search(
    lookup: &HashMap<(isize, isize), char>,
    search: &str,
    entry_pos: (isize, isize),
    strategy: NavigationStrategy,
) -> bool
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
        .is_some()
}

pub fn solve_day_04_part_01(input: String) -> u32 {
    let letters = parse_to_map(input);
    let search_word = &"XMAS"[1..]; // X is the entry point so we essentially search for "MAS" from there on
    letters
        .iter()
        .filter(|(_, &v)| v.to_ascii_uppercase() == 'X')
        .map(|(&(x, y), _)| {
            DIRECTIONS.iter().map(|dir| {
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
                return match search(&letters, search_word, (x, y), strategy) {
                    true => 1,
                    false => 0,
                };
            })
                .sum::<u32>()
        })
        .sum()
}


pub fn solve_day_04_part_02(input: String) -> u32 {
    let letters = parse_to_map(input);
    let search_word = "MAS";
    // a primary direction and it's two allowed counterparts that form an X
    // NW     NE
    //   \   /
    //    \ /
    //     X
    //    / \
    //   /   \
    // SW     SE
    let directions = [
        (NE, (NW, SE)),
        (SW, (NW, SE)),
        (SE, (NE, SW)),
        (NW, (NE, SW))
    ];

    let double: u32 = letters
        .iter()
        .filter(|(_, &v)| v.to_ascii_uppercase() == 'A')
        .map(|(&(x, y), _)| {
            directions.iter().map(|(primary_dir, (secondary_dir_option_1, secondary_dir_option_2))| {
                let first = search_dir(&letters, search_word, x, y, primary_dir);
                if !first { return 0; }
                let sec_1 = search_dir(&letters, search_word, x, y, secondary_dir_option_1);
                if sec_1 { return 1; }
                let sec_2 = search_dir(&letters, search_word, x, y, secondary_dir_option_2);
                if sec_2 { return 1; }
                0
            })
                .sum::<u32>()
        })
        .sum();
    double / 2
}

fn search_dir(letters: &HashMap<(isize, isize), char>, search_word: &str, x: isize, y: isize, dir: &Direction) -> bool {
    let (strategy, entry_pos): (NavigationStrategy, (isize, isize)) = match dir {
        NW => ((|(x, y)| (x - 1, y - 1)),
               (x + 2, y + 2)),
        SW => ((|(x, y)| (x - 1, y + 1)),
               (x + 2, y - 2)),
        NE => ((|(x, y)| (x + 1, y - 1)),
               (x - 2, y + 2)),
        SE => ((|(x, y)| (x + 1, y + 1)),
               (x - 2, y - 2)),
        _ => panic!("part_2 should only ever deal with diagonals")
    };
    search(&letters, search_word, entry_pos, strategy)
}

#[cfg(test)]
mod tests {
    use crate::day04::{solve_day_04_part_01, solve_day_04_part_02};
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
    fn should_solve_day_04_part_02_sample_1() {
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
MXMXAXMASX".trim().to_string();

        let actual = solve_day_04_part_02(input);

        assert_eq!(9, actual);
    }

    #[test]
    fn should_solve_day_04_part_02() {
        let input = read_string("./src/day04/input.txt")
            .expect("Should have")
            .trim()
            .to_string();

        let solution = solve_day_04_part_02(input);

        println!("{solution}");
    }
}
