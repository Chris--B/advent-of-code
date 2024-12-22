use crate::prelude::*;

#[cfg(target_feature = "neon")]
use core::arch::aarch64::*;

fn secrets(seed: i64) -> impl Iterator<Item = i64> {
    std::iter::successors(Some(seed), |secret| {
        let mut secret = *secret;

        secret ^= secret << 6;
        secret &= (1 << 24) - 1;

        secret ^= secret >> 5;
        // secret &= (1 << 24) - 1;

        secret ^= secret << 11;
        secret &= (1 << 24) - 1;

        Some(secret)
    })
    .take(2_001)
}

fn prices(seed: i64) -> impl Iterator<Item = i64> {
    secrets(seed).map(|s| s % 10)
}

#[allow(unused)]
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

#[cfg(target_feature = "neon")]
fn sum_secret_iters_neon(secret: &[u32; 4], times: usize) -> i64 {
    unsafe {
        let mask = vld1q_dup_u32(&((1 << 24) - 1));
        let neg5 = vld1q_dup_s32(&-5);

        let mut secret = vld1q_u32(secret.as_ptr());

        for _ in 0..times {
            // secret ^= secret << 6;
            secret = veorq_u32(secret, vshlq_n_u32(secret, 6));
            // secret &= ((1 << 24) - 1);
            secret = vandq_u32(secret, mask);

            // secret ^= secret >> 5;
            secret = veorq_u32(secret, vshlq_u32(secret, neg5));

            // secret ^= secret << 11;
            secret = veorq_u32(secret, vshlq_n_u32(secret, 11));
            // secret &= ((1 << 24) - 1);
            secret = vandq_u32(secret, mask);
        }

        vaddvq_u32(secret) as i64
    }
}

// Part1 ========================================================================
#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .i64s()
        .map(|secret| secrets(secret).nth(2_000).unwrap())
        .sum()
}

#[cfg(target_feature = "neon")]
#[aoc(day22, part1, neon)]
pub fn part1_neon(input: &str) -> i64 {
    let mut seeds: Vec<u32> = input.i64s().map(|s| s as _).collect_vec();

    // 0 never changes, so we can pad freely
    while seeds.len() % 4 != 0 {
        seeds.push(0);
    }

    let mut sum: i64 = 0;
    for i in 0..(seeds.len() / 4) {
        unsafe {
            let reg: [_; 4] = std::ptr::read(seeds[4 * i..].as_ptr() as *const _);
            sum += sum_secret_iters_neon(&reg, 2_000);
        }
    }

    sum
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

    let worth_while = seen.iter().filter(|&(_k, v)| *v > seeds.len() / 6);
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

    #[test]
    #[cfg(target_feature = "neon")]
    fn check_sum_secret_iters_neon() {
        let seed = 123;
        let n = 10;

        let mut answers: Vec<_> = vec![];
        for i in 0..n {
            // Note: 0 stays 0 so we can ignore it
            let ans = sum_secret_iters_neon(&[seed, 0, 0, 0], i);
            answers.push(ans);
        }

        let expected = secrets(seed as i64).take(n).map(|s| s as _).collect_vec();
        if answers != expected {
            for (a, b) in std::iter::zip(&answers, &expected) {
                if a != b {
                    println!("Checking {a} vs {b}");
                    println!("  0b{a:>024b}");
                    println!("  0b{b:>024b}");
                }
            }
        }

        assert_eq!(answers, expected);
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

    #[rstest]
    #[case::given(37327623, EXAMPLE_INPUT_P1)]
    #[trace]
    #[cfg(target_feature = "neon")]
    fn check_ex_part_1_neon(
        #[notrace]
        #[values(part1_neon)]
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
