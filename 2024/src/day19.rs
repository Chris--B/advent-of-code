#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct State {
    possible: bool,
}

impl State {
    fn new(possible: bool) -> Self {
        #![allow(clippy::needless_update)]
        Self {
            possible,
            ..Self::default()
        }
    }
}

fn is_pattern_possible<'a>(
    pattern: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, State>,
) -> bool {
    if let Some(state) = cache.get_mut(pattern) {
        return state.possible;
    }

    let mut possible = false;
    for towel in towels {
        if let Some(next_pattern) = pattern.strip_prefix(towel) {
            // We found a match!
            possible |= is_pattern_possible(next_pattern, towels, cache);
            if possible {
                break;
            }
        }
    }
    cache.insert(pattern, State::new(possible));

    if cfg!(test) {
        println!(
            "({possible}) cache[{pattern}] == {:?}",
            cache.get(pattern).map(|s| s.possible)
        );
    }
    possible
}

// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(input: &str) -> i64 {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect_vec();
    let patterns: Vec<&str> = patterns.lines().collect_vec();

    if cfg!(test) {
        println!("Found {} towels: {towels:?}", towels.len());
        println!("Found {} patterns: {patterns:?}", patterns.len());
    }

    let mut cache: HashMap<&str, State> = HashMap::new();
    cache.insert("", State::new(true));
    for towel in &towels {
        cache.insert(towel, State::new(true));
    }

    for pattern in &patterns {
        if let Some(state) = cache.get_mut(pattern) {
            continue;
        }

        is_pattern_possible(pattern, &towels, &mut cache);
    }

    let mut possible_patterns: Vec<&&str> = patterns
        .iter()
        .filter(|&&p| {
            if cache.contains_key(p) {
                cache[p].possible
            } else {
                false
            }
        })
        .collect_vec();

    if cfg!(test) {
        possible_patterns.sort();

        println!("Possible patterns:");
        for p in &possible_patterns {
            println!("  + {p:?}");
        }

        println!("{} entries in cache", cache.len());
        let mut pairs = cache.iter().collect_vec();
        pairs.sort_by_key(|(&k, &v)| (k, v.possible));
        for (p, s) in pairs {
            println!("    + {:<5} <- {p:?}", s.possible);
        }
    }

    if !cfg!(test) {
        let good = patterns.iter().filter(|&&p| cache[p].possible).count();
        println!("Good patterns: {good}");

        let bad = patterns.iter().filter(|&&p| !cache[p].possible).count();
        println!("Bad patterns:  {bad}");

        assert!(
            possible_patterns.len() > 322,
            "{} <= 322, answer too low, UNDER counting",
            possible_patterns.len()
        );
    }

    possible_patterns.len() as i64
}

// Part2 ========================================================================
#[aoc(day19, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    fn ex(pattern: &str) -> String {
        let mut s = String::new();
        s.push_str(EXAMPLE_INPUT_TOWELS);
        s.push('\n');
        s.push('\n');
        s.push_str(pattern);
        s.push('\n');

        s
    }
    const EXAMPLE_INPUT_TOWELS: &str = "r, wr, b, g, bwu, rb, gb, br";

    #[rstest]
    // # Given Examples
    #[case::given(6, EXAMPLE_INPUT)]
    //
    // ## Possible
    //           "brwrr"  ==  "br", "wr", " r"
    #[case::given_brwrr_(1, &ex("brwrr"))]
    //           "bggr"   ==  "b", "g", "g", "r"
    #[case::given_bggr__(1, &ex("bggr"))]
    //           "gbbr"   ==  "gb", "br"
    #[case::given_gbbr__(1, &ex("gbbr"))]
    //           "rrbgbr" ==  "r", "rb", "g", "br"
    #[case::given_rrbgbr(1, &ex("rrbgbr"))]
    //           "bwurrg" ==  "bwu", "r", "r", "g"
    #[case::given_bwurrg(1, &ex("bwurrg"))]
    //           "brgr"   ==  "br", "g", "r"
    #[case::given_brgr__(1, &ex("brgr"))]
    //
    // ## Impossible
    //           "ubwu" is impossible.
    #[case::given_ubwu__(0, &ex("ubwu"))]
    //           "bbrgwb" is impossible.
    #[case::given_bbrgwb(0, &ex("bbrgwb"))]
    //
    // # bleh my sutff doesn't work Examples
    #[case::big_b(1, &ex("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"))]
    #[case::big_b_trailing_w(0, &ex("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbw"))]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
