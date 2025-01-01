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

impl MulCache {
    /// Reset this cache to the default state (keeps the sum as is)
    fn reset(mut self) -> Self {
        self.mul_state = MulState::default();
        self.left.clear();
        self.right.clear();
        self
    }

    /// Advance to a new state.
    fn advance(mut self, state: MulState) -> Self {
        self.mul_state = state;
        self
    }

    /// Advance to a new state or reset based on whether a and b match
    fn advance_on_match_or_reset(self, a: char, b: char, advanced_state: MulState) -> Self {
        return if a == b {
            self.advance(advanced_state)
        } else {
            self.reset()
        };
    }
}

pub fn solve_day_03_part_01(input: impl Iterator<Item=char>) -> u32 {
    input.fold(MulCache::default(), |mut mul_cache, char|
        match mul_cache.mul_state {
            MulState::AwaitingM => mul_cache.advance_on_match_or_reset(char, 'm', MulState::M),
            MulState::M => mul_cache.advance_on_match_or_reset(char, 'u', MulState::U),
            MulState::U => mul_cache.advance_on_match_or_reset(char, 'l', MulState::L),
            MulState::L => mul_cache.advance_on_match_or_reset(char, '(', MulState::OpeningBracket),
            MulState::OpeningBracket =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::LeftNum;
                    mul_cache.left.push(char);
                    mul_cache
                } else {
                    mul_cache.reset()
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
                        _ => mul_cache.reset()
                    };
                },
            MulState::Comma =>
                return if char.is_numeric() {
                    mul_cache.mul_state = MulState::RightNum;
                    mul_cache.right.push(char);
                    mul_cache
                } else {
                    mul_cache.reset()
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
                        _ => mul_cache.reset()
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
}
