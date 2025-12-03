#![allow(unused)]

use crate::prelude::*;

fn joltage(mut bat: &[u8], mut count: usize) -> i64 {
    let mut ans: i64 = 0;

    while count > 0 {
        let n = bat.len() - count + 1;
        let i = first_max(&bat[..n]).unwrap();

        ans = 10 * ans + (bat[i] - b'0') as i64;
        bat = &bat[(i + 1)..];
        count -= 1;
    }

    ans
}

// Part1 ========================================================================
#[aoc(day3, part1)]
pub fn part1(input: &str) -> i64 {
    input.lines().map(|l| joltage(l.as_bytes(), 2)).sum()
}

#[aoc(day3, part1, memchr)]
pub fn part1_memchr(input: &str) -> i64 {
    let mut input = input.as_bytes();

    let mut sum = 0;
    while let Some(pos) = memchr(b'\n', input) {
        let line = &input[..pos];
        sum += joltage(line, 2);
        input = &input[pos + 1..];
    }
    sum += joltage(input, 2);

    sum
}

// Part2 ========================================================================
#[aoc(day3, part2)]
pub fn part2(input: &str) -> i64 {
    input.lines().map(|l| joltage(l.as_bytes(), 12)).sum()
}

#[aoc(day3, part2, memchr)]
pub fn part2_memchr(input: &str) -> i64 {
    let mut input = input.as_bytes();

    let mut sum = 0;
    while let Some(pos) = memchr(b'\n', input) {
        let line = &input[..pos];
        sum += joltage(line, 12);
        input = &input[pos + 1..];
    }
    sum += joltage(input, 12);

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[rstest]
    #[case::given(357, EXAMPLE_INPUT)]
    #[case::given_1(98, "987654321111111")]
    #[case::given_2(89, "811111111111119")]
    #[case::given_3(78, "234234234234278")]
    #[case::given_4(92, "818181911112111")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_memchr)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(3121910778619, EXAMPLE_INPUT)]
    #[case::given_1(987654321111, "987654321111111")]
    #[case::given_2(811111111119, "811111111111119")]
    #[case::given_3(434234234278, "234234234234278")]
    #[case::given_4(888911112111, "818181911112111")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_memchr)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
