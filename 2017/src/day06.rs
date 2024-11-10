#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut banks: Vec<i64> = input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let l = banks.len();

    let mut history: Vec<Vec<_>> = vec![];

    for step in 0.. {
        // Check if we've seen this state yet
        // TODO: This has terrible big-O
        if let Some(_i) = history.iter().position(|h| h == &banks) {
            break;
        } else {
            history.push(banks.clone());
        }

        // Decide which bank to redistribute
        // Note: max_by_key() breaks ties with high indices, the opposite of what we want, so we include the index to get the ordering we want.
        let redist_idx = (0..l).max_by_key(|&i| (banks[i], banks.len() - i)).unwrap();

        // Move the blocks out of the bank
        let mut redist = banks[redist_idx];
        banks[redist_idx] = 0;

        // Distribute each block, one at a time, to the rest of the banks
        for i in (0..l)
            .cycle()
            // We want to skip elements indexed 0..=redist_idx, so we don't redistribute to this bank, hence +1
            .skip(redist_idx + 1)
            .take(redist as usize)
        {
            banks[i] += 1;
            redist -= 1;
        }
    }

    history.len()
}

// Part2 ========================================================================
#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut banks: Vec<i64> = input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let l = banks.len();

    let mut history: Vec<Vec<_>> = vec![];
    let mut loop_start = 0;

    for step in 0.. {
        // Check if we've seen this state yet
        // TODO: This has terrible big-O
        if let Some(i) = history.iter().position(|h| h == &banks) {
            loop_start = i;
            break;
        } else {
            history.push(banks.clone());
        }

        // Decide which bank to redistribute
        // Note: max_by_key() breaks ties with high indices, the opposite of what we want, so we include the index to get the ordering we want.
        let redist_idx = (0..l).max_by_key(|&i| (banks[i], banks.len() - i)).unwrap();

        // Move the blocks out of the bank
        let mut redist = banks[redist_idx];
        banks[redist_idx] = 0;

        // Distribute each block, one at a time, to the rest of the banks
        for i in (0..l)
            .cycle()
            // We want to skip elements indexed 0..=redist_idx, so we don't redistribute to this bank, hence +1
            .skip(redist_idx + 1)
            .take(redist as usize)
        {
            banks[i] += 1;
            redist -= 1;
        }
    }

    history.len() - loop_start
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
0 2 7 0
";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(4, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
