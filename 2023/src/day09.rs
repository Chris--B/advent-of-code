use crate::prelude::*;

fn make_deltas(list: &[i64]) -> Vec<i64> {
    list.iter().tuple_windows().map(|(a, b)| a - b).collect()
}

fn calc_next_value(history: &[i64]) -> i64 {
    let mut derivates: Vec<Vec<i64>> = vec![history.into()];

    while !derivates[0].iter().all(|n| *n == 0) {
        let next = make_deltas(&derivates[0]);
        derivates.insert(0, next);
    }

    derivates.iter().map(|dxs| dxs[0]).sum()
}

// Part1 ========================================================================
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            calc_next_value(
                &l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .rev()
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

// Part2 ========================================================================
#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            calc_next_value(
                &l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[rstest]
    #[case::given(114, EXAMPLE_INPUT)]
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
    #[case::given(2, EXAMPLE_INPUT)]
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
