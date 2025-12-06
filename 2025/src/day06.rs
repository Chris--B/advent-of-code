#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let mut lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .collect_vec();

    let ops: Vec<&str> = lines.pop().unwrap();
    let rows: Vec<Vec<i64>> = lines
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;

    for i in 0..ops.len() {
        let op = ops[i];
        let mut partial = if op == "*" { 1 } else { 0 };
        for j in 0..rows.len() {
            if op == "*" {
                partial *= rows[j][i];
            } else {
                partial += rows[j][i];
            }
        }
        sum += partial;
    }

    sum
}

// Part2 ========================================================================
#[aoc(day6, part2)]
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
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[rstest]
    #[case::given(4277556, EXAMPLE_INPUT)]
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
    #[case::given(3263827, EXAMPLE_INPUT)]
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
