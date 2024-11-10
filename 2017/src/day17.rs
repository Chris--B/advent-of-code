#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    const LIMIT: usize = 2017;
    let steps: usize = input.trim().parse().unwrap();

    let mut buf = vec![0];

    let mut i = 0;
    for n in 1..=LIMIT {
        // if cfg!(debug_assertions) {
        //     println!("[{i:>2}] buf={buf:2?}");
        //     let b = vec![b'~'; 10 + 4 * i];
        //     let s = String::from_utf8_lossy(&b);
        //     println!("{s} ^");
        //     println!();
        // }

        i += steps;
        i %= buf.len();
        i += 1;

        buf.insert(i, n);
    }

    buf[i + 1]
}

// Part2 ========================================================================
#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    const LIMIT: usize = 50_000_000;
    let steps: usize = input.trim().parse().unwrap();

    let mut buf = vec![0];
    let mut fake_len = 1;

    let mut i = 0;
    for n in 1..=LIMIT {
        // if cfg!(debug_assertions) {
        //     println!("[{i:>2}] buf={buf:2?}");
        //     let b = vec![b'~'; 10 + 4 * i];
        //     let s = String::from_utf8_lossy(&b);
        //     println!("{s} ^");
        //     println!();
        // }

        i += steps;
        i %= fake_len;

        i += 1;
        // Only really insert in the positions we need. This keeps the first 2 elements of buf valid as if we did the full buffer!
        if i < 2 {
            buf.insert(i, n);
        }
        fake_len += 1;
    }

    buf[1]
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"3 ";

    #[rstest]
    #[case::given(638, EXAMPLE_INPUT)]
    #[trace]
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
    // I hope this is right?
    #[case::given(1_222_153, EXAMPLE_INPUT)]
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
