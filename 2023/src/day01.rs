use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.as_bytes().split(|b| *b == b'\n') {
        let a = line
            .iter()
            .copied()
            .find(u8::is_ascii_digit)
            .map(|b| b - b'0')
            .unwrap() as i64;

        let b = line
            .iter()
            .copied()
            .rfind(u8::is_ascii_digit)
            .map(|b| b - b'0')
            .unwrap() as i64;

        sum += a * 10 + b;
    }

    sum
}

#[aoc(day1, part1, v2)]
pub fn part1_v2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.as_bytes().split(|b| *b == b'\n') {
        let mut a = 0;
        for i in 0..line.len() {
            if let Some(c) = line.get(i) {
                if let b'0'..=b'9' = *c {
                    a = (c - b'0') as i64;
                }
            }
            if a != 0 {
                break;
            }
        }

        let mut b = 0;
        for i in 0..line.len() {
            let j = line.len() - i - 1;

            if let Some(c) = line.get(j) {
                if let b'0'..=b'9' = *c {
                    b = (c - b'0') as i64;
                }
            }
            if b != 0 {
                break;
            }
        }

        sum += a * 10 + b;
    }

    sum
}

// Part2 ========================================================================
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        sum += resolve_line(line);
    }

    sum
}

fn resolve_line(line: &str) -> i64 {
    const THINGS: &[(&str, i64)] = &[
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let a = THINGS
        .iter()
        .filter_map(|(s, n)| line.find(*s).map(|p| (p, *n)))
        .min_by_key(|(p, _n)| *p)
        .map(|(_p, n)| n)
        .unwrap();

    let b = THINGS
        .iter()
        .filter_map(|(s, n)| line.rfind(*s).map(|p| (p, *n)))
        .max_by_key(|(p, _n)| *p)
        .map(|(_p, n)| n)
        .unwrap();

    a * 10 + b
}

#[aoc(day1, part2, v2)]
pub fn part2_v2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        sum += resolve_line_v2(line);
    }

    sum
}

fn resolve_line_v2(line: &str) -> i64 {
    if line.is_empty() {
        return 0;
    }

    let line = line.as_bytes();

    let mut a = 0;
    for i in 0..line.len() {
        a = check_front(&line[i..]);
        if a != 0 {
            break;
        }
    }

    let mut b = 0;
    for i in 0..line.len() {
        let j = line.len() - i;
        b = check_back(&line[..j]);
        if b != 0 {
            break;
        }
    }

    return a * 10 + b;

    fn check_front(line: &[u8]) -> i64 {
        assert!(!line.is_empty());

        if let Some(c) = line.first() {
            if let b'0'..=b'9' = *c {
                return (c - b'0') as i64;
            }
        }

        if line.starts_with(b"one") {
            1
        } else if line.starts_with(b"two") {
            2
        } else if line.starts_with(b"three") {
            3
        } else if line.starts_with(b"four") {
            4
        } else if line.starts_with(b"five") {
            5
        } else if line.starts_with(b"six") {
            6
        } else if line.starts_with(b"seven") {
            7
        } else if line.starts_with(b"eight") {
            8
        } else if line.starts_with(b"nine") {
            9
        } else {
            0
        }
    }

    fn check_back(line: &[u8]) -> i64 {
        assert!(!line.is_empty());

        if let Some(c) = line.last() {
            if let b'0'..=b'9' = *c {
                return (c - b'0') as i64;
            }
        }

        if line.ends_with(b"one") {
            1
        } else if line.ends_with(b"two") {
            2
        } else if line.ends_with(b"three") {
            3
        } else if line.ends_with(b"four") {
            4
        } else if line.ends_with(b"five") {
            5
        } else if line.ends_with(b"six") {
            6
        } else if line.ends_with(b"seven") {
            7
        } else if line.ends_with(b"eight") {
            8
        } else if line.ends_with(b"nine") {
            9
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const EXAMPLE_INPUT_2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[rstest]
    #[case::ex_line0(12, "1abc2")]
    #[case::ex_line1(38, "pqr3stu8vwx")]
    #[case::ex_line2(15, "a1b2c3d4e5f")]
    #[case::ex_line3(77, "treb7uchet")]
    #[case::given(142, EXAMPLE_INPUT_1)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_v2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::ex_line0(29, "two1nine")]
    #[case::ex_line1(83, "eightwothree")]
    #[case::ex_line2(13, "abcone2threexyz")]
    #[case::ex_line3(24, "xtwone3four")]
    #[case::ex_line4(42, "4nineeightseven2")]
    #[case::ex_line5(14, "zoneight234")]
    #[case::ex_line6(76, "7pqrstsixteen")]
    #[case::given(281, EXAMPLE_INPUT_2)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_v2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
