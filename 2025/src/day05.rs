#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();
    let n = memmem::find(input, b"\n\n").expect("Couldn't find \\n\\n");
    let (ranges_text, ids_text) = input.split_at(n);

    let mut ranges: Vec<(i64, i64)> = ranges_text.i64s().tuples().map(|(a, b)| (a, -b)).collect();
    ranges.sort();
    let ranges = merge_ranges(ranges);

    let ids: Vec<i64> = ids_text.i64s().collect();

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
    let input = input.as_bytes();
    let n = memmem::find(input, b"\n\n").expect("Couldn't find \\n\\n");
    let input = &input[..n];

    let mut ranges: Vec<(i64, i64)> = input.i64s().tuples().map(|(a, b)| (a, -b)).collect();
    ranges.sort();

    merge_ranges(ranges)
        .into_iter()
        .map(|(a, b)| b - a + 1)
        .sum()
}

#[aoc(day5, part2, flat)]
pub fn part2_flat(input: &str) -> i64 {
    let input = input.as_bytes();
    let n = memmem::find(input, b"\n\n").expect("Couldn't find \\n\\n");
    let input = &input[..n];

    let mut ranges: Vec<_> = input.i64s().collect();
    ranges.sort_unstable_by_key(|x| (x.abs(), -x.signum()));

    let mut start = 0;
    let mut stack = 0;
    let mut sum = 0;
    for x in ranges {
        debug_assert!(x != 0);

        // Start a range if we have none
        if start == 0 {
            debug_assert!(x > 0);
            start = x;
        }

        // Start or end a range, depending on the sign value
        stack += x.signum();

        if stack == 0 {
            debug_assert!(x < 0);
            sum += -x - start + 1;
            start = 0;
        }
    }

    sum
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

1
";

    const SUPERSET_BOUNDS: &str = r"
1-100
10-20
20-30

1";

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
        #[values(part2, part2_flat)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
