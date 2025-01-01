/*
PROGRAM := ( COMMAND | GARBAGE )*

COMMAND := MUL_CMD | DO_CMD | DONT_CMD

MUL_CMD := "mul(" NUMBER "," NUMBER ")"
DO_CMD := "do()"
DONT_CMD := "don't()"

essentially only `mul(5,4)` should match - not `mul*(5,4)` or `mul( 5, 4)` etc. (no extra tokens except `mul(number,number)` )
*/

#[derive(Debug, Clone, Copy, PartialEq)]
enum Word {
    Mul,
    Do,
    Dont,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum MulState {
    #[default]
    /* nothing of value has been found - also initial state */
    Garbage,
    /* d found transition to o allowed */
    D,
    /* o found transition to n or ( allowed */
    O,
    /* n found transition to ' allowed */
    N,
    /* ' found transition to t allowed */
    Apostrophe,
    /* t found transition to ( allowed */
    T,
    /* m found transition to u allowed */
    M,
    /* u found transition to l allowed */
    U,
    /* l found transition to ( allowed */
    L,
    /* ( found transition to digit or ) allowed - need to state which word we're parsing */
    OpeningBracket(Word),
    /* left-digit found transition to digit or comma allowed */
    LeftNum,
    /* comma found transition to digit allowed */
    Comma,
    /* right-digit found transition to digit or ) allowed */
    RightNum,
    /* ) found - sequence is valid */
    ClosingBracket,
}

#[derive(Debug, PartialEq, Default)]
struct MulCache {
    /* caching left digits */
    left: Vec<char>,
    /* caching right digits */
    right: Vec<char>,
    /* state of token sequence */
    mul_state: MulState,
    /* disabled by 'don't()' sequence*/
    disabled: bool,
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

    /// Advance to a new state (if a and b match), transition into a word entry state or reset if garbage is found
    fn advance_on_match_or_reset(self, a: char, allowed: &[(char, MulState)]) -> Self {
        let allowed = allowed.iter().find(|(c, _)| *c == a);
        if allowed.is_some() {
            return self.advance(allowed.copied().unwrap().1);
        }
        match a {
            'm' => self.advance(MulState::M),
            'd' => self.advance(MulState::D),
            _ => self.reset()
        }
    }
}


pub fn solve_day_03_part_02(input: impl Iterator<Item=char>) -> u32 {
    input.fold(MulCache::default(), |mut mul_cache, char|
        match mul_cache.mul_state {
            MulState::Garbage => mul_cache.advance_on_match_or_reset(char, &[]),
            // do & dont
            MulState::D => mul_cache.advance_on_match_or_reset(char, &[('o', MulState::O)]),
            MulState::O => mul_cache.advance_on_match_or_reset(char, &[('n', MulState::N), ('(', MulState::OpeningBracket(Word::Do))]),
            MulState::N => mul_cache.advance_on_match_or_reset(char, &[('\'', MulState::Apostrophe)]),
            MulState::Apostrophe => mul_cache.advance_on_match_or_reset(char, &[('t', MulState::T)]),
            MulState::T => mul_cache.advance_on_match_or_reset(char, &[('(', MulState::OpeningBracket(Word::Dont))]),
            // mul
            MulState::M => mul_cache.advance_on_match_or_reset(char, &[('u', MulState::U)]),
            MulState::U => mul_cache.advance_on_match_or_reset(char, &[('l', MulState::L)]),
            MulState::L => mul_cache.advance_on_match_or_reset(char, &[('(', MulState::OpeningBracket(Word::Mul))]),
            MulState::OpeningBracket(Word::Do) =>
                match char {
                    ')' => {
                        mul_cache.disabled = false;
                        mul_cache
                    }
                    _ => mul_cache.reset()
                }
            MulState::OpeningBracket(Word::Dont) =>
                match char {
                    ')' => {
                        mul_cache.disabled = true;
                        mul_cache
                    }
                    _ => mul_cache.reset()
                }
            MulState::OpeningBracket(Word::Mul) =>
                if char.is_numeric() {
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
                } else if !mul_cache.disabled {
                    return match char {
                        ')' => {
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
                            mul_cache.mul_state = MulState::ClosingBracket;
                            mul_cache
                        }
                        _ => mul_cache.reset()
                    };
                } else {
                    mul_cache.reset()
                },
            MulState::ClosingBracket => mul_cache.advance_on_match_or_reset(char, &[]),
        },
    ).sum
}

#[cfg(test)]
mod tests {
    use crate::util::read_chars;

    use super::*;

    #[test]
    fn should_solve_day_03_part_02() {
        let input = read_chars("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_02(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_03_part_02_example() {
        assert_eq!(48, solve_day_03_part_02("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".bytes().map(|b| b as char)));
        assert_eq!(193, solve_day_03_part_02("mul()mul(1,3)don't()mul(2,5)dodomul(192,95)do()mul(20,5domul(190,1)".bytes().map(|b| b as char)));
        assert_eq!(868198, solve_day_03_part_02("(who()where()''~[how()'&do()why()$;mul(323,598)&/-'}{&-/<do(), '~>[?-mul(933,97)how()?from();}{+mul(864,562):#<*$>mul(63,747)what()mul(514,101){".bytes().map(|b| b as char)));
    }
}
