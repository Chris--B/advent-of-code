#![allow(dead_code, unused)]
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
struct Map {
    dst: i64,
    src: i64,
    count: i64,
}

impl Map {
    fn new(dst: i64, src: i64, count: i64) -> Self {
        Self { src, dst, count }
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn find_in_map(map: &[Map], src: i64) -> i64 {
    let map = map
        .iter()
        .find(|map| map.src <= src && src < map.src + map.count);

    if let Some(map) = map {
        if map.src <= src && src <= map.src + map.count {
            // dbg!(src, map, map.src, map.dst, (src - map.dst));
            return map.dst + (src - map.src);
        }
    }

    // 1-to-1
    src
}

impl Almanac {
    fn from_str(s: &str) -> Self {
        let mut maps: Vec<Vec<Map>> = vec![];

        let mut i = 0;
        maps.push(vec![]);

        for (is_empty, group) in &s.lines().group_by(|line| line.is_empty()) {
            if is_empty {
                maps.push(vec![]);
                i += 1;
            } else {
                for line in group.into_iter().skip(1) {
                    let mut parts = line.split_whitespace();
                    let dst: i64 = parts.next().unwrap().parse().unwrap();
                    let src: i64 = parts.next().unwrap().parse().unwrap();
                    let count: i64 = parts.next().unwrap().parse().unwrap();

                    maps[i].push(Map { dst, src, count });
                }
            }
        }

        // Note: This is BACKWARDS from the order in the Almanac struct because we are popping off of our list.
        let humidity_to_location = maps.pop().unwrap();
        let temperature_to_humidity = maps.pop().unwrap();
        let light_to_temperature = maps.pop().unwrap();
        let water_to_light = maps.pop().unwrap();
        let fertilizer_to_water = maps.pop().unwrap();
        let soil_to_fertilizer = maps.pop().unwrap();
        let seed_to_soil = maps.pop().unwrap();

        // Should be empty
        debug_assert_eq!(maps, Vec::<Vec<_>>::new());

        Self {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn get_seed_to_soil(&self, seed: i64) -> i64 {
        find_in_map(&self.seed_to_soil, seed)
    }

    fn get_soil_to_fertilizer(&self, seed: i64) -> i64 {
        find_in_map(&self.soil_to_fertilizer, seed)
    }

    fn get_fertilizer_to_water(&self, seed: i64) -> i64 {
        find_in_map(&self.fertilizer_to_water, seed)
    }

    fn get_water_to_light(&self, seed: i64) -> i64 {
        find_in_map(&self.water_to_light, seed)
    }

    fn get_light_to_temperature(&self, seed: i64) -> i64 {
        find_in_map(&self.light_to_temperature, seed)
    }

    fn get_temperature_to_humidity(&self, seed: i64) -> i64 {
        find_in_map(&self.temperature_to_humidity, seed)
    }

    fn get_humidity_to_location(&self, seed: i64) -> i64 {
        find_in_map(&self.humidity_to_location, seed)
    }

    fn get_seed_to_location(&self, seed: i64) -> i64 {
        let x = self.get_seed_to_soil(seed);
        let x = self.get_soil_to_fertilizer(x);
        let x = self.get_fertilizer_to_water(x);
        let x = self.get_water_to_light(x);
        let x = self.get_light_to_temperature(x);
        let x = self.get_temperature_to_humidity(x);
        self.get_humidity_to_location(x)
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

    let num_seeds_total: i64 = seeds.iter().map(|(a, b)| b).sum();
    dbg!(num_seeds_total);

    if cfg!(test) {
        println!();
        for label in [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temp",
            "humidity",
            "location",
        ] {
            print!("{label}, ");
        }
        println!();

        for row in seeds.iter().flat_map(|(a, b)| {
            let a = *a;
            let b = a + *b;

            (a..=b).map(|s| {
                let x0 = almanac.get_seed_to_soil(s);
                let x1 = almanac.get_soil_to_fertilizer(x0);
                let x2 = almanac.get_fertilizer_to_water(x1);
                let x3 = almanac.get_water_to_light(x2);
                let x4 = almanac.get_light_to_temperature(x3);
                let x5 = almanac.get_temperature_to_humidity(x4);
                let x6 = almanac.get_humidity_to_location(x5);

                [s, x0, x1, x2, x3, x4, x5, x6]
            })
        }) {
            for v in row {
                print!("{v}, ");
            }
            println!();
        }

        println!();
    }

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
    #[case::sample_line_1([50, 98, 2], Map { dst: 50, src: 98, count: 2})]
    #[case::sample_line_2([52, 50, 48], Map { dst: 52, src: 50, count: 48})]
    #[trace]
    fn check_sample_lines(#[case] nums: [i64; 3], #[case] map: Map) {
        assert_eq!(Map::new(nums[0], nums[1], nums[2]), map);
    }

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

        assert_eq!(almanac.get_seed_to_soil(seed), soil);
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
