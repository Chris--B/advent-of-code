use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Machine {
    a: [i64; 2],
    b: [i64; 2],
    p: [i64; 2],
}

fn parse(b: &[u8]) -> Machine {
    if cfg!(debug_assertions) {
        println!("  + Parsing bytes {:?}", std::str::from_utf8(b));
        debug_assert!(!b.is_empty());
    }

    let mut nums = [0_i64; 6];
    for (i, n) in b.i64s().enumerate().take(6) {
        nums[i] = n;
    }

    if cfg!(debug_assertions) {
        for (i, n) in nums.iter().enumerate() {
            debug_assert!(
                *n != 0,
                "nums[{i}] == 0: did we parse everything? b={:?}, nums={nums:?}",
                std::str::from_utf8(b)
            );
        }
    }

    Machine {
        a: [nums[0], nums[1]],
        b: [nums[2], nums[3]],
        p: [nums[4], nums[5]],
    }
}

fn tokens<const PART: u8>(Machine { a, b, p }: Machine) -> i64 {
    let det = a[0] * b[1] - b[0] * a[1];
    debug_assert!(det != 0, "Never expect det(btn) == 0");
    if det == 0 {
        return 0;
    }

    let inv = [[b[1], -b[0]], [-a[1], a[0]]];
    let press_a = (p[0] * inv[0][0] + p[1] * inv[0][1]) / det;
    let press_b = (p[0] * inv[1][0] + p[1] * inv[1][1]) / det;

    // Either prizes are small (part 1) and inputs must be small
    // ... or prizes are huge (part 2) and we need way more than 100 presses
    if PART == 1 {
        if !(0..=100).contains(&press_a) && (0..=100).contains(&press_b) {
            return 0;
        }
    }
    debug_assert!(press_a > 0);
    debug_assert!(press_b > 0);

    // Check again (why do we need this?)
    let should_be_p = [
        a[0] * press_a + b[0] * press_b,
        a[1] * press_a + b[1] * press_b,
    ];

    (should_be_p == p) as i64 * (3 * press_a + press_b)
}

// Part1 ========================================================================
#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut start = 0;

    let mut sum = 0;
    for end in memmem::find_iter(bytes, b"\n\n").chain([bytes.len()]) {
        let m = parse(&bytes[start..end]);
        sum += tokens::<1>(m);
        start = end + 2;
    }

    sum
}

// Part2 ========================================================================
#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut start = 0;

    let mut sum = 0;
    for end in memmem::find_iter(bytes, b"\n\n").chain([bytes.len()]) {
        let mut m = parse(&bytes[start..end]);
        m.p[0] += 1_0000_000_000_000;
        m.p[1] += 1_0000_000_000_000;
        sum += tokens::<2>(m);
        start = end + 2;
    }

    sum
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
    #[case::given_1(
        Machine { a: [94, 34], b: [22, 67], p: [8400, 5400] },
        "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X= 8400, Y= 5400")]
    #[case::given_2(
        Machine { a: [26, 66], b: [67, 21], p: [12748, 12176] },
        "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176"
    )]
    #[case::given_3(
        Machine { a: [17, 86], b: [84, 37], p: [ 7870,  6450] },
        "Button A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X= 7870, Y= 6450")]
    #[case::given_4(
        Machine { a: [69, 23], b: [27, 71], p: [18641, 10279] },
        "Button A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279"
    )]
    #[trace]
    fn check_parse(#[case] expected: Machine, #[case] machine: &str) {
        let parsed = parse(machine.as_bytes());
        assert_eq!(parsed, expected);
    }

    #[rustfmt::skip]
    #[rstest]
    #[case::given_01(3*80 + 40, "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X= 8400, Y= 5400")]
    #[case::given_03(3*38 + 86, "Button A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X= 7870, Y= 6450")]
    #[case::round   (3*31 + 35, "Button A: X+63, Y+26\nButton B: X+41, Y+75\nPrize: X= 3388, Y= 3431")]
    #[case::given_02(0, "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176")]
    #[case::given_04(0, "Button A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279")]
    #[case::weird_01(0, "Button A: X+16, Y+68\nButton B: X+33, Y+11\nPrize: X= 2036, Y= 4852")]
    #[case::weird_02(0, "Button A: X+41, Y+21\nButton B: X+41, Y+67\nPrize: X= 3510, Y= 4822")]
    #[case::weird_03(0, "Button A: X+56, Y+11\nButton B: X+38, Y+81\nPrize: X= 1058, Y= 2074")]
    #[case::weird_04(0, "Button A: X+22, Y+65\nButton B: X+75, Y+31\nPrize: X= 4996, Y= 7118")]
    #[case::weird_05(0, "Button A: X+49, Y+14\nButton B: X+19, Y+33\nPrize: X= 2815, Y= 3480")]
    #[case::weird_06(0, "Button A: X+15, Y+63\nButton B: X+52, Y+16\nPrize: X= 5098, Y= 7126")]
    #[case::weird_07(0, "Button A: X+37, Y+15\nButton B: X+27, Y+57\nPrize: X= 1639, Y= 3029")]
    #[case::weird_08(0, "Button A: X+17, Y+55\nButton B: X+51, Y+24\nPrize: X=  584, Y=  404")]
    #[case::weird_09(0, "Button A: X+31, Y+16\nButton B: X+14, Y+47\nPrize: X= 2191, Y= 1666")]
    #[case::weird_10(0, "Button A: X+67, Y+28\nButton B: X+14, Y+49\nPrize: X= 3226, Y= 2755")]
    #[trace]
    fn check_presses_to_win(
        #[case] expected: i64,
        #[case] machine: &str,
    ) {
        assert_eq!(tokens::<1>(parse(machine.as_bytes())), expected);
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
