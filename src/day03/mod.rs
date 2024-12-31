/* essentially only `mul(5,4)` should match - not `mul*(5,4)` or `mul( 5, 4)` etc. (no extra tokens except `mul(number,number)` )*/


/* State of the char sequence */
#[derive(Debug, Clone, PartialEq, Default)]
enum MulState {
    /* no mul sequence - awaiting mul sequence */
    #[default]
    AwaitingM,
    /* m found transition to u allowed */
    M,
    /* u found transition to l allowed */
    U,
    /* l found transition to ( allowed */
    L,
    /* ( found transition to digit allowed */
    OpeningBracket,
    /* left-digit found transition to digit or comma allowed */
    LeftNum,
    /* comma found transition to digit allowed */
    Comma,
    /* right-digit found transition to digit or ) allowed */
    RightNum,
    /* ) found - sequence is valid */
    ClosingBracket,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct MulCache {
    /* caching left digits */
    left: Vec<char>,
    /* caching right digits */
    right: Vec<char>,
    /* state of token sequence */
    mul_state: MulState,
    /* aggregated sum of all previously parsed multiplications */
    sum: u32,
}

fn reset(mut cache: MulCache) -> MulCache {
    cache.mul_state = MulState::AwaitingM;
    cache.left.clear();
    cache.right.clear();
    return cache;
}

pub fn solve_day_03_part_01(input: impl Iterator<Item=char>) -> u32 {
    input.fold(MulCache::default(), |mut mul_cache, char|
        match mul_cache.mul_state {
            MulState::AwaitingM =>
                match char {
                    'm' => {
                        mul_cache.mul_state = MulState::M;
                        mul_cache
                    }
                    _ => reset(mul_cache)
                }
            MulState::M =>
                match char {
                    'u' => {
                        mul_cache.mul_state = MulState::U;
                        mul_cache
                    }
                    _ => reset(mul_cache)
                },
            MulState::U =>
                match char {
                    'l' => {
                        mul_cache.mul_state = MulState::L;
                        mul_cache
                    }
                    _ => reset(mul_cache)
                },
            MulState::L =>
                match char {
                    '(' => {
                        mul_cache.mul_state = MulState::OpeningBracket;
                        mul_cache
                    }
                    _ => reset(mul_cache)
                },
            MulState::OpeningBracket =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::LeftNum;
                    mul_cache.left.push(char);
                    mul_cache
                } else {
                    reset(mul_cache)
                },
            MulState::LeftNum =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::LeftNum;
                    mul_cache.left.push(char);
                    mul_cache
                } else {
                    return match char {
                        ',' => {
                            mul_cache.mul_state = MulState::Comma;
                            mul_cache
                        }
                        _ => reset(mul_cache)
                    };
                },
            MulState::Comma =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::RightNum;
                    mul_cache.right.push(char);
                    mul_cache
                } else {
                    reset(mul_cache)
                },
            MulState::RightNum =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::RightNum;
                    mul_cache.right.push(char);
                    mul_cache
                } else {
                    return match char {
                        ')' => {
                            mul_cache.mul_state = MulState::ClosingBracket;
                            mul_cache
                        }
                        _ => reset(mul_cache)
                    };
                },
            MulState::ClosingBracket => {
                let left_num: u32 = mul_cache.left
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let right_num: u32 = mul_cache.right
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                mul_cache.sum += left_num * right_num;
                mul_cache.left.clear();
                mul_cache.right.clear();

                match char {
                    'm' => {
                        mul_cache.mul_state = MulState::M;
                        mul_cache
                    }
                    _ => { // no reset needed
                        mul_cache.mul_state = MulState::AwaitingM;
                        mul_cache
                    }
                }
            }
        },
    ).sum
}


pub fn solve_day_03_part_02(input: Vec<String>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::{read_chars, read_lines};

    use super::*;

    #[test]
    fn check_all_ascii() {
        read_lines("./src/day03/input.txt")
            .unwrap()
            .iter()
            .for_each(|c| if !c.is_ascii() {
                panic!("Contains other than ascii '{c}'");
            });
    }


    #[test]
    fn should_solve_day_03_part_01() {
        let input = read_chars("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_03_part_02() {
        let input = read_lines("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_02(input);

        println!("{solution}");
    }
}
