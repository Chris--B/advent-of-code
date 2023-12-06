#![allow(unused)]
use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let (time_str, dist_str) = input.trim().split_once('\n').unwrap();
    let time = time_str
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| -> i64 { s.parse().unwrap() });
    let dist = dist_str
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| -> i64 { s.parse().unwrap() });

    let mut r = 1;
    for (t, d) in time.zip(dist) {
        let mut ways = 0;
        for tt in 0..=t {
            if tt * (t - tt) > d {
                ways += 1;
            }
        }
        r *= ways;
    }

    r
}

// Part2 ========================================================================
#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let (time_str, dist_str) = input.trim().split_once('\n').unwrap();
    let time: String = time_str
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let t: i64 = time.parse().unwrap();
    let dist: String = dist_str
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let d: i64 = dist.parse().unwrap();

    let mut ways = 0;
    for tt in 0..=t {
        if tt * (t - tt) > d {
            ways += 1;
        }
    }

    ways
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_1: &str = r"
Time:      7
Distance:  9
";

    const EXAMPLE_INPUT_2: &str = r"
Time:      15
Distance:  40
";

    const EXAMPLE_INPUT_3: &str = r"
Time:      30
Distance:  200
";

    const EXAMPLE_INPUT: &str = r"
Time:      7  15   30
Distance:  9  40  200
";

    const MY_INPUT: &str = r"
Time:        59     79     65     75
Distance:   597   1234   1032   1328
";

    #[rstest]
    #[case::given(4, EXAMPLE_INPUT_1)]
    #[case::given(8, EXAMPLE_INPUT_2)]
    #[case::given(9, EXAMPLE_INPUT_3)]
    #[case::given(288, EXAMPLE_INPUT)]
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
    #[case::given(71503, EXAMPLE_INPUT)]
    #[case::mine(34454850, MY_INPUT)]
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
