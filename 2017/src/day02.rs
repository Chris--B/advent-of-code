use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| -> i64 { s.parse().unwrap() })
                .minmax()
                .into_option()
                .unwrap()
        })
        .map(|(a, b)| b - a)
        .sum()
}

// Part2 ========================================================================
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| -> i64 {
            let mut ns = line
                .split_whitespace()
                .map(|s| -> i64 { s.parse().unwrap() })
                .collect_vec();
            ns.sort();

            for i in 0..ns.len() {
                for j in (i + 1)..ns.len() {
                    if ns[j] % ns[i] == 0 {
                        return ns[j] / ns[i];
                    }
                }
            }

            unreachable!("Couldn't find evenly divisible pair in ns={ns:?}");
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_1: &str = r"
5 1 9 5
7 5 3
2 4 6 8
";

    #[rstest]
    #[case::given(18, EXAMPLE_INPUT_1.trim())]
    #[case::given_row_1(9-1, "5 1 9 5")]
    #[case::given_row_2(7-3, "7 5 3")]
    #[case::given_row_3(8-2, "2 4 6 8")]
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

    const EXAMPLE_INPUT_2: &str = r"
5 9 2 8
9 4 7 3
3 8 6 5
";

    #[rstest]
    #[case::given(9, EXAMPLE_INPUT_2.trim())]
    #[case::given_row_1(4, "5 9 2 8")]
    #[case::given_row_2(3, "9 4 7 3")]
    #[case::given_row_3(2, "3 8 6 5")]
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
