use crate::prelude::*;

type PrefixCache<'a> = HashMap<&'a str, State<'a>>;
type CountCache<'a> = HashMap<&'a str, i64>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct State<'a> {
    // Whether this state is possible at all
    possible: bool,

    // Which towels can be used as a prefix to make this state.
    // We will take each prefix, peel it off, and use the rest of the pattern to build a final list.
    // This is only one "level" deep.
    towel_prefixes: Vec<&'a str>,
}

fn is_pattern_possible<'a>(
    pattern: &'a str,
    towels: &[&'a str],
    cache: &mut PrefixCache<'a>,
) -> bool {
    if cache.contains_key(pattern) {
        return cache[pattern].possible;
    }

    let mut possible = false;
    let mut towel_prefixes: Vec<&str> = vec![];
    for &towel_prefix in towels {
        if let Some(next_pattern) = pattern.strip_prefix(towel_prefix) {
            let next_possible = is_pattern_possible(next_pattern, towels, cache);
            if next_possible {
                towel_prefixes.push(towel_prefix);
            }
            possible |= next_possible;
        }
    }

    cache.insert(
        pattern,
        State {
            possible,
            towel_prefixes,
        },
    );

    if cfg!(test) {
        let towel_prefixes = &cache[pattern].towel_prefixes;
        println!(
            "  + \"{pattern:>6}\" has {} prefixes(s): {towel_prefixes:?}",
            towel_prefixes.len()
        );
    }
    possible
}

fn build_p_cache<'a>(patterns: &[&'a str], towels: &[&'a str]) -> PrefixCache<'a> {
    let mut p_cache = PrefixCache::new();

    // The empty string is always creatable, and has no valid prefixes.
    // This is the only thing we seed our cache with, to account for towels with prefixes!
    // e.g. "g", "b", and "gb" are all towels, but one can create "gb" two ways.
    p_cache.insert(
        "",
        State {
            possible: true,
            towel_prefixes: vec![],
        },
    );

    // Run through our patterns and build up a cache
    for pattern in patterns {
        // If we've seen it, just use it.
        if p_cache.contains_key(pattern) {
            continue;
        }

        is_pattern_possible(pattern, towels, &mut p_cache);
    }

    if cfg!(test) {
        println!();
    }

    p_cache
}

fn build_c_cache<'a>(patterns: &[&'a str], p_cache: &PrefixCache<'a>) -> CountCache<'a> {
    let mut c_cache = CountCache::new();
    c_cache.insert("", 1);

    for &pattern in patterns {
        // If we've seen it, just use it.
        if c_cache.contains_key(pattern) {
            continue;
        }

        count_ways(pattern, p_cache, &mut c_cache);
    }

    for &pattern in patterns {
        let _ = c_cache.entry(pattern).or_insert(0);
    }

    if cfg!(test) {
        println!();
    }

    c_cache
}

fn count_ways<'a>(
    pattern: &'a str,
    p_cache: &PrefixCache<'a>,
    c_cache: &mut CountCache<'a>,
) -> i64 {
    if c_cache.contains_key(pattern) {
        return c_cache[pattern];
    }

    assert!(!pattern.is_empty());
    if cfg!(test) {
        println!("  + Counting \"{pattern:>6}\"");
    }

    let state = &p_cache[pattern];
    if !state.possible {
        return 0;
    }

    // "gbbr" will generate a few entries:
    //      "gbbr" -> {true, ["g", "gb"]},
    //      "bbr"  -> {true, ["b"]}
    //      "br"   -> {true, ["b", "br"]},
    //      "g"    -> {true, ["g"]},
    //      "b"    -> {true, ["b"]},
    //      ""     -> {true, []},
    // "gb" doesn't show up because while it starts our word, it's never inside of it.
    let mut count = 0;
    for prefix in &state.towel_prefixes {
        let suffix = pattern.strip_prefix(prefix).unwrap();
        count += count_ways(suffix, p_cache, c_cache);
    }

    if cfg!(test) {
        println!(
            "  + Counting \"{pattern:>6}\" == {count} (# prefix == {})",
            state.towel_prefixes.len()
        );
    }

    c_cache.insert(pattern, count);
    count
}

// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(input: &str) -> i64 {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect_vec();
    let patterns: Vec<&str> = patterns.lines().collect_vec();

    if cfg!(test) {
        println!("Found {} towels:   {towels:?}", towels.len());
        println!("Found {} patterns: {patterns:?}", patterns.len());
    }

    let p_cache = build_p_cache(&patterns, &towels);

    patterns.iter().filter(|&&p| p_cache[p].possible).count() as i64
}

// Part2 ========================================================================
#[aoc(day19, part2)]
pub fn part2(input: &str) -> i64 {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect_vec();
    let patterns: Vec<&str> = patterns.lines().collect_vec();

    if cfg!(test) {
        println!("Found {} towels:   {towels:?}", towels.len());
        println!("Found {} patterns: {patterns:?}", patterns.len());
    }

    let p_cache = build_p_cache(&patterns, &towels);
    let c_cache = build_c_cache(&patterns, &p_cache);

    patterns.iter().map(|&p| c_cache[p]).sum()
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
    // #[case::given(2 + 1 + 4 + 6 + 1 + 2, EXAMPLE_INPUT)]
    #[case::given_gbbr__(4, &ex("gbbr"))]
    // #[case::given_rrbgbr(6, &ex("rrbgbr"))]
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
