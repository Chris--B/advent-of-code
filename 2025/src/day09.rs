#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = input.i64s().tuples().collect_vec();

    let mut area = 0;

    let n = red_tiles.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let a = red_tiles[i];
            let b = red_tiles[j];
            let this_area = (1 + (b.1 - a.1).abs()) * (1 + (b.0 - a.0).abs());
            area = area.max(this_area);
        }
    }

    area
}

// Part2 ========================================================================
#[aoc(day9, part2)]
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[rstest]
    #[case::given(50, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1))]
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
    #[case::given(24, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    #[timeout(Duration::from_millis(1))]
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
