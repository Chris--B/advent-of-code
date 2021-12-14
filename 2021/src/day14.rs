use aoc_runner_derive::aoc;

use itertools::Itertools;

use std::collections::HashMap;

fn parse_input(input: &str) -> (String, HashMap<String, String>) {
    let template = input.lines().next().unwrap().to_owned();

    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut iter = line.split(" -> ");

            let from = iter.next().unwrap().to_owned();
            let to = iter.next().unwrap().to_owned();
            assert_eq!(iter.next(), None);

            (from, to)
        })
        .collect();

    (template, rules)
}

fn expand(rules: &HashMap<String, String>, s: &mut String) {
    let mut inserts: HashMap<usize, Vec<&str>> = HashMap::new();

    for ((i, a), (_, b)) in s.chars().enumerate().tuple_windows() {
        let mut pair = String::new();
        pair.push(a);
        pair.push(b);

        if let Some(x) = rules.get(&pair) {
            inserts.entry(i + 1).or_insert_with(Vec::new).push(x);
        }
    }

    let mut inserts: Vec<(usize, Vec<_>)> = inserts.drain().collect();

    // insert backwards so the index stays correct
    inserts.sort_by_key(|(idx, _)| usize::MAX - idx);
    for (idx, texts) in inserts {
        for text in texts {
            for c in text.chars() {
                s.insert(idx, c);
            }
        }
    }
}

// Part1 ======================================================================
#[aoc(day14, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u64 {
    let (mut template, rules) = parse_input(input);

    for _ in 0..10 {
        expand(&rules, &mut template);
    }

    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let most = counts.iter().map(|(_c, count)| *count).max().unwrap();
    let least = counts.iter().map(|(_c, count)| *count).min().unwrap();

    most - least
}

// Part2 ======================================================================
#[aoc(day14, part2)]
#[inline(never)]
pub fn part2(input: &str) -> u64 {
    let (mut template, rules) = parse_input(input);

    for t in 0..40 {
        dbg!(t);
        expand(&rules, &mut template);
    }

    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = counts.drain().collect();
    counts.sort_by_key(|(_c, count)| *count);

    counts.last().unwrap().1 - counts.first().unwrap().1
}

#[test]
fn check_example_1() {
    let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    let (mut template, rules) = parse_input(input);

    expand(&rules, &mut template);
    assert_eq!(template, "NCNBCHB");

    expand(&rules, &mut template);
    assert_eq!(template, "NBCCNBBBCBHCB");

    expand(&rules, &mut template);
    assert_eq!(template, "NBBBCNCCNBBNBNBBCHBHHBCHB");

    expand(&rules, &mut template);
    assert_eq!(
        template,
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
    );
}
