#![allow(unused)]

use indicatif::ProgressBar;

use crate::prelude::*;

const MAX_SECRET: i64 = ((1 << 24) - 1);

fn secrets(seed: i64) -> impl Iterator<Item = i64> {
    let mut secret = seed;
    (0..=2_000).map(move |_| {
        let ret = secret;

        secret ^= secret << 6;
        secret &= ((1 << 24) - 1);

        secret ^= secret >> 5;
        // secret &= ((1 << 24) - 1);

        secret ^= secret << 11;
        secret &= ((1 << 24) - 1);

        ret
    })
}

fn prices(seed: i64) -> impl Iterator<Item = i64> {
    secrets(seed).map(|s| s % 10)
}

fn price_changes(seed: i64) -> impl Iterator<Item = i64> {
    prices(seed).tuple_windows().map(|(a, b)| b - a)
}

fn price_after_seq(seed: i64, seq: [i64; 4]) -> Option<i64> {
    for (a, b, c, d, e) in prices(seed).tuple_windows() {
        if [b - a, c - b, d - c, e - d] == seq {
            return Some(e);
        }
    }

    None
}

// Part1 ========================================================================
#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .i64s()
        .map(|secret| secrets(secret).nth(2_000).unwrap())
        .sum()
}

// Part2 ========================================================================
#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    use rayon::prelude::*;

    let seeds: Vec<i64> = input.i64s().collect_vec();

    let mut seen = HashMap::new();
    for &seed in &seeds {
        for (a, b, c, d, e) in prices(seed).tuple_windows() {
            *seen.entry([b - a, c - b, d - c, e - d]).or_insert(0) += 1;
        }
    }

    let worth_while = seen.iter().filter(|&(k, v)| *v > seeds.len() / 6);
    let estimate = worth_while.clone().count() as u64;
    println!(
        "Found {} unique sequences... but only {estimate} are worth checking (show up in >1/6 seeds)",
        seen.len()
    );

    let (&seq, &count) = worth_while
        .max_by_key(|(seq, _)| -> i64 {
            seeds
                .par_iter()
                .filter_map(|seed| price_after_seq(*seed, **seq))
                .sum()
        })
        .unwrap();

    println!("Found good seq: {seq:?}!");
    println!(
        "  + {count} seeds ({percent:.2}%) have it",
        percent = 100. * (count as f32 / seeds.len() as f32)
    );

    seeds
        .iter()
        .filter_map(|&seed| price_after_seq(seed, seq))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[test]
    fn check_secrets() {
        assert_eq!(
            secrets(123).take(11).collect_vec(),
            vec![
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254,
            ]
        );
    }

    #[test]
    fn check_prices() {
        assert_eq!(
            prices(123).take(10).collect_vec(),
            vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
        );
    }

    #[test]
    fn check_price_changes() {
        // Prices: vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
        assert_eq!(
            price_changes(123).take(9).collect_vec(),
            vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]
        )
    }

    const EXAMPLE_INPUT_P1: &str = r"
1
10
100
2024
";

    #[rstest]
    #[case::given(37327623, EXAMPLE_INPUT_P1)]
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

    const EXAMPLE_INPUT_P2: &str = r"
1
2
3
2024
";
    #[rstest]
    #[case::given(23, EXAMPLE_INPUT_P2)]
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
