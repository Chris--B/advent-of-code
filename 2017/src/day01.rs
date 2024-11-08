#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();

    std::iter::zip(input.iter(), input.iter().cycle().skip(1))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as i64)
        .sum()
}

// Part2 ========================================================================
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let input = input.as_bytes();

    std::iter::zip(input.iter(), input.iter().cycle().skip(input.len() / 2))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as i64)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[rstest]
    #[case::given(3, "1122")]
    #[case::given(4, "1111")]
    #[case::given(0, "1234")]
    #[case::given(9, "91212129")]
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

    #[rstest]
    #[case::given(6, "1212")]
    #[case::given(0, "1221")]
    #[case::given(4, "123425")]
    #[case::given(12, "123123")]
    #[case::given(4, "12131415")]
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
