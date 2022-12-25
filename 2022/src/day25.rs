use crate::prelude::*;

type Text = SmallString<[u8; 32]>;

fn parse_snafu(bs: &[u8]) -> i64 {
    let mut n = 0;

    for b in bs {
        n *= 5;

        match *b {
            b'2' => n += 2,
            b'1' => n += 1,
            b'0' => n += 0,
            b'-' => n -= 1,
            b'=' => n -= 2,
            _ => unreachable!("Unexpected character: {} ({})", *b as char, b),
        }
    }

    n
}

const fn make_snafu_lut() -> [i8; 256] {
    let mut lut = [0; 256];

    lut[b'2' as usize] = 2;
    lut[b'1' as usize] = 1;
    lut[b'0' as usize] = 0;
    lut[b'-' as usize] = -1;
    lut[b'=' as usize] = -2;

    lut
}

const LUT: [i8; 256] = make_snafu_lut();

fn parse_snafu_lut(bs: &[u8]) -> i64 {
    let mut n: i64 = 0;

    for b in bs.iter().copied() {
        n *= 5;
        n += LUT[b as usize] as i64;
    }

    n
}

fn to_snafu(mut n: i64) -> Text {
    let mut buf = [0; 32];
    let mut i = buf.len() - 1;

    while n > 0 {
        buf[i] = b"012=-"[n as usize % 5];
        i -= 1;
        n = (n + 2) / 5;
    }

    if cfg!(debug_assertions) {
        let s = std::str::from_utf8(&buf[i + 1..]).unwrap();
        debug_assert!(
            !s.contains('\0'),
            "snafu string has '\0' but shouldn't:\n\ts =   {s:?}\n\tbuf = {buf:?}\n"
        );
    }

    unsafe { std::str::from_utf8_unchecked(&buf[i + 1..]).into() }
}

// Part1 ========================================================================
#[aoc(day25, part1)]
pub fn part1(input: &str) -> Text {
    let sum: i64 = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(parse_snafu)
        .sum();

    to_snafu(sum)
}

#[aoc(day25, part1, parse_lut)]
pub fn part1_parse_lut(input: &str) -> Text {
    let sum: i64 = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(parse_snafu_lut)
        .sum();

    to_snafu(sum)
}

#[aoc(day25, part1, no_parse)]
pub fn part1_no_parse(input: &str) -> Text {
    let mut digits = [0; 20];
    for line in input.as_bytes().split(|b| *b == b'\n') {
        for (i, b) in line.iter().copied().rev().enumerate() {
            digits[i] += LUT[b as usize] as i64;
        }
    }

    let mut sum = 0;
    let mut n: i64 = 1;
    for d in digits.iter().copied() {
        sum += d * n;
        n *= 5;
    }

    to_snafu(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[rstest]
    #[case::check_1747(1747, "1=-0-2")]
    #[case::check_906(906, "12111")]
    #[case::check_198(198, "2=0=")]
    #[case::check_11(11, "21")]
    #[case::check_201(201, "2=01")]
    #[case::check_31(31, "111")]
    #[case::check_1257(1257, "20012")]
    #[case::check_32(32, "112")]
    #[case::check_353(353, "1=-1=")]
    #[case::check_107(107, "1-12")]
    #[case::check_7(7, "12")]
    #[case::check_3(3, "1=")]
    #[case::check_37(37, "122")]
    #[case::check_1(1, "1")]
    #[case::check_2(2, "2")]
    #[case::check_3(3, "1=")]
    #[case::check_4(4, "1-")]
    #[case::check_5(5, "10")]
    #[case::check_6(6, "11")]
    #[case::check_7(7, "12")]
    #[case::check_8(8, "2=")]
    #[case::check_9(9, "2-")]
    #[case::check_10(10, "20")]
    #[case::check_15(15, "1=0")]
    #[case::check_20(20, "1-0")]
    #[case::check_2022(2022, "1=11-2")]
    #[case::check_12345(12345, "1-0---0")]
    #[case::check_314159265(314159265, "1121-1110-1=0")]
    #[trace]
    fn check_snafu_parse(#[case] num: i64, #[case] snafu: &str) {
        let snafu = snafu.as_bytes();
        assert_eq!(parse_snafu(snafu), num);
    }

    #[rstest]
    #[case::check_1747(1747, "1=-0-2")]
    #[case::check_906(906, "12111")]
    #[case::check_198(198, "2=0=")]
    #[case::check_11(11, "21")]
    #[case::check_201(201, "2=01")]
    #[case::check_31(31, "111")]
    #[case::check_1257(1257, "20012")]
    #[case::check_32(32, "112")]
    #[case::check_353(353, "1=-1=")]
    #[case::check_107(107, "1-12")]
    #[case::check_7(7, "12")]
    #[case::check_3(3, "1=")]
    #[case::check_37(37, "122")]
    #[case::check_1(1, "1")]
    #[case::check_2(2, "2")]
    #[case::check_3(3, "1=")]
    #[case::check_4(4, "1-")]
    #[case::check_5(5, "10")]
    #[case::check_6(6, "11")]
    #[case::check_7(7, "12")]
    #[case::check_8(8, "2=")]
    #[case::check_9(9, "2-")]
    #[case::check_10(10, "20")]
    #[case::check_15(15, "1=0")]
    #[case::check_20(20, "1-0")]
    #[case::check_2022(2022, "1=11-2")]
    #[case::check_12345(12345, "1-0---0")]
    #[case::check_314159265(314159265, "1121-1110-1=0")]
    #[trace]
    fn check_to_snafu(#[case] num: i64, #[case] snafu: &str) {
        assert_eq!(to_snafu(num), snafu);
    }

    #[rstest]
    #[case::given("2=-1=0", EXAMPLE_INPUT)]
    #[case::check_longest_snafu("10=0==00=1==0--=1000", "10=0==00=1==0--=1000")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_parse_lut, part1_no_parse)]
        p: impl FnOnce(&str) -> Text,
        #[case] expected: &str,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
