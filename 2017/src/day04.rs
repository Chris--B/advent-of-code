#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
fn is_valid_password_v1(pw: impl AsRef<str>) -> bool {
    let mut words: Vec<&str> = pw.as_ref().split_ascii_whitespace().collect_vec();
    words.sort();

    std::iter::zip(words[..].iter(), words[1..].iter())
        .filter(|(a, b)| a == b)
        .count()
        == 0
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .filter(|s| is_valid_password_v1(s))
        .count() as i64
}

// Part2 ========================================================================
fn counts_of(word: &str) -> [u8; 26] {
    let mut counts = [0; 26];

    for c in word.as_bytes() {
        match c {
            b'a'..=b'z' => {
                counts[(c - b'a') as usize] += 1;
            }
            _ => unreachable!("Invalid character '{c}' found in word {word:?}"),
        }
    }

    counts
}

fn is_valid_password_v2(pw: impl AsRef<str>) -> bool {
    let mut counts: Vec<[u8; 26]> = pw
        .as_ref()
        .split_ascii_whitespace()
        .map(counts_of)
        .collect_vec();
    counts.sort();

    std::iter::zip(counts[..].iter(), counts[1..].iter())
        .filter(|(a, b)| a == b)
        .count()
        == 0
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        // Turns out this check isn't even used on our input!
        // .filter(|s| is_valid_password_v1(s) && is_valid_password_v2(s))
        .filter(|s| is_valid_password_v2(s))
        .count() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_1: &str = r"
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa
";

    #[rstest]
    #[case::given(true, "aa bb cc dd ee")]
    #[case::given(false, "aa bb cc dd aa")]
    #[case::given(true, "aa bb cc dd aaa")]
    #[trace]
    fn check_is_valid_pw_part1(#[case] is_really_valid: bool, #[case] pw: &str) {
        let actual = is_valid_password_v1(pw);
        assert_eq!(
            actual,
            is_really_valid,
            "password {pw:?} is supposed to be {expected} but was found to be {actual}",
            expected = if is_really_valid { "valid" } else { "invalid" },
            actual = if actual { "valid" } else { "invalid" }
        );
    }

    #[rstest]
    #[case::given(2, EXAMPLE_INPUT_1)]
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

    const EXAMPLE_INPUT_2: &str = r"
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi
oiii ioii iioi iiio
";

    #[rstest]
    #[case::given(true, "abcde fghij")]
    #[case::given(false, "abcde xyz ecdab")]
    #[case::given(true, "a ab abc abd abf abj")]
    #[case::given(true, "iiii oiii ooii oooi")]
    #[case::given(false, "oiii ioii iioi iiio")]
    #[trace]
    fn check_is_valid_pw_part2(#[case] is_really_valid: bool, #[case] pw: &str) {
        let actual = is_valid_password_v2(pw);
        assert_eq!(
            actual,
            is_really_valid,
            "password {pw:?} is supposed to be {expected} but was found to be {actual}",
            expected = if is_really_valid { "valid" } else { "invalid" },
            actual = if actual { "valid" } else { "invalid" }
        );
    }

    #[rstest]
    #[case::given(3, EXAMPLE_INPUT_2)]
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
