use crate::prelude::*;

fn parse_seeds_p1(s: &str) -> Vec<i64> {
    debug_assert!(!s.is_empty());
    debug_assert_eq!(s.trim(), s);

    // Example line:
    //      seeds: 79 14 55 13
    s[7..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_seeds_p2(s: &str) -> Vec<(i64, i64)> {
    debug_assert!(!s.is_empty());
    debug_assert_eq!(s.trim(), s);

    // Example line:
    //      seeds: 79 14 55 13
    s[7..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .tuples()
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MapEntry {
    dst: i64,
    src: i64,
    count: i64,
}

#[derive(Clone, Debug)]
struct Almanac {
    maps: [Vec<MapEntry>; 7],
}

impl Almanac {
    fn from_str(s: &str) -> Self {
        let mut maps: [Vec<MapEntry>; 7] = Default::default();
        let mut i = 0;

        for (is_empty, group) in &s.lines().group_by(|line| line.is_empty()) {
            if is_empty {
                i += 1;
            } else {
                for line in group.into_iter().skip(1) {
                    let mut parts = line.split_whitespace();
                    let dst: i64 = parts.next().unwrap().parse().unwrap();
                    let src: i64 = parts.next().unwrap().parse().unwrap();
                    let count: i64 = parts.next().unwrap().parse().unwrap();

                    maps[i].push(MapEntry { dst, src, count });
                }
            }
        }

        Self { maps }
    }

    fn find_in_map(&self, src: i64, map: &[MapEntry]) -> i64 {
        if let Some(e) = map.iter().find(|e| e.src <= src && src < e.src + e.count) {
            if e.src <= src && src <= e.src + e.count {
                return e.dst + (src - e.src);
            }
        }

        // 1-to-1
        src
    }

    fn get_seed_to_location(&self, mut rsrc: i64) -> i64 {
        for map in &self.maps {
            rsrc = self.find_in_map(rsrc, map);
        }

        rsrc
    }
}

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let (seeds_line, input) = input.trim().split_once('\n').unwrap();
    let seeds = parse_seeds_p1(seeds_line);
    let almanac = Almanac::from_str(input.trim());

    seeds
        .iter()
        .map(|s| almanac.get_seed_to_location(*s))
        .min()
        .unwrap()
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let (seeds_line, input) = input.trim().split_once('\n').unwrap();
    let seeds = parse_seeds_p2(seeds_line);
    let almanac = Almanac::from_str(input.trim());

    let num_seeds_total: i64 = seeds.iter().map(|ab| ab.1).sum();
    dbg!(num_seeds_total);

    seeds
        .iter()
        .flat_map(|(a, b)| {
            let a = *a;
            let b = a + *b;

            (a..=b).map(|s| almanac.get_seed_to_location(s))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[rstest]
    #[case::seed_79(79, 81)]
    #[case::seed_14(14, 14)]
    #[case::seed_55(55, 57)]
    #[case::seed_13(13, 13)]
    #[trace]
    fn check_ex_seed_to_soil(#[case] seed: i64, #[case] soil: i64) {
        let (seeds_line, input) = EXAMPLE_INPUT.trim().split_once('\n').unwrap();
        let _seeds = parse_seeds_p1(seeds_line);
        let almanac = Almanac::from_str(input.trim());

        assert_eq!(almanac.find_in_map(seed, &almanac.maps[0]), soil);
    }

    #[rstest]
    #[case::given(35, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(46, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
