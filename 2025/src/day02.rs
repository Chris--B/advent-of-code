#![allow(unused)]

use crate::prelude::*;

fn hex_to_dec(mut x: i64) -> i64 {
    let mut d = 0;
    let mut m = 1;

    while x > 0 {
        assert!((x & 0xf) < 10, "{} not a dec digit", x & 0xf);
        d += (x & 0xf) * m;
        m *= 10;
        x >>= 4;
    }

    d
}

// Part1 ========================================================================
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    fn invalid(id: i64) -> bool {
        {
            let mut idx = id;
            while idx > 0 {
                if idx & 0xf > 0x9 {
                    return false;
                }
                idx >>= 4;
            }
        }

        let mut shift = 4;
        loop {
            if 2 * shift > 63 {
                break;
            }

            let mask: i64 = (1 << shift) - 1;

            if id > (1 << (2 * shift - 4)) {
                if (id & mask) == (id >> shift) {
                    return true;
                }
            }

            shift += 4;
        }

        false
    }

    input
        .split(",")
        .flat_map(|ab| ab.split("-").map(|x| i64::from_str_radix(x, 16).unwrap()))
        .tuples()
        // .inspect(|x| println!("{x:x?}"))
        .flat_map(|(lo, hi)| lo..=(hi))
        .filter(|&id| invalid(id))
        .map(hex_to_dec)
        // .inspect(|id| println!("{id:x?}"))
        .sum()
}

// Part2 ========================================================================
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    fn invalid(id: i64) -> bool {
        {
            let mut idx = id;
            while idx > 0 {
                if idx & 0xf > 0x9 {
                    return false;
                }
                idx >>= 4;
            }
        }

        let mut shift = 4;
        loop {
            if 2 * shift > 63 {
                break;
            }

            let mask: i64 = (1 << shift) - 1;

            if id > (1 << (2 * shift - 4)) {
                if (id & mask) == (id >> shift) {
                    return true;
                }
            }

            shift += 4;
        }

        false
    }

    input
        .split(",")
        .flat_map(|ab| ab.split("-").map(|x| i64::from_str_radix(x, 16).unwrap()))
        .tuples()
        // .inspect(|x| println!("{x:x?}"))
        .flat_map(|(lo, hi)| lo..=(hi))
        .filter(|&id| invalid(id))
        .map(hex_to_dec)
        // .inspect(|id| println!("{id:x?}"))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[rstest]
    #[case(0x1, 1)]
    #[case(0x12, 12)]
    #[case(0x1234, 1234)]
    fn check_hex_to_dec(#[case] hex: i64, #[case] dec: i64) {
        assert_eq!(hex_to_dec(hex), dec);
    }

    #[rstest]
    #[case::given(1227775554, EXAMPLE_INPUT)]
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
    #[case::given(4174379265, EXAMPLE_INPUT)]
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
