#![allow(unused)]

use crate::prelude::*;

fn report_is_safe(report: impl Iterator<Item = i32> + Clone) -> bool {
    let inc = report
        .clone()
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .all(|diff| [1, 2, 3].contains(&diff));

    let dec = report
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .all(|diff| [1, 2, 3].contains(&-diff));

    inc || dec
}

// Part1 ========================================================================
#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_whitespace().map(parse_or_fail))
        .map(report_is_safe)
        .filter(|&is_safe| is_safe)
        .count()
}

// Part2 ========================================================================
fn report_is_safe_ish(report: impl Iterator<Item = i32> + Clone) -> bool {
    // All safe reports are safe-ish too
    if report_is_safe(report.clone()) {
        return true;
    }

    // A report is safe-ish if it's safe with a single removal
    // Abuse iterators for fun and profit
    let len = report.clone().count();
    for i in 0..len {
        let modified_report = report
            .clone()
            .enumerate()
            // remove elem #`i`
            .filter_map(|(idx, level)| if idx != i { Some(level) } else { None });
        if report_is_safe(modified_report) {
            return true;
        }
    }

    // Still couldn't make it safe, so it's not even safe-ish
    false
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        // double iterator all the way
        .map(|l| l.split_whitespace().map(parse_or_fail))
        .map(report_is_safe_ish)
        .filter(|&is_safe| is_safe)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[rstest]
    #[case::given(2, EXAMPLE_INPUT)]
    #[case::given_line_1(1, "7 6 4 2 1")]
    #[case::given_line_2(0, "1 2 7 8 9")]
    #[case::given_line_3(0, "9 7 6 2 1")]
    #[case::given_line_4(0, "1 3 2 4 5")]
    #[case::given_line_5(0, "8 6 4 4 1")]
    #[case::given_line_6(1, "1 3 6 7 9")]
    #[case::small_triangle(0, "1 2 3 2 1")]
    #[case::large_triangle(0, "10 20 30 20 10")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    // #[case::given(4, EXAMPLE_INPUT)]
    #[case::given_line_1(1, "7 6 4 2 1")]
    #[case::given_line_2(0, "1 2 7 8 9")]
    #[case::given_line_3(0, "9 7 6 2 1")]
    #[case::given_line_4(1, "1 3 2 4 5")]
    #[case::given_line_5(1, "8 6 4 4 1")]
    #[case::given_line_6(1, "1 3 6 7 9")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
