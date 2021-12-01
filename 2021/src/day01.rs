use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut count = 0;

    for window in input.windows(2) {
        if window[0] < window[1] {
            count += 1;
        }
    }

    count
}

// Part2 ======================================================================
#[aoc(day1, part2)]
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
