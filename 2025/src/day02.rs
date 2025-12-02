#![allow(unused)]

use crate::prelude::*;

fn is_invalid_id(id: i64) -> bool {
    let mut x = 10;
    for _ in 0.. {
        let Some(xx) = x.checked_mul(&x) else {
            break;
        };

        if (id > x * x / 10) {
            if id % x == id / x {
                return true;
            }
        }

        let Some(xx) = x.checked_mul(10) else {
            break;
        };

        x = xx;
    }

    false
}

// Part1 ========================================================================
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .i64s()
        .tuples()
        .flat_map(|(lo, hi)| lo..=(-hi))
        .filter(|&id| is_invalid_id(id))
        // .inspect(|id| println!("{id:?}"))
        .sum()
}

// Part2 ========================================================================
#[aoc(day2, part2)]
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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[rstest]
    #[case::given(1227775554, EXAMPLE_INPUT)]
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
