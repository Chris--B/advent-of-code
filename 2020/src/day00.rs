#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(dayN, part1)]
pub fn part1(input: &[i64]) -> i64 {
    unimplemented!();
}

#[test]
fn check_part1_ex() {
    let input = r#"
    TODO
    "#;

    let parsed = parse_input(input);

    assert_eq!(0, part1(&parsed));
}

// Part1 ======================================================================
#[aoc(dayN, part2)]
pub fn part2(input: &[i64]) -> i64 {
    unimplemented!();
}

#[test]
fn check_part2_ex() {
    let input = r#"
    TODO
    "#;

    let parsed = parse_input(input);

    assert_eq!(0, part2(&parsed));
}
