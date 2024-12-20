use crate::day02::Direction::{Decreasing, Increasing, Undecided};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    Undecided,
}

pub fn solve_day_02_part_01(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let folded = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number"))
                .fold(Some((Undecided, 0)), |option, curr| {
                    if option.is_none() { return None; }
                    let dir = option.unwrap().0;
                    let last = option.unwrap().1;
                    println!("dir: {:?} - last: {} - curr: {}", dir, last, curr);
                    if last == 0 {
                        return Some((Undecided, curr));
                    }
                    if last == curr || curr < last - 3 || curr > last + 3 {
                        return None;
                    }
                    return match dir {
                        Undecided => Some((if last < curr { Increasing } else { Decreasing }, curr)),
                        Increasing => if last < curr { Some((dir, curr)) } else { None },
                        Decreasing => if last > curr { Some((dir, curr)) } else { None }
                    };
                });

            return match folded {
                None => 0,
                Some(_) => 1
            };
        })
        .sum()
}


pub fn solve_day_02_part_02(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let folded = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number"))
                /* direction, last value, bonus life */
                // TODO fix - nice try but there is an conceptual issue here
                //   bc the element might itself be fine but it's
                //   removal might still make the row safe
                //   e.g. 1 3 2 4 5 - 3 is safe but must still be removed!
                .fold(Some((Undecided, 0, true)), |option, curr| {
                    if option.is_none() { return None; }
                    let dir = option.unwrap().0;
                    let last = option.unwrap().1;
                    let bonus_life = option.unwrap().2;
                    println!("dir: {:?} - last: {} - curr: {}", dir, last, curr);
                    if last == 0 {
                        return Some((Undecided, curr, true));
                    }
                    if last == curr || curr < last - 3 || curr > last + 3 {
                        return if bonus_life {
                            Some((dir, last, false))
                        } else {
                            None
                        };
                    }
                    return match dir {
                        Undecided => Some((if last < curr { Increasing } else { Decreasing }, curr, bonus_life)),
                        Increasing => if last < curr { Some((dir, curr, bonus_life)) } else { None },
                        Decreasing => if last > curr { Some((dir, curr, bonus_life)) } else { None }
                    };
                });

            println!("{} is {}", line, if folded.is_some() { "safe" } else { "unsafe" });

            return match folded {
                None => 0,
                Some(_) => 1
            };
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::read_lines;

    use super::*;

    #[test]
    fn should_solve_day_02_part_01_to_unsafe() {
        // arrange
        let line = String::from("2 4 6 9 10 9");
        let mut lines = Vec::new();
        lines.push(line);

        println!("{:?}", lines);

        // act
        let actual = solve_day_02_part_01(lines);

        // assert
        assert_eq!(0, actual);
    }

    #[test]
    fn should_solve_day_02_part_01_to_safe() {
        // arrange
        let line = String::from("23 26 28 29 32");
        let mut lines = Vec::new();
        lines.push(line);

        println!("{:?}", lines);

        // act
        let actual = solve_day_02_part_01(lines);

        // assert
        assert_eq!(1, actual);
    }

    #[test]
    fn should_solve_day_02_part_01() {
        match read_lines("./src/day02/input.txt") {
            Ok(lines) => println!("Solution: {}", solve_day_02_part_01(lines)),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }

    #[test]
    fn should_solve_day_02_part_02_example() {
        let examples = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9"
        ];

        let actual = solve_day_02_part_02(examples.iter().map(|s| s.to_string()).collect());

        assert_eq!(4, actual);
    }

    #[test]
    fn should_solve_day_02_part_02() {
        match read_lines("./src/day02/input.txt") {
            Ok(lines) => println!("Solution: {}", solve_day_02_part_02(lines)),
            Err(e) => println!("Failed to parse: {}", e),
        }
    }
}

