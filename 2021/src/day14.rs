use aoc_runner_derive::aoc;

use itertools::Itertools;

use std::collections::HashMap;

fn str_to_pair(s: &str) -> (char, char) {
    let mut iter = s.chars();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    (a, b)
}

fn parse_input(input: &str) -> (String, HashMap<(char, char), char>) {
    let template = input.lines().next().unwrap().to_owned();

    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();

            let from: (char, char) = str_to_pair(from);
            let to: char = to.chars().next().unwrap();

            (from, to)
        })
        .collect();

    (template, rules)
}

fn expand(rules: &HashMap<(char, char), char>, s: &mut String) {
    let mut inserts: HashMap<usize, Vec<char>> = HashMap::new();

    for ((i, a), (_, b)) in s.chars().enumerate().tuple_windows() {
        if let Some(x) = rules.get(&(a, b)) {
            inserts.entry(i + 1).or_default().push(*x);
        }
    }

    let mut inserts: Vec<(usize, Vec<_>)> = inserts.drain().collect();

    // insert backwards so the index stays correct
    inserts.sort_by_key(|(idx, _)| usize::MAX - idx);
    for (idx, text) in inserts {
        for c in text {
            s.insert(idx, c);
        }
    }
}

// Part1 ======================================================================
#[aoc(day14, part1, slow)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let (mut template, rules) = parse_input(input);

    for _ in 0..10 {
        expand(&rules, &mut template);
    }

    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();

    most - least
}

// Part2 ======================================================================
fn expand_fast(template: &str, rules: &HashMap<(char, char), char>, times: usize) -> usize {
    const fn idx1(a: char) -> usize {
        let a = a as u8 - b'A';
        a as usize
    }

    const fn idx2((a, b): (char, char)) -> usize {
        idx1(b) + 26 * idx1(a)
    }

    let mut pair_counts = [0_usize; 26 * 26];

    // initialize with the template
    for (a, b) in template.chars().tuple_windows() {
        pair_counts[idx2((a, b))] += 1;
    }

    for _ in 0..times {
        let mut next_pair_counts = [0_usize; 26 * 26];

        for ((a, b), x) in rules {
            // "remove" our pair
            let count = pair_counts[idx2((*a, *b))];
            if count > 0 {
                // "insert" its replacement pairs:
                next_pair_counts[idx2((*a, *x))] += count;
                next_pair_counts[idx2((*x, *b))] += count;
            }
        }

        std::mem::swap(&mut next_pair_counts, &mut pair_counts);
    }

    let mut letter_counts = [0_usize; 26];

    for a in 'A'..='Z' {
        for b in 'A'..='Z' {
            letter_counts[idx1(a)] += pair_counts[idx2((a, b))];
        }
    }

    // And the last letter in our template, which was ignored above
    {
        let c = template.chars().last().unwrap();
        letter_counts[idx1(c)] += 1;
    }

    let most = letter_counts
        .iter()
        .filter(|count| **count != 0)
        .max()
        .unwrap();
    let least = letter_counts
        .iter()
        .filter(|count| **count != 0)
        .min()
        .unwrap();

    most - least
}

#[aoc(day14, part1, fast)]
#[inline(never)]
pub fn part1_fast(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    expand_fast(&template, &rules, 10)
}

#[aoc(day14, part2, fast)]
#[inline(never)]
pub fn part2(input: &str) -> usize {
    let (template, rules) = parse_input(input);

    expand_fast(&template, &rules, 40)
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

#[test]
fn check_example_1_fast() {
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

    assert_eq!(part1_fast(input), 1588);
}

#[test]
fn check_example_2() {
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

    assert_eq!(part2(input), 2_188_189_693_529);
}
