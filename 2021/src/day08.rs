use aoc_runner_derive::aoc;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SSDisplay(u8);

impl SSDisplay {
    fn set(self) -> u32 {
        self.0.count_ones()
    }
}

impl std::fmt::Debug for SSDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0b{:08b}", self.0)
    }
}

impl std::str::FromStr for SSDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #![allow(clippy::identity_op)]

        let mut letters = [0_u8; 7];
        for b in s.as_bytes() {
            debug_assert!(*b <= b'g');

            letters[(*b - b'a') as usize] = 1;
        }

        debug_assert!(letters.iter().all(|l| (*l == 0) || (*l == 1)));

        Ok(Self(
            0 | (letters[6] << 6)
                | (letters[5] << 5)
                | (letters[4] << 4)
                | (letters[3] << 3)
                | (letters[2] << 2)
                | (letters[1] << 1)
                | (letters[0] << 0),
        ))
    }
}

fn parse_line(line: &str) -> ([SSDisplay; 10], [SSDisplay; 4]) {
    let split: Vec<&str> = line.split(" | ").collect();
    let input: Vec<SSDisplay> = split[0].split(' ').map(|x| x.parse().unwrap()).collect();
    let output: Vec<SSDisplay> = split[1].split(' ').map(|x| x.parse().unwrap()).collect();

    (input.try_into().unwrap(), output.try_into().unwrap())
}

fn parse_input(input: &str) -> impl Iterator<Item = ([SSDisplay; 10], [SSDisplay; 4])> + '_ {
    input.lines().map(|line| parse_line(line.trim()))
}

// Part1 ======================================================================
#[aoc(day8, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|(_in, out)| {
            out.into_iter()
                .filter(|d| matches!(d.set(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>()
}

// Part2 ======================================================================
fn _make_known() -> [SSDisplay; 10] {
    [
        /* 0: 6 set */ "abcefg".parse().unwrap(),
        /* 1: 2 set */ "cf".parse().unwrap(),
        /* 2: 5 set */ "acdeg".parse().unwrap(),
        /* 3: 5 set */ "acdfg".parse().unwrap(),
        /* 4: 4 set */ "bcdf".parse().unwrap(),
        /* 5: 5 set */ "abdfg".parse().unwrap(),
        /* 6: 6 set */ "abdefg".parse().unwrap(),
        /* 7: 3 set */ "acf".parse().unwrap(),
        /* 8: 7 set */ "abcdefg".parse().unwrap(),
        /* 9: 6 set */ "abcdfg".parse().unwrap(),
    ]
}

#[aoc(day8, part2)]
#[inline(never)]
pub fn part2(input: &str) -> u64 {
    fn solve_one(pair: ([SSDisplay; 10], [SSDisplay; 4])) -> u64 {
        let (input, output) = pair;

        // These should all be known instantly
        let maybe_1: Vec<_> = input.iter().filter(|d| d.set() == 2).collect();
        let maybe_4: Vec<_> = input.iter().filter(|d| d.set() == 4).collect();
        let maybe_7: Vec<_> = input.iter().filter(|d| d.set() == 3).collect();
        let maybe_8: Vec<_> = input.iter().filter(|d| d.set() == 7).collect();

        // "cf"
        let def_1 = maybe_1[0];
        // "bcdf"
        let def_4 = maybe_4[0];
        // "acf"
        let def_7 = maybe_7[0];
        // "abcdefg"
        let def_8 = maybe_8[0];

        {
            debug_assert_eq!(maybe_1.len(), 1);
            drop(maybe_1);

            debug_assert_eq!(maybe_4.len(), 1);
            drop(maybe_4);

            debug_assert_eq!(maybe_7.len(), 1);
            drop(maybe_7);

            debug_assert_eq!(maybe_8.len(), 1);
            drop(maybe_8);
        }

        // "a"
        let just_a = def_1.0 ^ def_7.0;

        let maybe_2_3_5: Vec<_> = input.iter().filter(|d| d.set() == 5).collect();

        // X = "a cde g"
        // Y = "a cd fg"
        // Z = "ab d fg"
        // X ^ Y == "ef"
        // X ^ Z == "bcef" <-- only one with 4 set, so look for it
        // Y ^ Z == "bc"
        let guess_bcef: Vec<_> = [
            maybe_2_3_5[0].0 ^ maybe_2_3_5[1].0,
            maybe_2_3_5[1].0 ^ maybe_2_3_5[2].0,
            maybe_2_3_5[0].0 ^ maybe_2_3_5[2].0,
        ]
        .into_iter()
        .filter(|d| SSDisplay(*d).set() == 4)
        .collect();
        debug_assert_eq!(guess_bcef.len(), 1);

        let guess_bcef = guess_bcef[0];
        let guess_dg = !just_a & !guess_bcef & def_8.0;

        let just_d = guess_dg & def_4.0;
        let just_g = !just_d & guess_dg;

        let ish_bdeg = def_8.0 & !def_7.0;
        let ish_beg = ish_bdeg & !just_d;

        let just_b = ish_beg & def_4.0;
        let just_e = ish_beg & !just_b & !just_g;

        let maybe_0_6_9: Vec<_> = input.iter().filter(|d| d.set() == 6).collect();

        let just_cf = !(just_a | just_b | just_d | just_e | just_g);

        // "0" -> "abc efg" -> & "  c  f " -> "  c  f "
        // "6" -> "ab defg" -> & "  c  f " -> "     f " <- only 1, so find it
        // "9" -> "abcd fg" -> & "  c  f " -> "  c  f "
        let def_6: Vec<_> = maybe_0_6_9
            .iter()
            .filter(|d| SSDisplay(d.0 & just_cf).set() == 1)
            .collect();
        debug_assert_eq!(def_6.len(), 1);
        let def_6 = def_6[0];

        let def_1: SSDisplay = *def_1;
        let def_4: SSDisplay = *def_4;
        let def_6: SSDisplay = **def_6;
        let def_7: SSDisplay = *def_7;
        let def_8: SSDisplay = *def_8;

        let just_f = !just_a & def_6.0 & def_7.0;
        let just_c = just_cf & !just_f;

        let a = just_a;
        let b = just_b;
        let c = just_c;
        let d = just_d;
        let e = just_e;
        let f = just_f;
        let g = just_g;

        let def_0 = SSDisplay(!(1 << 7) & (a | b | c | e | f | g));
        let def_2 = SSDisplay(!(1 << 7) & (a | c | d | e | g));
        let def_3 = SSDisplay(!(1 << 7) & (a | c | d | f | g));
        let def_5 = SSDisplay(!(1 << 7) & (a | b | d | f | g));
        let def_9 = SSDisplay(!(1 << 7) & (a | b | c | d | f | g));

        // dbg!(
        //     SSDisplay(a),
        //     SSDisplay(b),
        //     SSDisplay(c),
        //     SSDisplay(d),
        //     SSDisplay(e),
        //     SSDisplay(f),
        //     SSDisplay(g),
        // );
        debug_assert_eq!(a & b & c & d & e & f & g, 0);

        let mut res: u64 = 0;

        for digit in output.into_iter() {
            res *= 10;

            if digit == def_0 {
                res += 0;
            } else if digit == def_1 {
                res += 1;
            } else if digit == def_2 {
                res += 2;
            } else if digit == def_3 {
                res += 3;
            } else if digit == def_4 {
                res += 4;
            } else if digit == def_5 {
                res += 5;
            } else if digit == def_6 {
                res += 6;
            } else if digit == def_7 {
                res += 7;
            } else if digit == def_8 {
                res += 8;
            } else if digit == def_9 {
                res += 9;
            } else {
                panic!("{:?}", digit);
            }
        }

        res
    }

    parse_input(input).map(solve_one).sum()
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

#[test]
fn check_example_1() {
    debug_assert_eq!(part1(EXAMPLE_INPUT), 26);
}

#[test]
fn check_example_2() {
    debug_assert_eq!(part2(EXAMPLE_INPUT), 61229);
}
