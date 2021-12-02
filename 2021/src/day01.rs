use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(day1, part1, simple_loop)]
#[inline(never)]
pub fn part1(input: &[u32]) -> u32 {
    let mut count = 0;

    for window in input.windows(2) {
        if window[0] < window[1] {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part1, tuple_windows)]
#[inline(never)]
pub fn part1_itertools(input: &[u32]) -> u32 {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count() as u32
}

// Part2 ======================================================================
#[aoc(day1, part2, simple_loop)]
#[inline(never)]
pub fn part2(input: &[u32]) -> u32 {
    let mut last_sum = u32::MAX;
    let mut count = 0;

    for window in input.windows(3) {
        let s = window.iter().sum();
        if last_sum < s {
            count += 1;
        }

        last_sum = s;
    }

    count
}

#[aoc(day1, part2, clever_loop)]
#[inline(never)]
pub fn part2_clever_loop(input: &[u32]) -> u32 {
    let mut count = 0;

    for window in input.windows(4) {
        if window[0] < window[3] {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2, tuple_windows)]
#[inline(never)]
pub fn part2_itertools(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_windows()
        .filter(|(a, _b, _c, d)| a < d)
        .count() as u32
}
