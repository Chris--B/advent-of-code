use crate::prelude::*;

use std::mem::MaybeUninit;
use std::str::FromStr;

pub struct Parsedi64s<'a> {
    bytes: &'a [u8],
}

// TODO: Reverse would be nice
impl Iterator for Parsedi64s<'_> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        // Skip non-digits
        let mut i = 0;
        while i < self.bytes.len() && !(self.bytes[i].is_ascii_digit() || self.bytes[i] == b'-') {
            i += 1;
        }
        self.bytes = &self.bytes[i..];

        // Check for a leading '-' because we handle negatives
        let mut negative = false;
        let mut i = 0;
        if i < self.bytes.len() && self.bytes[i] == b'-' {
            negative = true;
            i += 1;
        }

        // Grab digits
        let mut n: i64 = 0;
        while i < self.bytes.len() && self.bytes[i].is_ascii_digit() {
            n *= 10;
            n += (self.bytes[i] - b'0') as i64;

            i += 1;
        }
        if negative {
            n = -n;
        }

        // Sanity check on debug builds
        if cfg!(debug_assertions) && i > 0 {
            let digits_text: &str =
                std::str::from_utf8(&self.bytes[..i]).expect("Failed to parse bytes as UTF8");

            let parsed_n: i64 = digits_text
                    .parse::<i64>()
                    .unwrap_or_else(|e| {
                panic!("Parsing {digits_text:?} to an i64 failed: {e:?}. Release builds will parse as n={n}")
            });
            debug_assert_eq!(
                n, parsed_n,
                "We parsed {digits_text:?} as {n}, but \"should\" have parsed {parsed_n}"
            );
        }
        self.bytes = &self.bytes[i..];

        // If we didn't grab any digits, we're done
        if i > 0 { Some(n) } else { None }
    }
}

pub trait IntParsable {
    fn i64s(&self) -> Parsedi64s<'_>;
}

impl IntParsable for &'_ str {
    fn i64s(&self) -> Parsedi64s<'_> {
        Parsedi64s {
            bytes: self.as_bytes(),
        }
    }
}

impl IntParsable for &'_ [u8] {
    fn i64s(&self) -> Parsedi64s<'_> {
        Parsedi64s { bytes: self }
    }
}

impl IntParsable for str {
    fn i64s(&self) -> Parsedi64s<'_> {
        Parsedi64s {
            bytes: self.as_bytes(),
        }
    }
}

impl IntParsable for [u8] {
    fn i64s(&self) -> Parsedi64s<'_> {
        Parsedi64s { bytes: self }
    }
}

// TODO: Use Pattern when it's stable, https://doc.rust-lang.org/std/str/pattern/index.html?
pub fn parse_list<const N: usize, T>(s: &str, pattern: &str) -> [T; N]
where
    T: FromStr + Copy + Sized,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let ty_name = std::any::type_name::<T>();
    let mut list: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = s.split(pattern).enumerate();
    for (i, t_s) in (&mut iter).take(N) {
        list[i] = match t_s.parse() {
            Ok(t) => MaybeUninit::new(t),
            Err(e) => {
                error!(
                    "While splitting \"{s}\" by \"{pattern}\", failed to parse {i}th elem \"{t_s}\" as {ty_name}: {e:?}"
                );
                unreachable!(
                    "While splitting \"{s}\" by \"{pattern}\", failed to parse {i}th elem \"{t_s}\" as {ty_name}: {e:?}"
                );
            }
        };
    }

    let rem = iter.count();
    if rem != 0 {
        error!(
            str=s,
            pattern=pattern;
                "Trying to parse exactly {N} values of {ty_name}, but found {rem} more!",
        );
        unreachable!("Trying to parse exactly {N} values of {ty_name}, but found {rem} more!");
    }

    unsafe { std::mem::transmute_copy::<_, [T; N]>(&list) }
}

pub fn parse_list_whitespace<const N: usize, T>(s: &str) -> [T; N]
where
    T: FromStr + Copy + Sized,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let ty_name = std::any::type_name::<T>();
    let mut list: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = s.split_whitespace().enumerate();
    for (i, t_s) in (&mut iter).take(N) {
        list[i] = match t_s.parse() {
            Ok(t) => MaybeUninit::new(t),
            Err(e) => {
                error!(
                    "While splitting \"{s}\" by whitespace, failed to parse {i}th elem \"{t_s}\" as {ty_name}: {e:?}"
                );
                unreachable!(
                    "While splitting \"{s}\" by whitespace, failed to parse {i}th elem \"{t_s}\" as {ty_name}: {e:?}"
                );
            }
        };
    }

    let rem = iter.count();
    if rem != 0 {
        error!(
            str=s,
            pattern="whitespace";
                "Trying to parse exactly {N} values of {ty_name}, but found {rem} more!",
        );
        unreachable!("Trying to parse exactly {N} values of {ty_name}, but found {rem} more!");
    }

    unsafe { std::mem::transmute_copy::<_, [T; N]>(&list) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[test]
    fn check_parse_list() {
        {
            let a: [i32; 1] = parse_list("10", ",");
            assert_eq!(a, [10]);
        }

        {
            let a: [i32; 3] = parse_list("10-100000-1", "-");
            assert_eq!(a, [10, 100000, 1]);
        }
    }

    #[rstest]
    #[case::basic("1", [1])]
    #[case::basic("1234", [1234])]
    #[case::basic("1 2 3 4", [1, 2, 3, 4])]
    #[case::basic("1 akl;dfalkdsjflakjsdflajsdlfjalkdf 2 asdjflakjdsfl;kajdslkfja  3 alsdjfla;jdflkaj 4", [1, 2, 3, 4])]
    #[case::basic("1 -2 3 -4", [1, -2, 3, -4])]
    #[trace]
    fn check_i64s(
        #[case] blah: impl IntParsable, // .
        #[case] expected: impl IntoIterator<Item = i64>,
    ) {
        let parsed: Vec<i64> = blah.i64s().collect_vec();
        let expected: Vec<_> = expected.into_iter().collect_vec();

        println!("parsed:   {parsed:?}");
        println!("expected: {expected:?}");

        assert_eq!(parsed, expected);
    }
}
