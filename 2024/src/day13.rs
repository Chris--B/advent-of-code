#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Machine {
    btn_a: [i64; 2],
    btn_b: [i64; 2],
    prize: [i64; 2],
}

impl Machine {
    fn parse(s: &str) -> Self {
        #[rustfmt::skip]
        let (ax, ay, bx, by, px, py) = scan_fmt!(
            s.trim(),
            r#"
                Button A: X+{}, Y+{}
                Button B: X+{}, Y+{}
                Prize: X={}, Y={}
            "#
            .trim(),
            i64, i64, i64, i64, i64, i64
        )
        .unwrap();

        Self {
            btn_a: [ax, ay],
            btn_b: [bx, by],
            prize: [px, py],
        }
    }
}

fn presses_to_win(m: Machine) -> Option<(i64, i64)> {
    /*
        m.prize[0] == A * m.btn_a[0] + B * m.btn_b[0];
        m.prize[1] == A * m.btn_a[1] + B * m.btn_b[1];

        prize              == [A, B]^T * btn_mat
        prize * btn_mat^-1 == [A, B]^T
    */

    #[rustfmt::skip]
    let btn = [
        [m.btn_a[0], m.btn_b[0]],
        [m.btn_a[1], m.btn_b[1]],
    ];
    let [[a, b], [c, d]] = btn;
    let det = a * d - b * c;
    let inv = [[d, -b], [-c, a]];

    if det == 0 {
        unreachable!("Never expect det(btn) == 0.0");
    }

    let press_a = (m.prize[0] * inv[0][0] + m.prize[1] * inv[0][1]) / det;
    let press_b = (m.prize[0] * inv[1][0] + m.prize[1] * inv[1][1]) / det;

    // Either prizes are small (part 1) and inputs must be small
    // ... or prizes are huge (part 2) and we need way more than 100 presses
    if (m.prize[0] >= 1_0000_000_000_000)
        || (0..=100).contains(&press_a) && (0..=100).contains(&press_b)
    {
        let x = m.btn_a[0] * press_a + m.btn_b[0] * press_b;
        let y = m.btn_a[1] * press_a + m.btn_b[1] * press_b;

        // WHY
        if [x, y] == m.prize {
            return Some((press_a, press_b));
        }
    }

    None
}

// Part1 ========================================================================
#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(Machine::parse)
        .filter_map(presses_to_win)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

// Part2 ========================================================================
#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(Machine::parse)
        .map(|mut m| {
            m.prize[0] += 1_0000_000_000_000;
            m.prize[1] += 1_0000_000_000_000;
            m
        })
        .filter_map(presses_to_win)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[rstest]
    #[case::given_1(Some((80, 40)), Machine { btn_a: [94, 34], btn_b: [22, 67], prize: [ 8400,  5400] })]
    #[case::given_2(None,           Machine { btn_a: [26, 66], btn_b: [67, 21], prize: [12748, 12176] })]
    #[case::given_3(Some((38, 86)), Machine { btn_a: [17, 86], btn_b: [84, 37], prize: [ 7870,  6450] })]
    #[case::given_4(None,           Machine { btn_a: [69, 23], btn_b: [27, 71], prize: [18641, 10279] })]
    #[case::round(Some((31, 35)),   Machine { btn_a: [63, 26], btn_b: [41, 75], prize: [ 3388,  3431] })]
    #[case::what_the_01(None, Machine { btn_a: [16, 68], btn_b: [33, 11], prize: [2036, 4852] })]
    #[case::what_the_02(None, Machine { btn_a: [41, 21], btn_b: [41, 67], prize: [3510, 4822] })]
    #[case::what_the_03(None, Machine { btn_a: [56, 11], btn_b: [38, 81], prize: [1058, 2074] })]
    #[case::what_the_04(None, Machine { btn_a: [22, 65], btn_b: [75, 31], prize: [4996, 7118] })]
    #[case::what_the_05(None, Machine { btn_a: [49, 14], btn_b: [19, 33], prize: [2815, 3480] })]
    #[case::what_the_06(None, Machine { btn_a: [15, 63], btn_b: [52, 16], prize: [5098, 7126] })]
    #[case::what_the_07(None, Machine { btn_a: [37, 15], btn_b: [27, 57], prize: [1639, 3029] })]
    #[case::what_the_08(None, Machine { btn_a: [17, 55], btn_b: [51, 24], prize: [ 584,  404] })]
    #[case::what_the_09(None, Machine { btn_a: [31, 16], btn_b: [14, 47], prize: [2191, 1666] })]
    #[case::what_the_10(None, Machine { btn_a: [67, 28], btn_b: [14, 49], prize: [3226, 2755] })]
    #[trace]
    fn check_presses_to_win(
        #[case] expected: Option<(i64, i64)>, //
        #[case] machine: Machine,             //
    ) {
        assert_eq!(presses_to_win(machine), expected);
    }

    #[rstest]
    #[case::given(480, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim_start();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(875318608908, EXAMPLE_INPUT)]
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
