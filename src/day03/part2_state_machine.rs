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


pub fn solve_day_03_part_02_state_machine(input: impl Iterator<Item=char>) -> u32 {
    input.fold(MulCache::default(), |mut mul_cache, char| {
        let next_mul_cache = match mul_cache.mul_state {
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
                        mul_cache.mul_state = MulState::ClosingBracket;
                        mul_cache.disabled = false;
                        mul_cache
                    }
                    _ => mul_cache.reset()
                }
            MulState::OpeningBracket(Word::Dont) =>
                match char {
                    ')' => {
                        mul_cache.mul_state = MulState::ClosingBracket;
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
        };
        // println!("{char} -> {:?}", next_mul_cache);
        return next_mul_cache;
    },
    ).sum
}

#[cfg(test)]
mod tests {
    use crate::util::read_chars;

    use super::*;

    #[test]
    fn should_solve_day_03_part_02_state_machine() {
        let input = read_chars("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_02_state_machine(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_03_part_02_examples() {
        assert_eq!(48, solve_day_03_part_02_state_machine("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".bytes().map(|b| b as char)));
        assert_eq!(193, solve_day_03_part_02_state_machine("mul()mul(1,3)don't()mul(2,5)dodomul(192,95)do()mul(20,5domul(190,1)".bytes().map(|b| b as char)));
        assert_eq!(868198, solve_day_03_part_02_state_machine("(who()where()''~[how()'&do()why()$;mul(323,598)&/-'}{&-/<do(), '~>[?-mul(933,97)how()?from();}{+mul(864,562):#<*$>mul(63,747)what()mul(514,101){".bytes().map(|b| b as char)));
        assert_eq!(17988259, solve_day_03_part_02_state_machine("(who()where()''~[how()'&do()why()$;mul(323,598)&/-'}{&-/<do(), '~>[?-mul(933,97)how()?from();}{+mul(864,562):#<*$>mul(63,747)what()mul(514,101){]$where())~>do(){:mul(53,731)mul(899,858)~~[select()(~mul(402,353)?^&!,who()what()-when()mul(4,41)-&mul(505,942)how()*/%select(667,826);mul(233,284)(&mul(484,956) #/mul(243,698)[;')how()'<%+[mul(153,970)!when()^{^;mul(176,383)@$$~[select(901,794)mul(322,492)from(183,121),-mul(212,356)who();)where()select()#do()>!who()!mul(138,847)&select()mul(128,454)select()what()(&<-mul(650,981) #when(636,522)(who()'-{?mul(149,431);/ !$}<#<!mul(806,218)when():mul(669,489)!@,) select()+mul(596,973)!@}mul(990,349)-]{,'mul(684,303)-[*mul(358,267)(mul(819,988)+;$}who()-[mul(67,603)< -!$%$who()?mul(753,49)[^who()>@mul(15,553)[[>;%mul(389,307)'mul(864,97)#[$why(),<>mul(322,599) ^mul(109,985)who()<from()from()?<?'mul(894,431)select(397,204)why()}mul(540,913)*?what()?~select()mul(411,407)/^how()-'select()mul(590,166) <how()mul(664,994)from()#^ *mul(384,184)][$^mul(113,201){$)*;{mul(634,407)who()@ how()why()(from()from(65,876)^mul(649,20)>when()<why()!~/who(),mul(586,611)mul(797,330)^&mul(409,692)@@from()]<select(){do()['who()who()&]mul(414,374)~from(){}what()>mul(763,870){from(905,178);$/mul(980,975)$why()@don't()#why()#from()from()when(){mul(436,3)[^]/*what()!!mul(300,322)where()!select()*(~why()mul(845,313),[:mul(937,973)how()^when()mul(552,183);from()/ &from()>'!mul(773,549)/where()~<?mul(869,838)%where()[%#&mul(835,525);?$<[], mul(69,159)what()>from(277,235)'mul(43,275)#>#[?-mul(110,899)~where()when())mul(976,743)%& mul(373,511)what(){{$)?}!mul(565,194)from()%#mul(47,349)(select()%when()&mul(263,392)-#,{+ *(mul(382,243)@:?+&-#%when()from()don't()where()what()(+from()- }where()mul(904,460)@how(768,761)}-why():<what()mul(312,552)@why()/-where()? >^mul(450,724)~mul(570,970)+-+/when():mul(203,363)why()[%%@mul(803,322)how()#who(),mul(368,46)select()what()#^*>from()from()+do()^(:)]+mul(585,478)^*:[mul(4,668);&select()'(mul(931,511);:mul(356,478)[mul(933,523)}';when()mul(616,605)why()$]{{select()how()~mul(869,274)mul(15,936)how()/:}mul(297,842)#}%~,select()$@(!mul(961,553)when()why(40,621)>mul(35,851),?+*mul(574,127)when()select()}who()from()/from()mul(107,604)<~@)mul(911,877who()?(when()!mul(496,778)select()when()select()&<mul(980,829)*<,>!$:,}+mul(278,194)%))who()when()*why()#+mul;when()??what()select() -%how()what()mul(873,684)where();where(741,393)+}?!'}mul(684,81)what()what()how()who()from()/<{>mul(286,481),<^mul(788,53)mul(561;)mul(376,887)+]-(when()[who();mul(205,369)!how(425,691)do(){}when() {mul(260,802)],*where()^why()how(){don't()where()}#?*), mul(670,405)^how()where()]mul(645,220)who();< > +select()how()+mul(763,932)+mul(141,632)+'}how(),@#from()'from()mul(211,161)mul(272,971)[@,>mul(340,784)-#why()&:@mul(343,209)when()#/]&mul(408,205)#!,,@;{>}@don't()mul(682,891)^:(>mul(234@when()} don't()) mul(322,500)~>when()*mul(826,896)$!^+->>what()don't()~&select()mul(127,828):,!^!%where(168,484)<~mul(709,801))@*{mul(148,549);from(649,310)+~what():why():?mul(62,198)mul(133,316)select()mul(580,623)who()who();<;[who(388,191)!{mul(819,559)/[mul(572,562)))@,mul(148,103)who(),select()(mul(669,636)-{mul(493,60)))&#^mul(766,322]*mul(290,960)mul(962,649)mul(209,105)".bytes().map(|b| b as char)));
        assert_eq!(27432119, solve_day_03_part_02_state_machine("mul(759,17why()&'!](%-mul(389,889),select(){/mul(907,972)why()&(<mul(778,477)-:from()*+@^mul(876,124)#}!what():what(){<mul(633,946)from()when()*]mul(61,166):*!select()select()mul(385,972))[*-<select()][do()/{select()what()mul(572,800)+%>#what()select()+!when()mul(379,398)'-{?mul(275,735)>why()(]%)select()+mul(811,330),%who()#mul(27,53)mul(810,378))how()[don't()^$'*mul(714,951)[>where()}:}%)mul(741,834~][<mul(515,311)[;)where()&-$when()!mul(943,268)>*% where()}&when()mul(84?mul(204,282)*,]+@^+%what()what()mul(442,73from()@select()}?when()~select()do()from()-}:where(){$* {mul(436,174)where(209,751)mul(576,189)[^when()what()why()don't()@when()%<$?mul(49,842)>who()from()-:$mul(309,375)-who() <+#+/when(225,777)mul(298,362)^~how()what()]&^;<mul(333,974)mul(760,603))<mul(547,974)#}}#}%mul(939,522)>&;^mul(573,219)[:/mul(714,428)}%mul(534who()@}}]:~'how()('mul(985,19)>( do()%what()&mul(17,24)when()),>from()do()@%!mul(967,50)why()}from()~who()<where()mul(619,399)-+mul(926,910)!mul(790,450)%what()mul(652,66))*~,)@%*'<do():#{/>/;!:mul(330,386)>why();]*<%[{from(285,989)mul(746,507),)where(131,831)'>select()!'who()mul(936,547)mul(698,306),mul(810,111)'[[when()mul(859,814when()(,^:$when()*^@*mul(639,6)()%mul(530,398)from()?'+]what()%mul(565~mul(664,297)][{where()$<'mul(521,918)<~'']?do();how()?[;,! who()^mul(993,640)-$;how():from()mul(555,779)where():?mul(802,307)/}&;@where(),],$mul(428,846)mul(914,333))%$?from()mul(132,678)*when(843,580)mul(360,826)*/when()% what();'mul(726,716)+select()'#~},when(587,50)select(871,597){mul(204,647)(from():{*-]$where()>mul(347,78)'@why()why(990,935)mul(708,962)(:^#what(),)+mul(946,615))>>$how()do()<<&mul(27,707){)&what()mul(734,74)+>'mul(889,356)+{why()why()&why()mul(875,327)[ $who();]#&!@mul(887,395)how()/<select(),~}'>mul(445,826)mul(325,137)why()$#&[+:mul(61,520)~when()]/from(682,457)^;&~,mul(909,277)mul(155,72)what()@<([?#do()&}why():who()mul(636,58)where())[<who()mul(963,11)?!what()what()} ~]mul(675,549)~%{)>@who(487,170)?mul(195,909)how()#;where()why()-)&[/mul(682,842)!(+}+^<<;mul(303,732)mul(709,34)@' (when()~mul(32,411)@don't()what()select()mul(765,345)mul(124,69)mul(68,95)/mul(967,749)do()who()where()who(){:why(430,451)(*when()where()mul(719,917)**)#mul(77,946),when()mul(273,971)#[what():+>why()]mul(583,407)<%from()mul(360,886)who())where(465,475)mul(398,130)#when()where()?where(),#~;mul(647,114*]who()-'*%!from()do()~';?(?$?mul(750,896)'^!how()+'!:/mul(95,159)who()>from(){;how()<mul(749,897)-'^mul(415,582)from()[(where()mul(568,507);~)why()) -mul(850,931)!where()why()~@<!{why()do() ^~%why()mul(909,10)!select(),!-}why()who()mul(654,569)'+mul(416,33)!+how() why()mul(384,73)(#why()where(){&mul(601,75)mul(186,890)-),mul(441,730)from()$+-mul(781,347)[{*from(),mul(412,933)how()]%$:><&,!mul(623,492)+?::from()who(){mul(915,36)#what()mul(855,275)mul(546,43)%%!&from()mul(256,937)^%how()where()+#mul(720,847)who()>mul(530,714))]~<mulwhen(127,51)#/#what()mul(442,992)mul(199,836)~(mul(133,954)#;@~ /:mul(807,810)-<!select()mul(967,241)'^mul(931,561)~;+$how(208,930)how()[(mul(536,241)$mul(161,306~!)$mul(823,497)[*:>what()#when()who()where(353,953)mul(948,731)} ?]~&don't())where()when()where()who():%,?-mul(348,179)why()where()^'/mul(858,210)^$what()!+mul(169,479)};from(106,826)/who()mul(44,593how(382,624)>)-'+#mul(822,120)when(509,411)]why()@from()@~/$do()mul(135,70)?select(){don't()]~mul(339,843)mul(89,314)*[-from()do()){>%^#mul(961,883)".bytes().map(|b| b as char)));
        assert_eq!(42730059, solve_day_03_part_02_state_machine("(who()where()''~[how()'&do()why()$;mul(323,598)&/-'}{&-/<do(), '~>[?-mul(933,97)how()?from();}{+mul(864,562):#<*$>mul(63,747)what()mul(514,101){]$where())~>do(){:mul(53,731)mul(899,858)~~[select()(~mul(402,353)?^&!,who()what()-when()mul(4,41)-&mul(505,942)how()*/%select(667,826);mul(233,284)(&mul(484,956) #/mul(243,698)[;')how()'<%+[mul(153,970)!when()^{^;mul(176,383)@$$~[select(901,794)mul(322,492)from(183,121),-mul(212,356)who();)where()select()#do()>!who()!mul(138,847)&select()mul(128,454)select()what()(&<-mul(650,981) #when(636,522)(who()'-{?mul(149,431);/ !$}<#<!mul(806,218)when():mul(669,489)!@,) select()+mul(596,973)!@}mul(990,349)-]{,'mul(684,303)-[*mul(358,267)(mul(819,988)+;$}who()-[mul(67,603)< -!$%$who()?mul(753,49)[^who()>@mul(15,553)[[>;%mul(389,307)'mul(864,97)#[$why(),<>mul(322,599) ^mul(109,985)who()<from()from()?<?'mul(894,431)select(397,204)why()}mul(540,913)*?what()?~select()mul(411,407)/^how()-'select()mul(590,166) <how()mul(664,994)from()#^ *mul(384,184)][$^mul(113,201){$)*;{mul(634,407)who()@ how()why()(from()from(65,876)^mul(649,20)>when()<why()!~/who(),mul(586,611)mul(797,330)^&mul(409,692)@@from()]<select(){do()['who()who()&]mul(414,374)~from(){}what()>mul(763,870){from(905,178);$/mul(980,975)$why()@don't()#why()#from()from()when(){mul(436,3)[^]/*what()!!mul(300,322)where()!select()*(~why()mul(845,313),[:mul(937,973)how()^when()mul(552,183);from()/ &from()>'!mul(773,549)/where()~<?mul(869,838)%where()[%#&mul(835,525);?$<[], mul(69,159)what()>from(277,235)'mul(43,275)#>#[?-mul(110,899)~where()when())mul(976,743)%& mul(373,511)what(){{$)?}!mul(565,194)from()%#mul(47,349)(select()%when()&mul(263,392)-#,{+ *(mul(382,243)@:?+&-#%when()from()don't()where()what()(+from()- }where()mul(904,460)@how(768,761)}-why():<what()mul(312,552)@why()/-where()? >^mul(450,724)~mul(570,970)+-+/when():mul(203,363)why()[%%@mul(803,322)how()#who(),mul(368,46)select()what()#^*>from()from()+do()^(:)]+mul(585,478)^*:[mul(4,668);&select()'(mul(931,511);:mul(356,478)[mul(933,523)}';when()mul(616,605)why()$]{{select()how()~mul(869,274)mul(15,936)how()/:}mul(297,842)#}%~,select()$@(!mul(961,553)when()why(40,621)>mul(35,851),?+*mul(574,127)when()select()}who()from()/from()mul(107,604)<~@)mul(911,877who()?(when()!mul(496,778)select()when()select()&<mul(980,829)*<,>!$:,}+mul(278,194)%))who()when()*why()#+mul;when()??what()select() -%how()what()mul(873,684)where();where(741,393)+}?!'}mul(684,81)what()what()how()who()from()/<{>mul(286,481),<^mul(788,53)mul(561;)mul(376,887)+]-(when()[who();mul(205,369)!how(425,691)do(){}when() {mul(260,802)],*where()^why()how(){don't()where()}#?*), mul(670,405)^how()where()]mul(645,220)who();< > +select()how()+mul(763,932)+mul(141,632)+'}how(),@#from()'from()mul(211,161)mul(272,971)[@,>mul(340,784)-#why()&:@mul(343,209)when()#/]&mul(408,205)#!,,@;{>}@don't()mul(682,891)^:(>mul(234@when()} don't()) mul(322,500)~>when()*mul(826,896)$!^+->>what()don't()~&select()mul(127,828):,!^!%where(168,484)<~mul(709,801))@*{mul(148,549);from(649,310)+~what():why():?mul(62,198)mul(133,316)select()mul(580,623)who()who();<;[who(388,191)!{mul(819,559)/[mul(572,562)))@,mul(148,103)who(),select()(mul(669,636)-{mul(493,60)))&#^mul(766,322]*mul(290,960)mul(962,649)mul(209,105)\
        mul(759,17why()&'!](%-mul(389,889),select(){/mul(907,972)why()&(<mul(778,477)-:from()*+@^mul(876,124)#}!what():what(){<mul(633,946)from()when()*]mul(61,166):*!select()select()mul(385,972))[*-<select()][do()/{select()what()mul(572,800)+%>#what()select()+!when()mul(379,398)'-{?mul(275,735)>why()(]%)select()+mul(811,330),%who()#mul(27,53)mul(810,378))how()[don't()^$'*mul(714,951)[>where()}:}%)mul(741,834~][<mul(515,311)[;)where()&-$when()!mul(943,268)>*% where()}&when()mul(84?mul(204,282)*,]+@^+%what()what()mul(442,73from()@select()}?when()~select()do()from()-}:where(){$* {mul(436,174)where(209,751)mul(576,189)[^when()what()why()don't()@when()%<$?mul(49,842)>who()from()-:$mul(309,375)-who() <+#+/when(225,777)mul(298,362)^~how()what()]&^;<mul(333,974)mul(760,603))<mul(547,974)#}}#}%mul(939,522)>&;^mul(573,219)[:/mul(714,428)}%mul(534who()@}}]:~'how()('mul(985,19)>( do()%what()&mul(17,24)when()),>from()do()@%!mul(967,50)why()}from()~who()<where()mul(619,399)-+mul(926,910)!mul(790,450)%what()mul(652,66))*~,)@%*'<do():#{/>/;!:mul(330,386)>why();]*<%[{from(285,989)mul(746,507),)where(131,831)'>select()!'who()mul(936,547)mul(698,306),mul(810,111)'[[when()mul(859,814when()(,^:$when()*^@*mul(639,6)()%mul(530,398)from()?'+]what()%mul(565~mul(664,297)][{where()$<'mul(521,918)<~'']?do();how()?[;,! who()^mul(993,640)-$;how():from()mul(555,779)where():?mul(802,307)/}&;@where(),],$mul(428,846)mul(914,333))%$?from()mul(132,678)*when(843,580)mul(360,826)*/when()% what();'mul(726,716)+select()'#~},when(587,50)select(871,597){mul(204,647)(from():{*-]$where()>mul(347,78)'@why()why(990,935)mul(708,962)(:^#what(),)+mul(946,615))>>$how()do()<<&mul(27,707){)&what()mul(734,74)+>'mul(889,356)+{why()why()&why()mul(875,327)[ $who();]#&!@mul(887,395)how()/<select(),~}'>mul(445,826)mul(325,137)why()$#&[+:mul(61,520)~when()]/from(682,457)^;&~,mul(909,277)mul(155,72)what()@<([?#do()&}why():who()mul(636,58)where())[<who()mul(963,11)?!what()what()} ~]mul(675,549)~%{)>@who(487,170)?mul(195,909)how()#;where()why()-)&[/mul(682,842)!(+}+^<<;mul(303,732)mul(709,34)@' (when()~mul(32,411)@don't()what()select()mul(765,345)mul(124,69)mul(68,95)/mul(967,749)do()who()where()who(){:why(430,451)(*when()where()mul(719,917)**)#mul(77,946),when()mul(273,971)#[what():+>why()]mul(583,407)<%from()mul(360,886)who())where(465,475)mul(398,130)#when()where()?where(),#~;mul(647,114*]who()-'*%!from()do()~';?(?$?mul(750,896)'^!how()+'!:/mul(95,159)who()>from(){;how()<mul(749,897)-'^mul(415,582)from()[(where()mul(568,507);~)why()) -mul(850,931)!where()why()~@<!{why()do() ^~%why()mul(909,10)!select(),!-}why()who()mul(654,569)'+mul(416,33)!+how() why()mul(384,73)(#why()where(){&mul(601,75)mul(186,890)-),mul(441,730)from()$+-mul(781,347)[{*from(),mul(412,933)how()]%$:><&,!mul(623,492)+?::from()who(){mul(915,36)#what()mul(855,275)mul(546,43)%%!&from()mul(256,937)^%how()where()+#mul(720,847)who()>mul(530,714))]~<mulwhen(127,51)#/#what()mul(442,992)mul(199,836)~(mul(133,954)#;@~ /:mul(807,810)-<!select()mul(967,241)'^mul(931,561)~;+$how(208,930)how()[(mul(536,241)$mul(161,306~!)$mul(823,497)[*:>what()#when()who()where(353,953)mul(948,731)} ?]~&don't())where()when()where()who():%,?-mul(348,179)why()where()^'/mul(858,210)^$what()!+mul(169,479)};from(106,826)/who()mul(44,593how(382,624)>)-'+#mul(822,120)when(509,411)]why()@from()@~/$do()mul(135,70)?select(){don't()]~mul(339,843)mul(89,314)*[-from()do()){>%^#mul(961,883)".bytes().map(|b| b as char)));
    }
}
