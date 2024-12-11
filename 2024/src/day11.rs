use crate::prelude::*;

fn print_stones(stones: &[i64], blinks: u32) {
    if cfg!(test) && stones.len() < 30 {
        if blinks == 0 {
            println!("Initial arrangement:");
        } else if blinks == 1 {
            println!("After 1 blink:");
        } else {
            println!("After {blinks} blinks:");
        }
        for stone in stones {
            print!("{stone} ");
        }
        println!();
    }
}

// Part1 ========================================================================
fn after_blinks_slow(input: &str, times: u32) -> i64 {
    fn half_digits(x: i64) -> Option<(i64, i64)> {
        const LUT: [i64; 7] = [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];
        let n_digits = 1 + x.ilog(10) as usize;
        if n_digits % 2 == 0 {
            let idx = n_digits / 2 - 1;
            Some((x / LUT[idx], x % LUT[idx]))
        } else {
            None
        }
    }
    let mut stones: Vec<i64> = input
        .split_ascii_whitespace()
        .map(parse_or_fail)
        .collect_vec();

    print_stones(&stones, 0);

    for blinks in 1..=times {
        let mut next = vec![];
        for stone in stones {
            if stone == 0 {
                // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                next.push(1);
            } else if let Some((left, right)) = half_digits(stone) {
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
                // if cfg!(test) { println!("{stone} -> [{left}, {right}]"); }
                next.push(left);
                next.push(right);
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
                next.push(2024 * stone);
            }
        }
        stones = next;

        print_stones(&stones, blinks);
    }

    stones.len() as i64
}

fn print_stones_p2(stones: &HashMap<usize, i64>, blinks: u32) {
    if cfg!(test) {
        println!("[{blinks}] Have {} stones:", stones.values().sum::<i64>());
        let mut counts = stones.iter().collect_vec();
        counts.sort();

        for (stone, count) in counts {
            if *count != 0 {
                println!("  + {count:>4} of Stone {stone:>12}");
            }
        }
    }
}

fn after_blinks(input: &str, times: u32) -> i64 {
    fn half_digits(x: usize) -> Option<(usize, usize)> {
        const LUT: [usize; 7] = [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];
        let n_digits = 1 + x.ilog(10) as usize;
        if n_digits % 2 == 0 {
            let idx = n_digits / 2 - 1;
            Some((x / LUT[idx], x % LUT[idx]))
        } else {
            None
        }
    }

    // stones[i] == n => there are n stones of value i
    let mut stones: HashMap<usize, i64> = HashMap::new();

    for stone in input.split_ascii_whitespace() {
        *stones.entry(parse_or_fail(stone)).or_default() += 1;
    }
    print_stones_p2(&stones, 0);

    for blinks in 1..=times {
        let mut next = HashMap::new();
        for (stone, count) in stones.into_iter() {
            if stone == 0 {
                // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                *next.entry(1).or_default() += count;
            } else if let Some((left, right)) = half_digits(stone) {
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
                *next.entry(left).or_default() += count;
                *next.entry(right).or_default() += count;
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
                *next.entry(2024 * stone).or_default() += count;
            }
        }
        stones = next;
        print_stones_p2(&stones, blinks);
    }
    print_stones_p2(&stones, 0);

    stones.values().sum()
}

#[aoc(day11, part1, slow)]
pub fn part1_slow(input: &str) -> i64 {
    after_blinks_slow(input, 25)
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    after_blinks(input, 25)
}

// Part2 ========================================================================
#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    after_blinks(input, 75)
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"125 17";

    #[rstest]
    #[case::given_just_0_times(2, (EXAMPLE_INPUT, 0))]
    #[case::given_just_1_time(3, (EXAMPLE_INPUT, 1))]
    #[case::given_just_2_time(4, (EXAMPLE_INPUT, 2))]
    #[case::given_just_3_time(5, (EXAMPLE_INPUT, 3))]
    #[case::given_just_4_time(9, (EXAMPLE_INPUT, 4))]
    #[case::given_just_5_time(13, (EXAMPLE_INPUT, 5))]
    #[case::given_just_6_time(22, (EXAMPLE_INPUT, 6))]
    #[case::given(55312, (EXAMPLE_INPUT, 25))]
    #[trace]
    #[timeout(Duration::from_millis(100))]
    fn check_blinks_small(
        #[notrace]
        #[values(after_blinks_slow, after_blinks)]
        p: impl FnOnce(&str, u32) -> i64,
        #[case] expected: i64,
        #[case] (input, times): (&str, u32),
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input, times), expected);
    }

    #[rstest]
    #[case::given(65601038650482, (EXAMPLE_INPUT, 75))]
    #[trace]
    #[timeout(Duration::from_millis(750))]
    fn check_blinks_big(
        #[notrace]
        #[values(after_blinks)]
        p: impl FnOnce(&str, u32) -> i64,
        #[case] expected: i64,
        #[case] (input, times): (&str, u32),
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input, times), expected);
    }
}
