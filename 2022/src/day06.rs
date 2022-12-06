use aoc_runner_derive::aoc;

use itertools::Itertools;

// First =======================================================================
#[aoc(day6, part1, first)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    for i in 0..(input.len()) {
        let xs = &input[i..i + 4];
        let n = xs.iter().unique().count();
        if n == 4 {
            return i + 4;
        }
    }

    unreachable!();
}

#[aoc(day6, part2, first)]
#[inline(never)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();

    for i in 0..(input.len()) {
        let xs = &input[i..i + 14];
        let n = xs.iter().unique().count();
        if n == 14 {
            return i + 14;
        }
    }

    unreachable!();
}

// Bits ========================================================================
fn check_for_runs<const N: usize>(input: &[u8]) -> usize {
    if cfg!(debug_assertions) {
        debug_assert_ne!(N, 0);
        debug_assert!(N <= 32);

        for x in input {
            debug_assert!((b'a'..=b'z').contains(x));
        }
    }

    let mut seen = 0_u32;

    for x in input.iter().take(N) {
        seen ^= 1_u32 << (x - b'a');
    }

    if seen.count_ones() == N as u32 {
        return N;
    }

    for i in N..input.len() {
        // Remove the earliest input
        seen ^= 1_u32 << (input[i - N] - b'a');

        // Add the newest
        seen ^= 1_u32 << (input[i] - b'a');

        if seen.count_ones() == N as u32 {
            return i + 1;
        }
    }

    unreachable!("Reached end of input with no runs of unique markers");
}

#[aoc(day6, part1, bits)]
#[inline(never)]
pub fn part1_bits(input: &str) -> usize {
    check_for_runs::<4>(input.as_bytes())
}

#[aoc(day6, part2, bits)]
#[inline(never)]
pub fn part2_bits(input: &str) -> usize {
    check_for_runs::<14>(input.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::given_mjqj(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[case::given_bvwb(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[case::given_nppd(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[case::given_nznr(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[case::given_zcfz(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_bits)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given_mjqj(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[case::given_bvwb(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[case::given_nppd(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[case::given_nznr(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[case::given_zcfz(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_bits)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
