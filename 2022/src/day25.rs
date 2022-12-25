use crate::prelude::*;

type Text = SmallString<[u8; 32]>;

fn parse_snafu(bs: &[u8]) -> u64 {
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

fn to_snafu(mut n: u64) -> Text {
    let mut s: SmallVec<[u8; 32]> = smallvec![];

    // Parse like a normal base 5 number
    while n > 0 {
        let rem = n % 5;
        n /= 5;
        s.push(rem as u8 + b'0');
    }

    // Pad with a leading 0 so we can index blindly
    s.push(b'0');
    s.reverse();

    // Snafu numbers don't have digits 3, 4, or 5 so map those to = and -, and increment one digit higher
    for i in (1..s.len()).rev() {
        match s[i] {
            b'3' => {
                s[i] = b'=';
                s[i - 1] += 1;
            }
            b'4' => {
                s[i] = b'-';
                s[i - 1] += 1;
            }
            b'5' => {
                s[i] = b'0';
                s[i - 1] += 1;
            }
            b'=' | b'-' | b'0'..=b'2' => {}
            _ => unreachable!("Unexpected digit: {}", s[i]),
        }
    }

    // If that leading 0 is still there, cut it
    if s[0] == b'0' {
        s.remove(0);
    }

    Text::from_str(std::str::from_utf8(&s).unwrap())
}

// Part1 ========================================================================
#[aoc(day25, part1)]
pub fn part1(input: &str) -> Text {
    let sum: u64 = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(parse_snafu)
        .sum();

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
    fn check_snafu_parse(#[case] num: u64, #[case] snafu: &str) {
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
    fn check_to_snafu(#[case] num: u64, #[case] snafu: &str) {
        assert_eq!(to_snafu(num), snafu);
    }

    #[rstest]
    #[case::given("2=-1=0", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> Text,
        #[case] expected: &str,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
