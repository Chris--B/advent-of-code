use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use strsim;

#[aoc(day2, part1)]
fn run1(input: &str) -> Result<u32, failure::Error> {
    let (twos, threes) = input
        .lines()
        .map(|id| checksum_id(&id))
        .fold((0, 0), |accum, p| {
            (accum.0 + p.0 as u32, accum.1 + p.1 as u32)
        });

    Ok(twos * threes)
}

#[aoc(day2, part2)]
fn run2(input: &str) -> Result<String, failure::Error> {
    // Create all pairs of all lines, and then filter them
    let ids = input.lines();
    let pairs: Vec<_> = ids
        .clone()
        .cartesian_product(ids)
        // The "<" here removes duplicate pairs
        // since (a, b) == (b, a), for our problem
        .filter(|p| p.0 != p.1 && p.0 < p.1)
        .filter(|p| strsim::hamming(p.0, p.1).unwrap() == 1)
        .collect();
    // We should now only have 1 pair.
    let pair = pairs.first().unwrap();
    assert_eq!(pairs.len(), 1);

    // Combine characters that appear in both ids.
    let result = String::from_utf8(
        pair.0
            .chars()
            .zip(pair.1.chars())
            .filter_map(|p| if p.0 == p.1 { Some(p.0 as u8) } else { None })
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    assert_eq!(pair.0.len(), result.len() + 1);

    Ok(result)
}

fn checksum_id(box_id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for c in box_id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let twos = counts.values().find(|count| **count == 2).is_some();
    let threes = counts.values().find(|count| **count == 3).is_some();
    (twos, threes)
}

#[test]
fn check_checksum_id() {
    assert_eq!(checksum_id("abcdef"), (false, false));
    assert_eq!(checksum_id("bababc"), (true, true));
    assert_eq!(checksum_id("abbcde"), (true, false));
    assert_eq!(checksum_id("abcccd"), (false, true));
    assert_eq!(checksum_id("aabcdd"), (true, false));
    assert_eq!(checksum_id("abcdee"), (true, false));
    assert_eq!(checksum_id("ababab"), (false, true));
}
