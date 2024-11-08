#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    0
}

// Part2 ========================================================================
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[rstest]
    #[case::given(3, "1122")]
    // #[case::given(4, "1111")]
    // #[case::given(0, "1234")]
    // #[case::given(9, "91212129")]
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
    #[case::given(-1, "1122")]
    #[case::given(-1, "1111")]
    #[case::given(-1, "1234")]
    #[case::given(-1, "91212129")]
    #[trace]
    #[ignore]
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
