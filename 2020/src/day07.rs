use aoc_runner_derive::{aoc, aoc_generator};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap as Map;
use std::collections::HashSet as Set;

// Each day:
//  - Uncomment out part2's attribute macros

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag(u8, String);
impl Bag {
    fn kind(&self) -> &str {
        &self.1
    }
}
type Rules = Map<String, Vec<Bag>>;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Rules {
    // Gosh English, you're the worse.
    // let input = input.replace("bags", "bag");
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_rule)
        .collect()
}

fn parse_rule(rule: &str) -> (String, Vec<Bag>) {
    lazy_static! {
        /// Parse the list into bag and sub-bags
        static ref RE_1: Regex =
            Regex::new(r#"(?P<bag>[a-z ]+) bags contain (?P<subs>[^.]+)\."#)
                .expect("RE_1");

        /// Parse a sub bag list into count and type pairs
        static ref RE_2: Regex =
        Regex::new(r#"(\d+) ([^.,]+) bag"#).expect("RE_2");
    }
    let cap = RE_1.captures(rule).expect("no RE_1 captures");

    let bag = cap["bag"].to_string();
    let subs = &cap["subs"];

    let mut subs_vec = vec![];

    if subs != "no other bags" {
        for cap in RE_2.captures_iter(subs) {
            let count = cap[1].parse().expect("count didn't parse");
            let sub = cap[2].to_string();
            subs_vec.push(Bag(count, sub));
        }
        assert_ne!(subs_vec, vec![]);
    }

    (bag, subs_vec)
}

// Part1 ======================================================================
#[aoc(day7, part1)]
pub fn part1(input: &Rules) -> usize {
    let mut checked: Set<String> = Set::default();
    let mut unchecked: Vec<String> = input.keys().cloned().collect();
    let mut next_unchecked: Vec<String> = vec![];

    let mut count = 0;

    while !unchecked.is_empty() {
        for u in &unchecked {
            if !checked.contains(u) {
                checked.insert(u.clone());

                for bag in &input[u] {
                    if bag.kind() == "shiny gold" {
                        println!("{} contains a shiny gold bag", u);
                        count += 1;
                    }

                    next_unchecked.push(bag.kind().to_string());
                }
            }
        }

        std::mem::swap(&mut unchecked, &mut next_unchecked);
        next_unchecked.clear();
    }

    count
}

// Part2 ======================================================================
// #[aoc(day7, part2)]
pub fn _part2(_input: &Rules) -> usize {
    unimplemented!();
}

#[test]
fn check_rule_parser_1() {
    let rule = "bright white bags contain 1 shiny gold bag.";
    assert_eq!(
        parse_rule(rule),
        (
            "bright white".to_string(),
            vec![Bag(1, "shiny gold".to_string())]
        )
    );
}

#[test]
fn check_rule_parser_2() {
    let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    assert_eq!(
        parse_rule(rule),
        (
            "light red".to_string(),
            vec![
                Bag(1, "bright white".to_string()),
                Bag(2, "muted yellow".to_string())
            ]
        )
    );
}

#[test]
fn check_ex() {
    const INPUT: &str = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    let expected: Rules = vec![
        (
            "light red".to_string(),
            vec![
                Bag(1, "bright white".to_string()),
                Bag(2, "muted yellow".to_string()),
            ],
        ),
        (
            "dark orange".to_string(),
            vec![
                Bag(3, "bright white".to_string()),
                Bag(4, "muted yellow".to_string()),
            ],
        ),
        (
            "bright white".to_string(),
            vec![Bag(1, "shiny gold".to_string())],
        ),
        (
            "muted yellow".to_string(),
            vec![
                Bag(2, "shiny gold".to_string()),
                Bag(9, "faded blue".to_string()),
            ],
        ),
        (
            "shiny gold".to_string(),
            vec![
                Bag(1, "dark olive".to_string()),
                Bag(2, "vibrant plum".to_string()),
            ],
        ),
        (
            "dark olive".to_string(),
            vec![
                Bag(3, "faded blue".to_string()),
                Bag(4, "dotted black".to_string()),
            ],
        ),
        (
            "vibrant plum".to_string(),
            vec![
                Bag(5, "faded blue".to_string()),
                Bag(6, "dotted black".to_string()),
            ],
        ),
        ("faded blue".to_string(), vec![]),   // "no other"
        ("dotted black".to_string(), vec![]), // "no other"
    ]
    .into_iter() // We use a Vec instead of an array so that we can consume it here
    .collect();

    let rules = parse_input(INPUT);

    assert_eq!(rules, expected);
    assert_eq!(part1(&rules), 4);
}
