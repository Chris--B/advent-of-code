#![allow(clippy::needless_late_init)]

use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|e| e.trim().parse().unwrap())
        .collect()
}

#[test]
fn check_example_1() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part1(input), 37);
}

#[test]
fn check_example_2() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part2(input), 168);
}

// Part1 ======================================================================
#[aoc(day7, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let positions = parse_input(input);

    let mut cheapest = i64::MAX;

    for target in 0..(positions.len() as i64) {
        let mut cost = 0;

        for p in &positions {
            cost += (p - target).abs();
        }

        cheapest = cheapest.min(cost);
    }

    cheapest
}

// Part2 ======================================================================
#[aoc(day7, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    let positions = parse_input(input);

    let mut cheapest = i64::MAX;

    for target in 0..(positions.len() as i64) {
        let mut cost = 0;

        for p in &positions {
            let steps = (p - target).abs();
            cost += steps * (steps + 1) / 2;
        }

        cheapest = cheapest.min(cost);
    }

    cheapest
}

fn radix_sort(nums: &mut Vec<i64>) {
    let max = 1 + *nums.iter().max().unwrap_or(&0) as usize;

    let mut counts = vec![0; max];
    for n in nums.drain(..) {
        counts[n as usize] += 1;
    }

    for (num, count) in counts.iter().enumerate() {
        for _ in 0..*count {
            nums.push(num as i64);
        }
    }
}

#[aoc(day7, part1, formula)]
#[inline(never)]
pub fn part1_formula(input: &str) -> i64 {
    let mut positions = parse_input(input);
    radix_sort(&mut positions);

    let median: i64;
    if positions.len() % 2 == 0 {
        let med_idx_lo = positions.len() / 2;
        let med_idx_hi = positions.len() / 2;

        median = (positions[med_idx_lo] + positions[med_idx_hi]) / 2;
    } else {
        median = positions[positions.len() / 2];
    }

    positions.iter().map(|p| (p - median).abs()).sum()
}

#[aoc(day7, part2, formula)]
#[inline(never)]
pub fn part2_formula(input: &str) -> i64 {
    let positions = parse_input(input);

    let mean = positions.iter().sum::<i64>() / positions.len() as i64;

    positions
        .iter()
        .map(|p| {
            let n = (p - mean).abs();
            n * (n + 1) / 2
        })
        .sum()
}
