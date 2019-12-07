use std::{
    collections::HashSet,
    fs,
    io,
    io::BufRead,
};

use aoc_runner_derive::{aoc, aoc_generator};
use failure;

#[aoc(day1, part1)]
fn run1(input: &str) -> Result<i32, failure::Error> {
    let mut freqs: Vec<i32> = vec![];
    for line in input.lines() {
        freqs.push(line.parse()?);
    }
    let freq_change: i32 = freqs.iter().sum();

    Ok(freq_change)
}

#[aoc(day1, part2)]
fn run2(input: &str) -> Result<i32, failure::Error> {
    let mut changes: Vec<i32> = vec![];
    for line in input.lines() {
        changes.push(line.parse()?);
    }

    let mut seen = HashSet::new();
    let mut freq = 0;
    seen.insert(freq);

    for change in changes.iter().cycle() {
        freq += change;

        if seen.contains(&freq) {
            break;
        }
        seen.insert(freq);
    }

    Ok(freq)
}
