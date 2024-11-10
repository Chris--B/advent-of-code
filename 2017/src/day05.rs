#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let mut mem: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let mut ip = 0;

    // println!("Memory Length = {}", mem.len());
    for step in 0.. {
        // print!("[s={step:2>}, ip={ip:>2}] Memory: ");
        // print_with_focus(&mem[..20], ip);

        if ip >= mem.len() {
            // println!("Escaped @ step={step}");
            return step;
        }

        // Figure out where we're going
        let next_ip = ((ip as i64) + mem[ip]) as usize;

        // Increment before we leave
        mem[ip] += 1;

        // And jump
        ip = next_ip;
    }

    unreachable!()
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let mut mem: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let mut ip = 0;

    // println!("Memory Length = {}", mem.len());
    for step in 0.. {
        // print!("[s={step:2>}, ip={ip:>2}] Memory: ");
        // print_with_focus(mem.get(..20).unwrap_or(mem.as_slice()), ip);

        if ip >= mem.len() {
            // println!("Escaped @ step={step}");
            return step;
        }

        // Figure out where we're going
        let next_ip = ((ip as i64) + mem[ip]) as usize;

        // Inc-/Decrement before we leave
        if mem[ip] >= 3 {
            mem[ip] -= 1;
        } else {
            mem[ip] += 1;
        }

        // And jump
        ip = next_ip;
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"0 3 0 1 -3";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1))]
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
    #[case::given(10, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1))]
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
