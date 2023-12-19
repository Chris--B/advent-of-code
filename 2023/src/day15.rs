use crate::prelude::*;

fn do_hash(ascii: &'_ [u8]) -> u8 {
    let mut cv = Wrapping(0_u8);

    for b in ascii {
        cv += *b;
        cv *= 17;
    }

    cv.0
}

// Part1 ========================================================================
#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|b| *b == b',')
        .map(|line| do_hash(line) as i64)
        .sum()
}

// Part2 ========================================================================
#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    #![allow(unused)]

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[test]
    fn check_hash_algo() {
        assert_eq!(do_hash(b"HASH"), 52_u8);
    }

    const EXAMPLE_INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[rstest]
    #[case::given(1320, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[ignore]
    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
