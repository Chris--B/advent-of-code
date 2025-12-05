#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let ranges_count = input.lines().take_while(|l| l.trim() != "").count();

    let mut ranges: Vec<(i64, i64)> = input
        .i64s()
        .tuples()
        .map(|(a, b)| (a, -b))
        .take(ranges_count)
        .collect();
    ranges.sort();

    let ids: Vec<i64> = input.i64s().skip(2 * ranges_count).collect();

    if cfg!(test) {
        println!("{:>8}={:?}", "ranges", ranges);
        println!("{:>8}={:?}", "ids", ids);
    }
    ids.into_iter()
        .filter(|id| {
            for (a, b) in &ranges {
                if (a..=b).contains(&id) {
                    return true;
                }
            }
            false
        })
        .count() as i64
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let ranges_count: usize = if cfg!(test) {
        input.lines().take_while(|l| l.trim() != "").count()
    } else {
        190
    };

    let mut ranges: Vec<_> = input
        .i64s()
        .tuples()
        .map(|(a, b)| (a, -b))
        .take(ranges_count)
        .collect();
    ranges.sort();

    merge_ranges(ranges)
        .into_iter()
        .map(|(a, b)| b - a + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    const EQUAL_BOUNDS: &str = r"
10-20
20-30

";

    const SUPERSET_BOUNDS: &str = r"
1-100
10-20
20-30

";

    #[rstest]
    #[case::given(3, EXAMPLE_INPUT)]
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
    #[case::given(14, EXAMPLE_INPUT)]
    #[case::equal_bounds(21, EQUAL_BOUNDS)]
    #[case::superset(100, SUPERSET_BOUNDS)]
    #[timeout(Duration::from_millis(10))]
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
