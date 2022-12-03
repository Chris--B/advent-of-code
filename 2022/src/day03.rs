use aoc_runner_derive::aoc;

use std::collections::HashSet;

pub fn parse_input_part_1(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let mut bytes = input.as_bytes().to_owned();

    for b in &mut bytes {
        *b = match *b {
            b'a'..=b'z' => *b - b'a' + 1,
            b'A'..=b'Z' => *b - b'A' + 27,
            b'\n' => u8::MAX,
            _ => unreachable!("Unexpected byte: {} ({})", *b as char, *b),
        };
    }

    bytes
        .split(|b| *b == u8::MAX)
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            debug_assert_eq!(a.len(), b.len());

            (
                HashSet::from_iter(a.iter().copied()),
                HashSet::from_iter(b.iter().copied()),
            )
        })
        .collect()
}

// Part1 ========================================================================
#[aoc(day3, part1, std_set)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let input = parse_input_part_1(input);
    let mut priority = 0;

    for (a, b) in &input {
        let mut iter = a.intersection(b);
        priority += *iter.next().unwrap() as i64;

        debug_assert_eq!(iter.next(), None);
    }

    priority
}

#[aoc(day3, part1, int_bitset)]
#[inline(never)]
pub fn part1_intbitset(input: &str) -> i64 {
    let input = input.as_bytes();

    let mut priority = 0;

    for line in input.split(|b| *b == b'\n') {
        let m = line.len() / 2;
        let (a, b) = line.split_at(m);
        debug_assert_eq!(a.len(), b.len());

        let a: u64 = a
            .iter()
            .copied()
            .map(|x| {
                let x = match x {
                    b'a'..=b'z' => x - b'a' + 1,
                    b'A'..=b'Z' => x - b'A' + 27,
                    _ => unreachable!(),
                };
                1 << x
            })
            .fold(0, |acc, x| acc | x);

        let b: u64 = b
            .iter()
            .copied()
            .map(|x| {
                let x = match x {
                    b'a'..=b'z' => x - b'a' + 1,
                    b'A'..=b'Z' => x - b'A' + 27,
                    _ => unreachable!(),
                };
                1 << x
            })
            .fold(0, |acc, x| acc | x);

        debug_assert_eq!((a & b).count_ones(), 1);
        priority += (a & b).trailing_zeros();
    }

    priority as i64
}

// Part2 ========================================================================

pub fn parse_input_part_2(input: &str) -> Vec<(u64, u64)> {
    let mut bytes = input.as_bytes().to_owned();

    for b in &mut bytes {
        *b = match *b {
            b'a'..=b'z' => *b - b'a' + 1,
            b'A'..=b'Z' => *b - b'A' + 27,
            b'\n' => u8::MAX,
            _ => unreachable!("Unexpected byte: {} ({})", *b as char, *b),
        };
    }

    let mut pairs = vec![];

    for (idx, line) in bytes.split(|b| *b == u8::MAX).enumerate() {
        if idx % 6 == 0 {
            pairs.push((0_u64, 0_u64));
        }
        let p = &mut pairs.last_mut().unwrap();
        let s: &mut u64 = if idx % 6 < 3 { &mut p.0 } else { &mut p.1 };

        if idx % 3 == 0 {
            // first entry, erase what's there
            for b in line {
                *s |= 1 << *b;
            }
        } else {
            // second or 3rd entry, intersect what's there
            let mut r = 0_u64;
            for b in line {
                r |= 1 << *b;
            }

            *s &= r;
        }
    }

    pairs
}

#[aoc(day3, part2, int_bitset)]
#[inline(never)]
pub fn part2_intbitset(input: &str) -> i64 {
    debug_assert_eq!(input.lines().count() % 6, 0);
    let input = parse_input_part_2(input);

    let mut priority = 0;

    for (a, b) in &input {
        // println!("a: 0b{a:064b} - {}", a.trailing_zeros());
        // println!("b: 0b{b:064b} - {}", b.trailing_zeros());
        // println!();

        priority += a.trailing_zeros() + b.trailing_zeros();
    }

    priority as i64
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[rstest]
    #[case::given(157, EXAMPLE_INPUT)]
    #[case::given_line1(16, "vJrwpWtwJgWrhcsFMMfFFhFp")]
    #[case::given_line2(38, "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_intbitset)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(70, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2_intbitset)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
