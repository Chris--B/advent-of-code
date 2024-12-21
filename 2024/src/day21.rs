#![allow(unused)]

use crate::prelude::*;

fn code_to_num(code: &str) -> i64 {
    code.trim_start_matches('0')
        .trim_end_matches('A')
        .parse()
        .unwrap()
}

// Part1 ========================================================================
#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let codes: [&str; 5] = [
        lines.next().unwrap(),
        lines.next().unwrap(),
        lines.next().unwrap(),
        lines.next().unwrap(),
        lines.next().unwrap(),
    ];

    let numeric_codes: [i64; 5] = [
        code_to_num(codes[0]),
        code_to_num(codes[1]),
        code_to_num(codes[2]),
        code_to_num(codes[3]),
        code_to_num(codes[4]),
    ];
    0
}

// Part2 ========================================================================
#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
029A
980A
179A
456A
379A
";

    #[rstest]
    #[case::given(126384, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
