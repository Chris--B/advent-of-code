use crate::prelude::*;

fn same_digits(id: i64, base: i64) -> bool {
    let (a, b) = id.div_rem(&base);
    if b < (base / 10) {
        return false;
    }
    if a == b {
        return true;
    }

    false
}

// Part1 ========================================================================
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    fn invalid(id: i64) -> bool {
        let mut b = 10;
        loop {
            if same_digits(id, b) {
                return true;
            }
            let Some(bb) = b.checked_mul(10) else {
                break;
            };
            b = bb;
        }

        false
    }

    input
        .i64s()
        .tuples()
        .flat_map(|(lo, hi)| lo..=(-hi))
        .filter(|&id| invalid(id))
        .sum()
}

#[aoc(day2, part1, only_invalid)]
pub fn part1_only_invalid(input: &str) -> i64 {
    let mut ranges: Vec<_> = input.i64s().tuples().map(|(lo, hi)| (lo, (-hi))).collect();
    ranges.sort();

    if cfg!(test) {
        let mut by_low = ranges.clone();
        by_low.sort_by_key(|(a, _b)| *a);

        let mut by_high = ranges.clone();
        by_high.sort_by_key(|(_a, b)| *b);

        assert_eq!(by_low, by_high);
    }

    let mut sum = 0;

    for n in 1..=99999 {
        let size = (n as f64).log10().floor() as u32 + 1;
        let id: i64 = n + n * 10.pow(size);

        if id > ranges.last().unwrap().1 {
            break;
        }

        if let Ok(_) = ranges.binary_search_by(|(a, b)| {
            use std::cmp::Ordering::*;

            if id < *a {
                Greater
            } else if id > *b {
                Less
            } else {
                Equal
            }
        }) {
            sum += id
        };
    }

    sum
}

// Part2 ========================================================================
#[allow(unused)]
// #[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[aoc(day2, part2, only_invalid)]
pub fn part2_only_invalid(input: &str) -> i64 {
    let mut ranges: Vec<_> = input.i64s().tuples().map(|(lo, hi)| (lo, (-hi))).collect();
    ranges.sort();

    if cfg!(test) {
        println!("{} ranges", ranges.len());

        let mut by_low = ranges.clone();
        by_low.sort_by_key(|(a, _b)| *a);

        let mut by_high = ranges.clone();
        by_high.sort_by_key(|(_a, b)| *b);

        assert_eq!(by_low, by_high, "Ranges overlap");
    }

    let mut ids = HashSet::new();

    for times in 2..10 {
        for n in 1.. {
            let size = (n as f64).log10().floor() as u32 + 1;
            let mut id: i64 = 0;
            for t in 0..times {
                id += n * 10.pow(size * t);
            }

            if id > ranges.last().unwrap().1 {
                break;
            }

            if let Ok(_) = ranges.binary_search_by(|(a, b)| {
                use std::cmp::Ordering::*;

                if id < *a {
                    Greater
                } else if id > *b {
                    Less
                } else {
                    Equal
                }
            }) {
                if cfg!(test) {
                    println!("id={id} is {n} repeated {times} times");
                }
                ids.insert(id);
            };
        }
    }

    ids.into_iter().sum()
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
    #[case::given(1227775554, EXAMPLE_INPUT)]
    #[case::just_11(11, "11-12")]
    #[case::just_99_to_1010(1109, "99-1010")]
    #[case::just_11_and_99_to_1010(11+1109, "99-1010,11-12")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_only_invalid)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[test]
    #[should_panic]
    fn check_overlapping_ranges() {
        part1_only_invalid("1-11,2-3,3-11");
    }

    #[rstest]
    #[case::given(4174379265, EXAMPLE_INPUT)]
    #[case::given(99+111, "95-115")]
    #[timeout(Duration::from_millis(100))]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(
            // part2,
            part2_only_invalid)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
