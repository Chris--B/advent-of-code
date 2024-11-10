#![allow(unused)]

use crate::prelude::*;

// Each day:
//  - Ctrl + F on dayN below
//  - Uncomment this module in lib.rs

// Part1 ========================================================================
#[aoc(dayN, part1)]
pub fn part1(input: &str) -> i64 {
    0
}

// Part2 ========================================================================
#[aoc(dayN, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
// todo
";

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
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
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    #[ignore]
    #[timeout(Duration::from_millis(1_500))]
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
