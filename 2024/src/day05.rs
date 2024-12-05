#![allow(unused)]

use crate::prelude::*;

#[derive(Debug)]
struct PrintingBits {
    // They're all 2 digit numbers - maybe we can use a u128 bit set
    before: HashMap<i32, Vec<i32>>,
    after: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

fn parse(input: &str) -> PrintingBits {
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').unwrap();
        let a = parse_or_fail(a);
        let b = parse_or_fail(b);

        before.entry(a).or_default().push(b);
        after.entry(b).or_default().push(a);
    }

    let updates: Vec<_> = lines
        .by_ref()
        .map(|line| -> Vec<i32> { line.split(",").map(parse_or_fail).collect_vec() })
        .collect_vec();

    PrintingBits {
        before,
        after,
        updates,
    }
}

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let bits = parse(input);

    let mut answer = 0;
    'updates_loop: for update in bits.updates {
        let middle = update[update.len() / 2];

        for (a, b) in update.into_iter().tuple_windows() {
            if let Some(before) = bits.before.get(&a) {
                if before.contains(&b) {
                    continue;
                }
            }
            continue 'updates_loop;
        }

        answer += middle;
    }

    answer
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut bits = parse(input);

    let mut answer = 0;
    'updates_loop: for update in &mut bits.updates {
        if update.is_sorted_by(|a, b| {
            if let Some(before) = bits.before.get(a) {
                before.contains(b)
            } else {
                false
            }
        }) {
            continue;
        }

        update.sort_by(|a, b| {
            use std::cmp::Ordering;
            if let Some(before) = bits.before.get(a) {
                if before.contains(b) {
                    return Ordering::Less;
                }
            }
            Ordering::Greater
        });

        let middle = update[update.len() / 2];
        answer += middle;
    }

    answer
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[rstest]
    #[case::given(143, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(123, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
