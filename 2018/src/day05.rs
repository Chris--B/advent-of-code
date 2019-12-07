
use std::{
    env,
    fs,
    io::{
        self,
        Read,
    },
    time,
};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day5, part1)]
fn run1(input: &str) -> Result<usize, failure::Error> {
    Ok(collapse(input.trim().chars()).len())
}

#[aoc(day5, part2)]
fn run2(input: &str) -> Result<usize, failure::Error> {
    Ok(optimize(input.trim()).len())
}

fn is_upper(c: char) -> bool {
    c == c.to_ascii_uppercase()
}

fn is_lower(c: char) -> bool {
    c == c.to_ascii_lowercase()
}

fn collapse(mut cur: impl Iterator<Item=char>) -> String {
    let mut res = String::new();
    res.push(cur.next().unwrap());
    loop {
        let mut dirty = false;
        loop {
            if let Some(next) = cur.next() {
                if res.is_empty() {
                    res.push(next);
                } else {
                    let a: char = res.chars().last().unwrap();
                    let b: char = next;
                    if (is_lower(a) && is_upper(b) && a.to_ascii_uppercase() == b) ||
                       (is_upper(a) && is_lower(b) && a.to_ascii_lowercase() == b)
                    {
                        res.pop();
                    } else {
                        dirty = true;
                        res.push(next)
                    }
                }
            } else {
                break;
            }
        }
        if !dirty {
            break;
        }
    }

    res
}

fn optimize(polymer: &str) -> String {
    // Save some work - collapse before further collapsing.
    let polymer = &collapse(polymer.chars());

    "abcdefghijklmnopqrstuvwxyz"
    .chars()
    .map(|unit| {
        let trial_polymer = polymer
            .chars()
            .filter(|c| *c != unit && *c != unit.to_ascii_uppercase());
        collapse(trial_polymer)
    })
    .min_by_key(|polymer| polymer.len()).unwrap()
}

#[test]
fn check() {
    assert_eq!(collapse("aA".chars()),     "");
    assert_eq!(collapse("abBA".chars()),   "");
    assert_eq!(collapse("abAB".chars()),   "abAB");
    assert_eq!(collapse("aabAAB".chars()), "aabAAB");
    assert_eq!(collapse("dabAcCaCBAcCcaDA".chars()), "dabCBAcaDA");
}
