use aoc_runner_derive::{aoc, aoc_generator};

use smallvec;
use std::collections::HashMap;
use std::collections::HashSet;

type SmallString = smallvec::SmallVec<[u8; 3]>;

// .1 orbits .0
#[derive(Clone, Debug)]
pub struct OrbitPair(SmallString, SmallString);

impl OrbitPair {
    fn in_orbit(&self) -> &SmallString {
        &self.1
    }

    fn center(&self) -> &SmallString {
        &self.0
    }
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<OrbitPair> {
    input
        .trim()
        .split_whitespace()
        .map(|line| {
            let mut iter = line.trim().split(")");
            OrbitPair(
                iter.next().unwrap().bytes().collect(),
                iter.next().unwrap().bytes().collect(),
            )
        })
        .collect()
}

fn count_orbit_chain(
    all_orbits: &HashMap<&SmallString, &SmallString>,
    start: &SmallString,
) -> usize {
    let mut len = 0;

    let mut next = Some(&start);
    while let Some(n) = next {
        len += 1;
        next = all_orbits.get(n);
    }

    len - 1
}

#[aoc(day6, part1)]
pub fn p1_simple(input: &[OrbitPair]) -> usize {
    let mut all_orbits: HashMap<&SmallString, &SmallString> = HashMap::new();

    for pair in input {
        debug_assert!(!pair.0.spilled());
        debug_assert!(!pair.1.spilled());

        // Any object only directly orbits one other object, so use the `in_orbit` as the key
        let already_existed = all_orbits.insert(pair.in_orbit(), pair.center());
        assert!(already_existed.is_none());
    }

    let mut counts = HashMap::<&SmallString, usize>::new();

    for key in all_orbits.keys() {
        let count = count_orbit_chain(&all_orbits, key);
        counts.insert(key, count);
    }

    counts.values().sum()
}

#[cfg(test)]
#[test]
fn check_sample() {
    let input = r#"
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L"#;

    assert_eq!(p1_simple(&parse(input)), 42);
}
