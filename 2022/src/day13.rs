use crate::prelude::*;

use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Foo {
    List(Vec<Foo>),
    Int(i32),
}
use Foo::*;

impl Ord for Foo {
    fn cmp(&self, b: &Foo) -> Ordering {
        self.partial_cmp(b).unwrap()
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, b: &Foo) -> Option<Ordering> {
        match (self, b) {
            // Two ints? Easy
            (Int(x), Int(y)) => x.partial_cmp(y),

            // Mixed? Make it two lists and continue
            (a @ List(_), x @ Int(_)) => a.partial_cmp(&List(vec![x.clone()])),
            (x @ Int(_), a @ List(_)) => List(vec![x.clone()]).partial_cmp(a),

            // List list list
            (List(a), List(b)) => a.partial_cmp(b),
        }
    }
}

// Part1 ========================================================================
#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let pairs = parse(input);

    let mut sum = 0;

    for (i, (a, b)) in pairs.into_iter().enumerate() {
        let i = i + 1; // 1-indexed
        if a <= b {
            sum += i;
        }
    }

    sum as i64
}

// Part2 ========================================================================
#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let pairs = parse(input);

    let d = [
        List(vec![List(vec![Int(2)])]),
        List(vec![List(vec![Int(6)])]),
    ];

    let mut packets = vec![d[0].clone(), d[1].clone()];
    for (a, b) in pairs {
        packets.push(a);
        packets.push(b);
    }
    packets.sort();

    let a = packets.iter().position(|x| *x == d[0]).unwrap() + 1;
    let b = packets.iter().position(|x| *x == d[1]).unwrap() + 1;

    (a * b) as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[rstest]
    #[case::given(13, EXAMPLE_INPUT)]
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

    #[rstest]
    #[case::given(140, EXAMPLE_INPUT)]
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

    #[test]
    fn check_ordering() {
        let pairs = vec![
            (
                // 1
                Ordering::Less,
                List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
                List(vec![Int(1), Int(1), Int(5), Int(1), Int(1)]),
            ),
            (
                // 2
                Ordering::Less,
                List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]),
                List(vec![List(vec![Int(1)]), Int(4)]),
            ),
            (
                // 3
                Ordering::Greater,
                List(vec![Int(9)]),
                List(vec![List(vec![Int(8), Int(7), Int(6)])]),
            ),
            (
                // 4
                Ordering::Less,
                List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)]),
                List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)]),
            ),
            (
                // 5
                Ordering::Greater,
                List(vec![Int(7), Int(7), Int(7), Int(7)]),
                List(vec![Int(7), Int(7), Int(7)]),
            ),
            (
                // 6
                Ordering::Less,
                List(vec![]),
                List(vec![Int(3)]),
            ),
            (
                // 7
                Ordering::Greater,
                List(vec![List(vec![List(vec![])])]),
                List(vec![List(vec![])]),
            ),
            (
                // 8
                Ordering::Less,
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])]),
                        ]),
                    ]),
                    Int(8),
                    Int(9),
                ]),
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])]),
                        ]),
                    ]),
                    Int(8),
                    Int(9),
                ]),
            ),
        ];

        for (i, (expected, a, b)) in pairs.into_iter().enumerate() {
            let i = i + 1;
            assert_eq!(expected, a.cmp(&b), "Failed at Pair {i}");
        }
    }

    #[test]
    fn check_ordering_567_560() {
        let (a, b) = (
            List(vec![Int(5), Int(6), Int(7)]),
            List(vec![Int(5), Int(6), Int(0)]),
        );

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Less));
    }

    #[test]
    fn check_ordering_pair_8_but_sub_range() {
        let (a, b) = (
            List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])]),
            List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])]),
        );

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Less));
    }
}

fn parse(s: &str) -> Vec<(Foo, Foo)> {
    // let mut pairs = vec![];

    // for (a, b, _c) in input.lines().tuples() {
    //     debug_assert_eq!(_c, "");

    //     let a = parse_list(a.trim().as_bytes());
    //     let b = parse_list(b.trim().as_bytes());

    //     pairs.push((a, b));
    // }

    // return pairs;

    if s.lines().count() == 23 {
        vec![
            (
                List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
                List(vec![Int(1), Int(1), Int(5), Int(1), Int(1)]),
            ),
            (
                List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]),
                List(vec![List(vec![Int(1)]), Int(4)]),
            ),
            (
                List(vec![Int(9)]),
                List(vec![List(vec![Int(8), Int(7), Int(6)])]),
            ),
            (
                List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)]),
                List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)]),
            ),
            (
                List(vec![Int(7), Int(7), Int(7), Int(7)]),
                List(vec![Int(7), Int(7), Int(7)]),
            ),
            (List(vec![]), List(vec![Int(3)])),
            (
                List(vec![List(vec![List(vec![])])]),
                List(vec![List(vec![])]),
            ),
            (
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])]),
                        ]),
                    ]),
                    Int(8),
                    Int(9),
                ]),
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])]),
                        ]),
                    ]),
                    Int(8),
                    Int(9),
                ]),
            ),
        ]
    } else if s.lines().count() == 449 {
        vec![
            (
                List(vec![
                    List(vec![Int(5)]),
                    List(vec![Int(1), List(vec![List(vec![Int(0)])])]),
                    List(vec![]),
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(9), Int(1)]),
                            List(vec![Int(3), Int(4), Int(10)]),
                            Int(8),
                            Int(3),
                        ]),
                        Int(6),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(6), Int(8)]), Int(4)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(4), Int(6), Int(0)]),
                            List(vec![Int(10), Int(4), Int(9)]),
                        ]),
                        Int(0),
                        List(vec![Int(3), Int(9)]),
                        List(vec![
                            List(vec![Int(2), Int(2), Int(4)]),
                            Int(7),
                            Int(4),
                            Int(2),
                            Int(8),
                        ]),
                        List(vec![Int(0), Int(8), Int(7), List(vec![Int(9)])]),
                    ]),
                    List(vec![List(vec![List(vec![]), Int(5), Int(7)])]),
                    List(vec![
                        List(vec![Int(10), List(vec![Int(9), Int(6)]), Int(7)]),
                        List(vec![List(vec![Int(5), Int(7)]), Int(4), Int(7)]),
                        Int(5),
                        List(vec![]),
                    ]),
                ]),
                List(vec![List(vec![Int(0), Int(5)])]),
            ),
            (
                List(vec![
                    List(vec![Int(7)]),
                    List(vec![]),
                    List(vec![List(vec![Int(1)])]),
                ]),
                List(vec![
                    List(vec![List(vec![List(vec![Int(9)]), List(vec![Int(1)])])]),
                    List(vec![Int(2)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(2), Int(10), Int(9)]),
                            Int(3),
                            Int(0),
                            Int(6),
                        ]),
                        Int(3),
                    ]),
                    List(vec![
                        List(vec![Int(9), List(vec![Int(9), Int(4), Int(8), Int(9)])]),
                        Int(3),
                    ]),
                    List(vec![
                        Int(9),
                        List(vec![Int(9), List(vec![Int(0), Int(6), Int(10)]), Int(3)]),
                        List(vec![
                            List(vec![Int(8), Int(10), Int(0), Int(7)]),
                            List(vec![Int(2), Int(3), Int(4), Int(2)]),
                            List(vec![Int(2), Int(2), Int(3)]),
                        ]),
                        List(vec![Int(7), Int(6), Int(10)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![]),
                            List(vec![Int(1), Int(3), Int(4)]),
                            Int(8),
                            Int(5),
                            Int(1),
                        ]),
                        Int(1),
                        List(vec![]),
                        Int(3),
                        Int(1),
                    ]),
                    List(vec![List(vec![Int(5)])]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![List(vec![
                        Int(3),
                        Int(9),
                        Int(8),
                        Int(7),
                        List(vec![Int(0), Int(6), Int(8)]),
                    ])]),
                    List(vec![List(vec![List(vec![Int(0), Int(5)])]), Int(3)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![]), List(vec![Int(4), Int(8)])]),
                        List(vec![]),
                        List(vec![List(vec![]), Int(9), Int(8)]),
                    ]),
                    List(vec![Int(0)]),
                    List(vec![
                        List(vec![
                            Int(2),
                            List(vec![Int(1), Int(3), Int(0)]),
                            Int(5),
                            Int(4),
                        ]),
                        Int(0),
                        Int(10),
                        Int(1),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(3), Int(10), Int(6), Int(9)])]),
                        Int(5),
                    ]),
                    List(vec![Int(0), Int(2)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![Int(3)]), Int(6)]),
                    List(vec![
                        List(vec![Int(1), List(vec![Int(6), Int(9), Int(3), Int(4)])]),
                        List(vec![]),
                        List(vec![Int(6)]),
                        Int(2),
                    ]),
                    List(vec![
                        Int(2),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(5)]),
                            Int(4),
                            List(vec![Int(3), Int(9), Int(9)]),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(6), Int(10), Int(5)]),
                            Int(6),
                            Int(0),
                            List(vec![Int(2), Int(6)]),
                            Int(0),
                        ]),
                        Int(8),
                        List(vec![Int(2), Int(9), List(vec![Int(1), Int(0), Int(5)])]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(7),
                            List(vec![Int(0), Int(0), Int(9), Int(0)]),
                            List(vec![Int(0), Int(6), Int(2), Int(6), Int(6)]),
                            List(vec![]),
                        ]),
                        Int(1),
                        List(vec![
                            List(vec![Int(4)]),
                            List(vec![Int(5), Int(2)]),
                            List(vec![Int(2), Int(4), Int(2)]),
                            Int(5),
                        ]),
                        List(vec![List(vec![Int(9)])]),
                        Int(2),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![List(vec![Int(10)]), Int(6), Int(9)]),
                        Int(10),
                    ]),
                    List(vec![Int(7)]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(3),
                    List(vec![
                        List(vec![Int(5), Int(7), Int(0), Int(1), Int(4)]),
                        Int(10),
                        Int(2),
                        List(vec![Int(2), Int(10), Int(7), Int(0)]),
                    ]),
                    List(vec![List(vec![])]),
                ])]),
                List(vec![]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(6),
                        List(vec![]),
                        List(vec![Int(6), List(vec![Int(6), Int(3)])]),
                    ]),
                    List(vec![Int(10)]),
                    List(vec![Int(5), Int(3)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![]), Int(0)]),
                    List(vec![
                        List(vec![
                            Int(4),
                            List(vec![Int(0), Int(2), Int(0)]),
                            Int(4),
                            Int(1),
                        ]),
                        List(vec![List(vec![Int(5), Int(2), Int(1), Int(4)])]),
                        List(vec![Int(10), Int(2), List(vec![Int(5), Int(5), Int(5)])]),
                        List(vec![
                            List(vec![Int(10)]),
                            List(vec![Int(6), Int(6), Int(3), Int(0)]),
                            Int(8),
                            Int(7),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![List(vec![]), Int(5)]),
                        List(vec![
                            List(vec![Int(10), Int(9), Int(5)]),
                            List(vec![Int(8), Int(2), Int(5)]),
                            Int(8),
                            Int(1),
                            Int(4),
                        ]),
                        Int(3),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(7), Int(8), Int(9), Int(0)]),
                            Int(3),
                            List(vec![Int(5)]),
                            List(vec![Int(4), Int(9)]),
                        ]),
                        List(vec![
                            List(vec![Int(3)]),
                            Int(9),
                            List(vec![Int(4), Int(0)]),
                            List(vec![Int(0), Int(9), Int(0)]),
                        ]),
                        List(vec![Int(1), Int(10), Int(9)]),
                        List(vec![Int(1), List(vec![Int(7), Int(8)]), Int(9)]),
                        Int(4),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(7),
                        List(vec![
                            Int(9),
                            List(vec![Int(3), Int(1)]),
                            List(vec![Int(10), Int(6), Int(2), Int(3), Int(1)]),
                        ]),
                    ]),
                    List(vec![List(vec![Int(2), Int(10)]), Int(6)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(9), Int(10)]),
                            List(vec![Int(6), Int(9)]),
                        ]),
                        Int(6),
                        Int(5),
                        List(vec![
                            Int(8),
                            List(vec![Int(0), Int(3)]),
                            Int(0),
                            Int(7),
                            Int(1),
                        ]),
                        List(vec![]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(8),
                    List(vec![
                        List(vec![Int(1), Int(0), Int(4), Int(6), Int(9)]),
                        Int(2),
                    ]),
                ])]),
                List(vec![List(vec![
                    Int(3),
                    List(vec![Int(9), List(vec![]), List(vec![Int(5)])]),
                ])]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![]),
                        List(vec![]),
                        Int(4),
                        List(vec![
                            List(vec![Int(10), Int(9), Int(7)]),
                            List(vec![Int(10), Int(1)]),
                            List(vec![Int(4), Int(8), Int(4), Int(0), Int(7)]),
                            Int(7),
                        ]),
                    ]),
                    List(vec![]),
                ]),
                List(vec![List(vec![Int(6), Int(0), List(vec![]), Int(3)])]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(3),
                        Int(3),
                        List(vec![
                            List(vec![Int(1), Int(9), Int(8)]),
                            Int(4),
                            List(vec![Int(1), Int(4), Int(1)]),
                            List(vec![]),
                            Int(8),
                        ]),
                        List(vec![
                            List(vec![Int(6), Int(1)]),
                            List(vec![Int(2), Int(2), Int(10), Int(3), Int(4)]),
                            List(vec![Int(0), Int(2), Int(4), Int(6), Int(4)]),
                            List(vec![Int(10), Int(7), Int(2)]),
                            List(vec![Int(1)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(0)]),
                        List(vec![List(vec![Int(6), Int(3), Int(3), Int(2), Int(5)])]),
                        Int(3),
                        Int(10),
                        List(vec![Int(10)]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(1), Int(9)]), Int(7), List(vec![])]),
                        List(vec![
                            Int(8),
                            List(vec![Int(6), Int(10)]),
                            List(vec![Int(8), Int(2)]),
                            List(vec![Int(2), Int(5)]),
                        ]),
                        List(vec![List(vec![Int(2)]), Int(0), Int(5), Int(4)]),
                        Int(5),
                        List(vec![Int(3), Int(5), Int(6), Int(3)]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![Int(8), List(vec![Int(6)]), Int(0)]),
                    List(vec![Int(7), Int(10), Int(9), List(vec![Int(1)])]),
                    List(vec![]),
                    List(vec![Int(7), List(vec![List(vec![])]), Int(5), Int(2)]),
                ]),
                List(vec![
                    List(vec![List(vec![])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(3), Int(7), Int(10), Int(9)]),
                            List(vec![Int(2), Int(3), Int(3), Int(1), Int(5)]),
                            List(vec![Int(7), Int(10), Int(0), Int(9), Int(3)]),
                            List(vec![]),
                        ]),
                        List(vec![Int(4), Int(0), Int(5)]),
                        List(vec![List(vec![]), List(vec![]), Int(9), List(vec![Int(0)])]),
                        List(vec![List(vec![Int(3)]), List(vec![Int(10)]), Int(6)]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(10),
                        List(vec![Int(10), Int(7), List(vec![Int(0), Int(10), Int(10)])]),
                        Int(8),
                        Int(8),
                    ]),
                    List(vec![Int(6)]),
                    List(vec![
                        Int(0),
                        Int(2),
                        List(vec![List(vec![]), Int(4)]),
                        List(vec![
                            List(vec![Int(2), Int(9), Int(6), Int(8), Int(5)]),
                            Int(6),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![List(vec![]), List(vec![]), Int(5), Int(3)])]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(7), Int(10)]),
                            List(vec![Int(2), Int(2), Int(3), Int(9), Int(7)]),
                        ]),
                        List(vec![Int(5)]),
                        Int(9),
                        Int(3),
                    ]),
                    List(vec![
                        List(vec![Int(2)]),
                        List(vec![
                            Int(5),
                            List(vec![Int(4), Int(9), Int(8), Int(3), Int(6)]),
                            Int(1),
                        ]),
                        Int(0),
                        List(vec![
                            List(vec![Int(8), Int(6)]),
                            Int(3),
                            List(vec![Int(8), Int(10)]),
                            Int(4),
                        ]),
                        Int(2),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(7),
                        List(vec![Int(6)]),
                        List(vec![List(vec![Int(1)])]),
                        List(vec![
                            Int(4),
                            List(vec![Int(8), Int(0), Int(1), Int(4), Int(8)]),
                            Int(0),
                            List(vec![Int(5)]),
                            Int(9),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        Int(5),
                        List(vec![List(vec![Int(10), Int(10), Int(10), Int(5), Int(4)])]),
                        List(vec![
                            Int(0),
                            List(vec![Int(6), Int(7), Int(4)]),
                            List(vec![Int(2), Int(5), Int(6)]),
                            List(vec![Int(1)]),
                        ]),
                        Int(8),
                    ]),
                ]),
                List(vec![List(vec![Int(1), Int(4), Int(8)])]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(1),
                        Int(0),
                        List(vec![
                            List(vec![Int(10), Int(5), Int(1), Int(9), Int(2)]),
                            List(vec![Int(0), Int(6), Int(6), Int(6)]),
                            List(vec![Int(9), Int(5), Int(8), Int(8)]),
                        ]),
                        Int(4),
                        List(vec![Int(3), Int(4)]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        List(vec![Int(7)]),
                        Int(4),
                        List(vec![
                            List(vec![Int(3), Int(2)]),
                            List(vec![Int(5), Int(5), Int(3), Int(3), Int(6)]),
                        ]),
                        Int(8),
                    ]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![Int(9), Int(3), List(vec![])]),
                    List(vec![
                        List(vec![
                            Int(0),
                            List(vec![Int(8), Int(9), Int(4), Int(4)]),
                            List(vec![Int(7), Int(4), Int(3), Int(7)]),
                        ]),
                        Int(6),
                    ]),
                    List(vec![Int(3), Int(9), Int(1), Int(0)]),
                    List(vec![List(vec![])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(4),
                        List(vec![
                            Int(3),
                            List(vec![Int(10), Int(10), Int(10), Int(8)]),
                            List(vec![Int(4), Int(4), Int(0)]),
                            Int(8),
                            List(vec![Int(9), Int(7), Int(9), Int(4)]),
                        ]),
                        Int(1),
                        Int(3),
                    ]),
                    List(vec![
                        Int(3),
                        Int(10),
                        Int(0),
                        Int(5),
                        List(vec![
                            List(vec![Int(9), Int(6)]),
                            Int(2),
                            Int(3),
                            List(vec![Int(1), Int(7), Int(10)]),
                            Int(4),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(10), Int(10), Int(7), Int(10), Int(1)]),
                            Int(10),
                        ]),
                        Int(9),
                    ]),
                    List(vec![Int(5), Int(6), Int(4)]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(5), Int(4), Int(9), Int(10)]),
                        Int(2),
                        List(vec![
                            List(vec![Int(5)]),
                            List(vec![Int(9), Int(9), Int(9), Int(7)]),
                            List(vec![]),
                            List(vec![Int(7), Int(10), Int(2)]),
                            List(vec![Int(6)]),
                        ]),
                        List(vec![Int(10), Int(3), List(vec![Int(4)])]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(8)]),
                            Int(1),
                            List(vec![]),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(2), Int(5), Int(7)]),
                            Int(9),
                            Int(5),
                            List(vec![Int(2), Int(8), Int(2)]),
                        ]),
                        Int(7),
                    ]),
                    List(vec![
                        Int(7),
                        List(vec![List(vec![]), List(vec![Int(10), Int(1), Int(0)])]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(6),
                    Int(8),
                    List(vec![Int(0), Int(7), Int(9), Int(3), Int(4)]),
                    List(vec![Int(8)]),
                ])]),
                List(vec![
                    List(vec![List(vec![
                        Int(6),
                        List(vec![Int(2)]),
                        List(vec![Int(10), Int(1), Int(2), Int(8), Int(6)]),
                        Int(2),
                        Int(6),
                    ])]),
                    List(vec![
                        List(vec![Int(10), Int(10)]),
                        List(vec![
                            List(vec![Int(0), Int(3), Int(1), Int(6), Int(0)]),
                            Int(10),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![Int(8), Int(9), Int(7), Int(2)]),
                    List(vec![
                        List(vec![Int(6), Int(9), Int(4), Int(2), Int(0)]),
                        Int(7),
                        Int(0),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![])]),
                    List(vec![
                        List(vec![
                            Int(3),
                            Int(6),
                            Int(9),
                            List(vec![Int(2)]),
                            List(vec![Int(7)]),
                        ]),
                        List(vec![
                            List(vec![Int(0), Int(9), Int(2), Int(7)]),
                            List(vec![Int(3)]),
                            Int(2),
                            Int(0),
                            List(vec![Int(9), Int(5), Int(1), Int(8)]),
                        ]),
                        Int(6),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(3),
                        List(vec![List(vec![]), Int(4)]),
                        Int(9),
                        Int(6),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(4)]),
                    List(vec![Int(1), Int(9), Int(4), Int(10)]),
                    List(vec![]),
                    List(vec![List(vec![]), Int(4), Int(5)]),
                    List(vec![List(vec![]), Int(6), Int(4)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(5)])]),
                    List(vec![
                        List(vec![
                            Int(4),
                            List(vec![Int(10), Int(9), Int(3)]),
                            List(vec![Int(1), Int(6), Int(0)]),
                            List(vec![Int(8)]),
                        ]),
                        List(vec![
                            List(vec![Int(7)]),
                            List(vec![Int(1), Int(3), Int(4), Int(6)]),
                            List(vec![Int(5)]),
                            List(vec![Int(1), Int(7), Int(8), Int(8), Int(1)]),
                        ]),
                        List(vec![Int(1), List(vec![]), List(vec![]), Int(8)]),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(5), Int(2)]),
                            Int(2),
                            List(vec![Int(3), Int(6), Int(1), Int(2)]),
                            Int(6),
                        ]),
                    ]),
                ]),
                List(vec![List(vec![List(vec![List(vec![Int(4)])]), Int(3)])]),
            ),
            (
                List(vec![List(vec![
                    Int(7),
                    Int(5),
                    List(vec![]),
                    List(vec![Int(8)]),
                    List(vec![
                        List(vec![Int(3)]),
                        List(vec![Int(8)]),
                        List(vec![Int(2), Int(6), Int(8)]),
                        List(vec![Int(10), Int(4), Int(7)]),
                    ]),
                ])]),
                List(vec![List(vec![Int(1), Int(4), Int(4)])]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(8),
                            List(vec![Int(10), Int(2), Int(8), Int(3), Int(4)]),
                        ]),
                        Int(8),
                    ]),
                    List(vec![]),
                    List(vec![List(vec![Int(3)]), List(vec![]), Int(5)]),
                    List(vec![
                        Int(7),
                        List(vec![List(vec![Int(7)])]),
                        List(vec![
                            List(vec![Int(0), Int(7), Int(6)]),
                            List(vec![Int(4)]),
                            Int(5),
                        ]),
                        List(vec![
                            List(vec![Int(4)]),
                            List(vec![Int(10), Int(4)]),
                            Int(7),
                            Int(6),
                        ]),
                    ]),
                    List(vec![Int(5), Int(8)]),
                ]),
                List(vec![List(vec![Int(0)])]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(2), List(vec![Int(9)]), Int(0)]), Int(6)]),
                    List(vec![Int(6), List(vec![])]),
                ]),
                List(vec![
                    List(vec![
                        Int(7),
                        Int(8),
                        List(vec![Int(8), Int(3)]),
                        Int(1),
                        Int(10),
                    ]),
                    List(vec![Int(10), List(vec![])]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(2), Int(5), Int(7), Int(9)]),
                            List(vec![Int(2), Int(3), Int(7), Int(0), Int(3)]),
                            List(vec![Int(3)]),
                        ]),
                        List(vec![]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![List(vec![Int(4)]), Int(6), Int(4), Int(1)])]),
                List(vec![
                    List(vec![Int(5), Int(3)]),
                    List(vec![
                        Int(2),
                        List(vec![Int(6), Int(9)]),
                        List(vec![List(vec![Int(5), Int(0), Int(2), Int(8)])]),
                        Int(1),
                        List(vec![Int(10), Int(10)]),
                    ]),
                    List(vec![Int(10)]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            Int(10),
                            List(vec![Int(10)]),
                            Int(6),
                            Int(6),
                        ]),
                        Int(5),
                        List(vec![List(vec![Int(0)]), Int(9)]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(2),
                    List(vec![Int(0)]),
                    Int(5),
                    List(vec![Int(6), Int(5), List(vec![Int(2)]), Int(7)]),
                ])]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(8),
                            Int(8),
                            List(vec![Int(8), Int(2), Int(1), Int(4), Int(5)]),
                        ]),
                        Int(6),
                        List(vec![
                            List(vec![Int(10)]),
                            Int(6),
                            List(vec![]),
                            List(vec![]),
                        ]),
                        List(vec![List(vec![Int(9), Int(6)])]),
                        Int(5),
                    ]),
                    List(vec![Int(7), Int(2), List(vec![Int(9)])]),
                    List(vec![Int(3), Int(8), Int(1)]),
                    List(vec![
                        Int(2),
                        List(vec![Int(5), Int(4), Int(4), Int(10)]),
                        Int(10),
                        Int(2),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(6), Int(3)]),
                    List(vec![List(vec![List(vec![Int(6)]), Int(0), Int(1)])]),
                    List(vec![Int(9), Int(10)]),
                    List(vec![Int(10), Int(10), Int(5)]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(1), Int(8), Int(5), Int(10)])]),
                        Int(7),
                        List(vec![]),
                    ]),
                    List(vec![List(vec![])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(1), Int(5), Int(6)]),
                            List(vec![Int(1)]),
                            List(vec![Int(8), Int(9), Int(10), Int(10)]),
                            List(vec![Int(4), Int(1), Int(10)]),
                        ]),
                        List(vec![
                            Int(6),
                            List(vec![Int(6), Int(9), Int(4), Int(3), Int(8)]),
                            List(vec![Int(4), Int(1), Int(0)]),
                        ]),
                        List(vec![List(vec![])]),
                        List(vec![
                            List(vec![Int(10), Int(2), Int(4), Int(8), Int(6)]),
                            Int(1),
                            Int(8),
                            Int(7),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![])]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(5), Int(6), Int(3), Int(7), Int(4)]),
                            List(vec![Int(1), Int(1), Int(8), Int(0)]),
                            List(vec![Int(0), Int(8)]),
                            List(vec![Int(2), Int(2), Int(10), Int(9), Int(6)]),
                        ]),
                        List(vec![
                            List(vec![Int(1), Int(7), Int(6), Int(10), Int(9)]),
                            List(vec![Int(3), Int(3), Int(3)]),
                            Int(6),
                        ]),
                        List(vec![
                            List(vec![Int(2), Int(7)]),
                            Int(6),
                            List(vec![Int(3), Int(8)]),
                        ]),
                    ]),
                    List(vec![Int(2), Int(3), Int(9), Int(5)]),
                    List(vec![]),
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![]),
                            List(vec![]),
                            List(vec![Int(1), Int(5), Int(4)]),
                            Int(7),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(1), Int(7), Int(1), Int(3), Int(4)])]),
                        Int(3),
                        Int(1),
                        List(vec![
                            Int(4),
                            Int(5),
                            Int(8),
                            List(vec![Int(2), Int(10), Int(7), Int(7)]),
                            Int(9),
                        ]),
                    ]),
                    List(vec![
                        Int(6),
                        List(vec![Int(1), List(vec![]), Int(4), Int(4), Int(9)]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(0),
                            List(vec![Int(1), Int(6), Int(0), Int(4)]),
                            Int(5),
                        ]),
                        Int(2),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(10), Int(0), Int(10), Int(9)]),
                            Int(2),
                            Int(7),
                            Int(0),
                            List(vec![Int(4), Int(4), Int(10), Int(10)]),
                        ]),
                        Int(4),
                        Int(2),
                        List(vec![]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(10),
                        Int(3),
                        Int(2),
                        Int(5),
                        List(vec![List(vec![Int(9), Int(6)]), List(vec![Int(9)]), Int(4)]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(9), Int(10)]),
                            List(vec![Int(9), Int(8), Int(7), Int(3), Int(7)]),
                            List(vec![Int(3), Int(7), Int(3)]),
                            Int(7),
                            Int(3),
                        ]),
                        List(vec![Int(8), Int(4)]),
                        Int(7),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(7), Int(4), Int(10)]),
                            List(vec![Int(9)]),
                            List(vec![Int(7), Int(9), Int(8), Int(1)]),
                            Int(4),
                            Int(5),
                        ]),
                        List(vec![
                            Int(9),
                            Int(2),
                            List(vec![Int(2)]),
                            List(vec![Int(10), Int(5)]),
                        ]),
                        Int(8),
                        Int(1),
                    ]),
                    List(vec![List(vec![]), Int(3)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(4),
                        Int(10),
                        Int(5),
                        Int(2),
                        List(vec![
                            Int(7),
                            List(vec![Int(6), Int(2), Int(1), Int(3), Int(3)]),
                        ]),
                    ]),
                    List(vec![Int(3), List(vec![List(vec![Int(3)]), Int(1)]), Int(6)]),
                ]),
                List(vec![
                    List(vec![List(vec![Int(0), Int(9)])]),
                    List(vec![
                        Int(5),
                        List(vec![
                            Int(9),
                            List(vec![Int(1), Int(5), Int(2), Int(7), Int(5)]),
                            Int(10),
                            List(vec![Int(4)]),
                            Int(1),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        Int(2),
                        List(vec![Int(2), Int(2), List(vec![Int(9), Int(4)])]),
                        List(vec![List(vec![Int(0), Int(7), Int(2)])]),
                        Int(5),
                        Int(1),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(8), Int(4), Int(1), Int(8), Int(5)])]),
                        List(vec![
                            Int(4),
                            Int(1),
                            List(vec![]),
                            List(vec![Int(2), Int(6), Int(5)]),
                        ]),
                        Int(5),
                        Int(3),
                    ]),
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(2)]),
                            Int(10),
                            List(vec![Int(8)]),
                            List(vec![Int(10)]),
                        ]),
                        Int(2),
                        Int(0),
                        List(vec![
                            List(vec![Int(6), Int(5)]),
                            List(vec![Int(7), Int(2), Int(8), Int(6)]),
                            List(vec![Int(7)]),
                            Int(8),
                        ]),
                        List(vec![Int(10), List(vec![Int(4), Int(6)]), List(vec![])]),
                    ]),
                    List(vec![List(vec![Int(10)]), Int(9)]),
                    List(vec![]),
                    List(vec![
                        Int(10),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(0), Int(0), Int(4)]),
                            List(vec![Int(9), Int(6), Int(7), Int(7), Int(10)]),
                            Int(2),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(4),
                            Int(10),
                            Int(7),
                            List(vec![Int(5), Int(10)]),
                            Int(8),
                        ]),
                        Int(0),
                        List(vec![]),
                        List(vec![List(vec![Int(1), Int(0)]), List(vec![Int(5)]), Int(9)]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(0),
                        List(vec![
                            Int(6),
                            List(vec![Int(1), Int(0)]),
                            List(vec![]),
                            List(vec![Int(9), Int(5), Int(9)]),
                        ]),
                        List(vec![
                            List(vec![Int(6), Int(10), Int(4), Int(7), Int(7)]),
                            Int(10),
                            List(vec![Int(10), Int(6), Int(1), Int(1), Int(10)]),
                            Int(7),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(8), List(vec![Int(0), Int(8), Int(7)]), Int(2)]),
                    List(vec![
                        Int(1),
                        List(vec![List(vec![Int(5), Int(10)])]),
                        List(vec![
                            List(vec![Int(3), Int(4), Int(0), Int(9), Int(0)]),
                            Int(6),
                            Int(0),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(7), Int(9), Int(0), Int(4)]),
                            Int(6),
                            List(vec![]),
                            List(vec![Int(9)]),
                            List(vec![Int(7)]),
                        ]),
                        List(vec![Int(5)]),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(5), Int(10)]),
                            List(vec![Int(0), Int(9), Int(6), Int(6), Int(6)]),
                            List(vec![Int(1), Int(6), Int(3)]),
                        ]),
                        Int(8),
                        Int(7),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![List(vec![]), List(vec![])])]),
                    List(vec![List(vec![Int(1), Int(10)]), Int(5)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(1)]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(9), Int(1), Int(4), Int(1)]),
                            Int(2),
                            List(vec![Int(1), Int(4), Int(8), Int(7), Int(7)]),
                        ]),
                        Int(2),
                        List(vec![]),
                        List(vec![]),
                        Int(8),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(10)]),
                        Int(9),
                        List(vec![]),
                        List(vec![
                            Int(6),
                            List(vec![]),
                            List(vec![Int(1)]),
                            Int(9),
                            List(vec![Int(4), Int(6), Int(6), Int(6)]),
                        ]),
                        Int(8),
                    ]),
                    List(vec![
                        Int(2),
                        List(vec![List(vec![Int(9), Int(7)])]),
                        List(vec![
                            Int(5),
                            Int(7),
                            Int(2),
                            List(vec![Int(2), Int(7), Int(5), Int(6), Int(8)]),
                            List(vec![Int(6), Int(1), Int(0), Int(1)]),
                        ]),
                        Int(1),
                        Int(5),
                    ]),
                    List(vec![List(vec![Int(4)]), Int(7), Int(0)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![Int(1), Int(5), Int(1)]),
                            Int(6),
                            Int(2),
                            Int(5),
                            List(vec![Int(5), Int(0), Int(3), Int(6), Int(5)]),
                        ]),
                    ]),
                    List(vec![
                        Int(4),
                        Int(3),
                        Int(2),
                        List(vec![
                            Int(4),
                            List(vec![Int(3), Int(1), Int(8), Int(0), Int(9)]),
                            List(vec![Int(6), Int(0), Int(8), Int(7), Int(6)]),
                            Int(5),
                            Int(10),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(9), Int(5), Int(1), Int(7)]),
                            Int(9),
                            List(vec![Int(9)]),
                            Int(0),
                            Int(7),
                        ]),
                        Int(10),
                        List(vec![
                            List(vec![Int(9), Int(10), Int(1), Int(8), Int(7)]),
                            Int(6),
                            Int(1),
                        ]),
                        Int(1),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![
                        List(vec![Int(2), Int(1), Int(10)]),
                        Int(6),
                    ])]),
                    List(vec![]),
                    List(vec![
                        Int(6),
                        List(vec![List(vec![Int(6), Int(7)]), Int(10)]),
                        List(vec![Int(9), Int(10), List(vec![]), Int(9), Int(4)]),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(7), Int(8)]),
                            Int(5),
                            List(vec![Int(6), Int(10), Int(6), Int(8)]),
                        ]),
                        List(vec![]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(8), Int(8), Int(9)]),
                            Int(8),
                            Int(8),
                            Int(1),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(0), Int(5), Int(2)]),
                            Int(2),
                            Int(6),
                        ]),
                        Int(10),
                        List(vec![Int(2), Int(4)]),
                        List(vec![]),
                        Int(7),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![List(vec![]), Int(5), Int(4), Int(9)]),
                        List(vec![Int(2)]),
                        List(vec![
                            List(vec![Int(4), Int(5)]),
                            List(vec![Int(3), Int(2), Int(8), Int(2)]),
                            List(vec![Int(10)]),
                            List(vec![Int(3)]),
                        ]),
                        Int(6),
                    ]),
                    List(vec![Int(2), Int(1), List(vec![])]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(1),
                            List(vec![Int(0), Int(8), Int(7), Int(6)]),
                            Int(8),
                            Int(8),
                            Int(4),
                        ]),
                        Int(10),
                    ]),
                    List(vec![
                        Int(1),
                        List(vec![
                            List(vec![Int(6), Int(4), Int(1)]),
                            List(vec![]),
                            Int(3),
                            List(vec![Int(1), Int(5), Int(5)]),
                            Int(4),
                        ]),
                        Int(2),
                    ]),
                    List(vec![Int(5), Int(2)]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(8)]),
                            List(vec![Int(6), Int(10), Int(3), Int(9)]),
                            List(vec![Int(1)]),
                        ]),
                        List(vec![Int(4)]),
                        List(vec![
                            List(vec![Int(9), Int(9)]),
                            List(vec![Int(10), Int(2), Int(10), Int(6), Int(5)]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(5), Int(5), Int(6)]),
                    List(vec![List(vec![List(vec![Int(2), Int(3)]), Int(9), Int(8)])]),
                    List(vec![List(vec![
                        List(vec![Int(2), Int(6), Int(9)]),
                        Int(9),
                        List(vec![Int(4)]),
                        List(vec![]),
                        Int(2),
                    ])]),
                    List(vec![
                        Int(6),
                        List(vec![
                            List(vec![Int(5), Int(6), Int(6)]),
                            List(vec![Int(10), Int(1), Int(3), Int(5)]),
                            List(vec![Int(1), Int(1), Int(7), Int(9)]),
                        ]),
                        List(vec![Int(6)]),
                        List(vec![
                            List(vec![Int(0), Int(10), Int(0), Int(6)]),
                            List(vec![Int(5), Int(2), Int(4), Int(4), Int(4)]),
                            List(vec![Int(5)]),
                            Int(8),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(1),
                        List(vec![
                            Int(0),
                            Int(0),
                            List(vec![Int(3), Int(7), Int(8), Int(6)]),
                            Int(2),
                        ]),
                        Int(8),
                        List(vec![
                            Int(5),
                            List(vec![Int(10), Int(8), Int(2), Int(3), Int(7)]),
                            List(vec![]),
                            List(vec![]),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![Int(1), List(vec![Int(4), Int(1)])]),
                        List(vec![Int(3), Int(2), List(vec![Int(5), Int(2), Int(7)])]),
                        List(vec![
                            Int(5),
                            List(vec![Int(7), Int(5), Int(1), Int(5)]),
                            Int(7),
                        ]),
                        List(vec![
                            Int(10),
                            List(vec![Int(9), Int(5), Int(1)]),
                            List(vec![Int(2)]),
                            List(vec![Int(3), Int(1), Int(9), Int(7), Int(6)]),
                            Int(10),
                        ]),
                    ]),
                    List(vec![Int(2), List(vec![List(vec![Int(5)])]), Int(0)]),
                    List(vec![Int(8)]),
                    List(vec![List(vec![Int(4)]), List(vec![]), Int(10)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(8), Int(0), Int(8), Int(7)]),
                    List(vec![
                        Int(5),
                        List(vec![Int(4)]),
                        Int(3),
                        Int(6),
                        List(vec![]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![List(vec![Int(9), Int(1)]), List(vec![Int(7)])]),
            ),
            (
                List(vec![
                    List(vec![Int(7), Int(6)]),
                    List(vec![]),
                    List(vec![List(vec![
                        Int(2),
                        List(vec![Int(1), Int(1), Int(9), Int(1), Int(1)]),
                        List(vec![Int(5)]),
                    ])]),
                ]),
                List(vec![List(vec![Int(7)])]),
            ),
            (
                List(vec![
                    List(vec![List(vec![Int(3)])]),
                    List(vec![
                        List(vec![Int(2), List(vec![Int(0), Int(6)])]),
                        Int(3),
                        List(vec![
                            Int(7),
                            List(vec![Int(9), Int(3), Int(2), Int(2), Int(1)]),
                            List(vec![]),
                        ]),
                        List(vec![Int(5), Int(9)]),
                    ]),
                    List(vec![
                        Int(6),
                        List(vec![List(vec![Int(4), Int(4)])]),
                        Int(10),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(0), Int(0)]),
                            Int(7),
                            List(vec![Int(9), Int(9), Int(4)]),
                            Int(4),
                        ]),
                    ]),
                    List(vec![List(vec![
                        List(vec![Int(7), Int(2)]),
                        List(vec![Int(7), Int(9), Int(1), Int(10), Int(9)]),
                        Int(9),
                    ])]),
                    List(vec![
                        List(vec![List(vec![Int(3), Int(3), Int(9), Int(3), Int(10)])]),
                        List(vec![]),
                        List(vec![List(vec![]), Int(3), Int(6)]),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![Int(7), Int(2), Int(9)]), Int(1)]),
                    List(vec![List(vec![Int(3)]), Int(4), Int(8)]),
                    List(vec![
                        List(vec![Int(8), Int(8), Int(5)]),
                        List(vec![Int(10)]),
                        List(vec![]),
                        Int(1),
                        List(vec![Int(10)]),
                    ]),
                    List(vec![
                        Int(10),
                        Int(2),
                        List(vec![Int(8)]),
                        List(vec![
                            List(vec![Int(4), Int(5), Int(4), Int(3), Int(9)]),
                            List(vec![]),
                        ]),
                        List(vec![Int(7), Int(5)]),
                    ]),
                    List(vec![
                        Int(3),
                        Int(6),
                        Int(10),
                        List(vec![List(vec![]), Int(4), Int(1), List(vec![])]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![Int(7), Int(0), List(vec![])])]),
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![Int(2), Int(8)]),
                    List(vec![Int(2), Int(0), Int(2), Int(6), Int(8)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![Int(8), List(vec![Int(7), Int(3), Int(10), Int(0)])]),
                        List(vec![]),
                        Int(8),
                        List(vec![
                            List(vec![Int(9), Int(7), Int(5), Int(9)]),
                            List(vec![Int(2)]),
                            Int(8),
                            Int(7),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(9),
                        List(vec![List(vec![Int(7)])]),
                        Int(0),
                        Int(0),
                        List(vec![Int(4)]),
                    ]),
                    List(vec![Int(10)]),
                ]),
                List(vec![
                    List(vec![Int(2), Int(5), Int(7)]),
                    List(vec![
                        List(vec![
                            Int(1),
                            Int(10),
                            List(vec![]),
                            List(vec![Int(9), Int(6), Int(5), Int(8)]),
                            Int(3),
                        ]),
                        List(vec![]),
                        Int(9),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![
                        Int(8),
                        List(vec![Int(10), Int(6)]),
                        Int(3),
                    ])]),
                    List(vec![Int(2)]),
                    List(vec![
                        Int(6),
                        List(vec![
                            Int(2),
                            List(vec![Int(9)]),
                            Int(3),
                            List(vec![Int(2), Int(1), Int(3), Int(0), Int(0)]),
                            List(vec![Int(0), Int(10), Int(3)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(9), List(vec![Int(4), Int(4)])]),
                        List(vec![]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(5),
                        Int(10),
                        List(vec![
                            List(vec![Int(0), Int(6), Int(6)]),
                            List(vec![]),
                            Int(0),
                            List(vec![Int(3), Int(10), Int(3)]),
                            List(vec![Int(0), Int(0), Int(8), Int(2), Int(9)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(2), Int(10), Int(1)]),
                            Int(3),
                            List(vec![Int(8)]),
                            List(vec![Int(8), Int(1)]),
                            Int(6),
                        ]),
                        Int(4),
                        Int(7),
                    ]),
                    List(vec![
                        Int(6),
                        Int(0),
                        List(vec![
                            Int(7),
                            List(vec![Int(9), Int(0), Int(10), Int(6), Int(7)]),
                        ]),
                        Int(9),
                        Int(10),
                    ]),
                    List(vec![List(vec![Int(5), Int(7)])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(9)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(7), Int(6), Int(0), Int(2)]),
                            List(vec![Int(10), Int(1), Int(7), Int(9), Int(4)]),
                            List(vec![Int(0), Int(1), Int(5), Int(3), Int(10)]),
                            Int(4),
                            Int(6),
                        ]),
                        List(vec![Int(9)]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(1), Int(1)]),
                            Int(7),
                            List(vec![Int(3)]),
                        ]),
                    ]),
                    List(vec![Int(9), List(vec![])]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(9), Int(7), Int(10), Int(1)]),
                            List(vec![Int(9), Int(8), Int(9)]),
                        ]),
                        List(vec![List(vec![Int(4), Int(6)]), List(vec![Int(3)]), Int(5)]),
                        List(vec![
                            Int(8),
                            Int(4),
                            Int(6),
                            Int(2),
                            List(vec![Int(9), Int(10)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(8)]),
                        List(vec![List(vec![Int(2), Int(6), Int(0), Int(0)])]),
                        Int(8),
                        Int(9),
                        Int(4),
                    ]),
                ]),
                List(vec![List(vec![
                    List(vec![Int(5), List(vec![Int(5), Int(6), Int(3)]), Int(10)]),
                    List(vec![
                        Int(1),
                        List(vec![Int(0), Int(9), Int(3), Int(5)]),
                        List(vec![]),
                        List(vec![Int(6), Int(8), Int(5), Int(1), Int(8)]),
                    ]),
                    List(vec![List(vec![Int(6), Int(3), Int(4)]), Int(3)]),
                ])]),
            ),
            (
                List(vec![List(vec![
                    List(vec![Int(10), Int(6)]),
                    Int(3),
                    Int(2),
                    Int(9),
                ])]),
                List(vec![
                    List(vec![List(vec![Int(8), Int(9), List(vec![Int(2)]), Int(4)])]),
                    List(vec![]),
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![Int(9), Int(0), Int(4), Int(4), Int(3)]),
                            List(vec![]),
                            List(vec![Int(5), Int(8)]),
                            Int(8),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(6), Int(10), Int(10)]),
                        List(vec![Int(2), Int(6)]),
                        Int(6),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![Int(5), Int(10), List(vec![Int(3), Int(9), Int(1)])]),
                        List(vec![List(vec![Int(1)])]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(6),
                        Int(0),
                        Int(4),
                        List(vec![
                            List(vec![Int(2), Int(5), Int(5)]),
                            List(vec![Int(10), Int(9), Int(2)]),
                            Int(1),
                            List(vec![Int(7), Int(1), Int(3), Int(8)]),
                        ]),
                    ]),
                    List(vec![Int(6), Int(4)]),
                ]),
                List(vec![
                    List(vec![Int(5)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4)]),
                            List(vec![Int(3)]),
                            List(vec![Int(2), Int(8), Int(0)]),
                            Int(9),
                        ]),
                        Int(9),
                        Int(10),
                        Int(0),
                    ]),
                    List(vec![List(vec![List(vec![])]), Int(0), List(vec![]), Int(8)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(4),
                        List(vec![List(vec![Int(5), Int(3), Int(7), Int(3)]), Int(3)]),
                        Int(6),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(7), Int(4)]),
                            List(vec![]),
                            List(vec![Int(6), Int(0), Int(4)]),
                            List(vec![Int(5), Int(4), Int(8)]),
                        ]),
                        Int(1),
                        List(vec![
                            Int(7),
                            List(vec![Int(3), Int(0)]),
                            Int(5),
                            List(vec![Int(0)]),
                        ]),
                        List(vec![List(vec![Int(1), Int(7), Int(5), Int(4), Int(0)])]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![]), List(vec![Int(0), Int(1), Int(5)])]),
                        Int(10),
                        Int(4),
                        List(vec![Int(0), List(vec![Int(2), Int(7)])]),
                        Int(6),
                    ]),
                    List(vec![Int(5)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(4), Int(0), Int(0)]),
                            Int(8),
                            List(vec![Int(8), Int(2), Int(8)]),
                            List(vec![Int(1), Int(2)]),
                        ]),
                        Int(6),
                        Int(1),
                        Int(0),
                    ]),
                    List(vec![List(vec![])]),
                    List(vec![
                        List(vec![Int(6), List(vec![])]),
                        List(vec![Int(7), List(vec![Int(8), Int(8)]), List(vec![])]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![List(vec![]), Int(2)])]),
                List(vec![
                    List(vec![Int(3)]),
                    List(vec![
                        Int(7),
                        List(vec![]),
                        List(vec![Int(0), Int(3), Int(8), List(vec![Int(8)])]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(4), Int(0), List(vec![])])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(2)]),
                            List(vec![Int(4), Int(8)]),
                            Int(10),
                            List(vec![Int(4), Int(10)]),
                        ]),
                        List(vec![
                            List(vec![Int(0), Int(1), Int(6), Int(6)]),
                            Int(1),
                            Int(4),
                            Int(9),
                            List(vec![Int(0)]),
                        ]),
                        Int(7),
                        List(vec![
                            Int(9),
                            Int(8),
                            List(vec![Int(8), Int(10), Int(5), Int(3), Int(3)]),
                            List(vec![Int(8)]),
                            List(vec![Int(5), Int(2), Int(0)]),
                        ]),
                        List(vec![
                            List(vec![Int(3), Int(5), Int(0)]),
                            List(vec![Int(10), Int(8), Int(10), Int(5)]),
                        ]),
                    ]),
                    List(vec![List(vec![Int(2), Int(2), Int(2), Int(4)])]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(3), Int(10)]),
                            Int(1),
                            Int(7),
                            List(vec![]),
                        ]),
                        Int(8),
                        Int(2),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(4), Int(4)]), Int(8), Int(6)]),
                        Int(0),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![
                            Int(3),
                            Int(5),
                            List(vec![Int(7), Int(9), Int(8), Int(10), Int(9)]),
                            Int(9),
                        ]),
                        Int(5),
                        List(vec![
                            List(vec![]),
                            Int(10),
                            List(vec![Int(3)]),
                            Int(2),
                            List(vec![Int(6), Int(8), Int(6), Int(7)]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(9),
                            Int(8),
                            List(vec![Int(6), Int(0), Int(3), Int(7), Int(7)]),
                        ]),
                        Int(0),
                    ]),
                    List(vec![Int(1), Int(3)]),
                    List(vec![Int(4)]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            Int(8),
                            Int(8),
                            Int(6),
                            List(vec![Int(3)]),
                        ]),
                        Int(2),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(5)]), Int(10)]),
                        List(vec![
                            Int(7),
                            List(vec![Int(0), Int(1), Int(9), Int(6), Int(6)]),
                            List(vec![Int(6)]),
                            Int(10),
                            List(vec![Int(1)]),
                        ]),
                        List(vec![
                            List(vec![Int(9), Int(8), Int(6)]),
                            List(vec![Int(3), Int(9)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![List(vec![])]),
                        List(vec![
                            List(vec![Int(10), Int(2)]),
                            List(vec![Int(9), Int(9), Int(0)]),
                            List(vec![Int(2), Int(8), Int(9), Int(7)]),
                            List(vec![Int(9), Int(1)]),
                        ]),
                        Int(7),
                        List(vec![
                            List(vec![Int(7)]),
                            List(vec![Int(8)]),
                            List(vec![Int(9), Int(2)]),
                            Int(4),
                            Int(0),
                        ]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(10), Int(5), Int(0), List(vec![Int(4)]), Int(10)]),
                    List(vec![Int(1)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(6),
                        Int(3),
                        List(vec![
                            List(vec![Int(10), Int(7), Int(10), Int(3), Int(4)]),
                            Int(3),
                        ]),
                        Int(0),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![])]),
                    List(vec![Int(1), Int(6), Int(6), Int(10), Int(10)]),
                    List(vec![List(vec![
                        Int(1),
                        List(vec![Int(7)]),
                        List(vec![Int(9)]),
                    ])]),
                    List(vec![]),
                    List(vec![
                        List(vec![List(vec![Int(4), Int(8), Int(6)]), List(vec![Int(6)])]),
                        Int(6),
                        List(vec![
                            List(vec![Int(8)]),
                            Int(2),
                            List(vec![Int(5)]),
                            List(vec![Int(8), Int(1), Int(9)]),
                            Int(0),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![Int(3), Int(3), Int(10), Int(8)]),
                    List(vec![Int(3), List(vec![List(vec![Int(7), Int(1), Int(9)])])]),
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![Int(6), Int(6), Int(7), Int(0)]),
                            Int(1),
                            Int(1),
                            List(vec![]),
                        ]),
                        List(vec![
                            Int(4),
                            List(vec![Int(9), Int(4), Int(9), Int(7), Int(9)]),
                        ]),
                        List(vec![Int(6)]),
                    ]),
                    List(vec![Int(7)]),
                ]),
            ),
            (
                List(vec![List(vec![]), List(vec![Int(8), Int(6)])]),
                List(vec![
                    List(vec![Int(6), Int(3)]),
                    List(vec![List(vec![List(vec![Int(10)])])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(8), Int(8), Int(9)]),
                            List(vec![Int(9), Int(0)]),
                            List(vec![Int(10), Int(6)]),
                        ]),
                        List(vec![
                            List(vec![Int(6), Int(5), Int(0)]),
                            Int(5),
                            List(vec![Int(3), Int(8), Int(8)]),
                            List(vec![Int(8), Int(3), Int(1)]),
                        ]),
                        Int(7),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(0)]),
                            Int(9),
                            List(vec![Int(6), Int(0), Int(8)]),
                            Int(4),
                            Int(10),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(10),
                        Int(10),
                        Int(3),
                        List(vec![Int(10)]),
                        List(vec![
                            List(vec![Int(2)]),
                            Int(1),
                            List(vec![Int(3), Int(3), Int(0), Int(2), Int(0)]),
                            Int(2),
                        ]),
                    ]),
                    List(vec![List(vec![
                        Int(10),
                        Int(10),
                        List(vec![Int(6), Int(9)]),
                        List(vec![Int(0), Int(9), Int(7), Int(9), Int(5)]),
                        Int(4),
                    ])]),
                    List(vec![Int(6)]),
                ]),
                List(vec![
                    List(vec![List(vec![
                        Int(8),
                        Int(7),
                        List(vec![Int(0), Int(5), Int(1), Int(7), Int(8)]),
                        Int(6),
                    ])]),
                    List(vec![Int(0), Int(1), Int(5), List(vec![])]),
                    List(vec![
                        Int(6),
                        List(vec![
                            List(vec![Int(8), Int(10), Int(8), Int(8), Int(9)]),
                            Int(8),
                            List(vec![Int(8), Int(6)]),
                            List(vec![Int(3), Int(9), Int(1), Int(7)]),
                            List(vec![]),
                        ]),
                        List(vec![List(vec![Int(10), Int(7)])]),
                        List(vec![
                            Int(9),
                            Int(5),
                            List(vec![Int(0), Int(10), Int(5), Int(3)]),
                            Int(6),
                            List(vec![Int(3), Int(10)]),
                        ]),
                    ]),
                    List(vec![
                        Int(8),
                        List(vec![
                            Int(9),
                            List(vec![]),
                            Int(9),
                            List(vec![Int(2)]),
                            List(vec![Int(5), Int(4)]),
                        ]),
                        List(vec![
                            List(vec![Int(10), Int(7), Int(2), Int(2)]),
                            List(vec![Int(8), Int(1)]),
                            List(vec![Int(10), Int(3)]),
                        ]),
                        Int(9),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![Int(7)]),
                        List(vec![
                            List(vec![Int(7), Int(1)]),
                            List(vec![]),
                            Int(9),
                            Int(9),
                            Int(10),
                        ]),
                        List(vec![]),
                        List(vec![Int(5), List(vec![])]),
                    ]),
                    List(vec![Int(1), Int(5)]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(7),
                            List(vec![Int(7), Int(1)]),
                            List(vec![]),
                            List(vec![Int(9)]),
                            List(vec![Int(5), Int(5), Int(10)]),
                        ]),
                        List(vec![
                            List(vec![Int(0), Int(0), Int(1), Int(5), Int(1)]),
                            Int(8),
                            List(vec![Int(6), Int(10), Int(8), Int(1), Int(10)]),
                            Int(10),
                            List(vec![Int(3), Int(4), Int(0)]),
                        ]),
                        List(vec![Int(8), Int(1), Int(2), Int(1)]),
                        List(vec![]),
                        List(vec![]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(8), Int(0)]),
                            List(vec![Int(7), Int(7), Int(10), Int(10), Int(9)]),
                            List(vec![Int(7), Int(7)]),
                            List(vec![Int(1), Int(5), Int(2), Int(4)]),
                        ]),
                        List(vec![]),
                        Int(9),
                        List(vec![
                            List(vec![Int(3), Int(7), Int(8), Int(10)]),
                            List(vec![Int(10), Int(3)]),
                            List(vec![Int(5), Int(6), Int(7), Int(6), Int(6)]),
                            List(vec![Int(4)]),
                            Int(3),
                        ]),
                        Int(5),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(2), Int(5), Int(2)]),
                            List(vec![]),
                            List(vec![Int(3), Int(5), Int(5), Int(7), Int(9)]),
                            Int(3),
                            List(vec![Int(3)]),
                        ]),
                        List(vec![]),
                        List(vec![
                            Int(1),
                            Int(0),
                            List(vec![Int(9), Int(10), Int(8), Int(1)]),
                            List(vec![Int(9)]),
                        ]),
                        Int(2),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(4), List(vec![]), Int(8)]),
                    List(vec![]),
                    List(vec![Int(3), Int(4), Int(5)]),
                    List(vec![
                        List(vec![]),
                        List(vec![Int(6), List(vec![Int(1), Int(7), Int(6), Int(9)])]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![]), List(vec![])]),
                        List(vec![
                            List(vec![Int(3)]),
                            Int(4),
                            Int(9),
                            List(vec![Int(6), Int(8), Int(10), Int(10), Int(2)]),
                            Int(6),
                        ]),
                        Int(2),
                        List(vec![
                            List(vec![Int(4), Int(3), Int(9), Int(6), Int(5)]),
                            Int(8),
                            List(vec![Int(10), Int(3), Int(3), Int(3), Int(9)]),
                            Int(3),
                        ]),
                        List(vec![Int(8), Int(5), List(vec![]), Int(6)]),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![]),
                        List(vec![Int(6)]),
                        Int(0),
                        List(vec![List(vec![Int(7)]), List(vec![Int(5), Int(7)]), Int(7)]),
                    ]),
                    List(vec![
                        Int(0),
                        List(vec![
                            List(vec![Int(7), Int(8), Int(0), Int(9), Int(4)]),
                            Int(6),
                            Int(9),
                            List(vec![]),
                        ]),
                        List(vec![
                            Int(9),
                            List(vec![Int(7)]),
                            List(vec![Int(0), Int(1), Int(1)]),
                            Int(9),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![Int(5), Int(6), Int(0), Int(4)]),
                List(vec![Int(5), Int(6), Int(0), Int(4), Int(2)]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(0),
                        List(vec![Int(10)]),
                        Int(3),
                        Int(9),
                        List(vec![]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(10),
                            Int(2),
                            List(vec![]),
                            Int(1),
                            List(vec![Int(10), Int(3)]),
                        ]),
                        List(vec![Int(5), Int(0), Int(3)]),
                        Int(8),
                        List(vec![
                            Int(9),
                            List(vec![Int(0)]),
                            List(vec![Int(10), Int(8)]),
                            Int(6),
                            Int(7),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![Int(7)])]),
                    List(vec![Int(9)]),
                    List(vec![Int(0), List(vec![])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(1)]), Int(7), Int(2)]),
                        List(vec![
                            List(vec![Int(5), Int(6), Int(7), Int(1), Int(3)]),
                            List(vec![]),
                            Int(6),
                            Int(1),
                            List(vec![Int(0), Int(3), Int(9), Int(0), Int(9)]),
                        ]),
                        List(vec![List(vec![Int(2), Int(4), Int(4), Int(4)]), Int(8)]),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![]),
                        List(vec![
                            Int(8),
                            List(vec![Int(10), Int(4), Int(6)]),
                            List(vec![Int(0), Int(10)]),
                            List(vec![Int(3), Int(1), Int(2), Int(4), Int(3)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(10)]),
                            Int(6),
                            List(vec![Int(7), Int(5), Int(8), Int(9), Int(1)]),
                            Int(8),
                            List(vec![Int(1)]),
                        ]),
                        List(vec![]),
                        Int(7),
                        List(vec![List(vec![])]),
                    ]),
                    List(vec![List(vec![]), List(vec![]), List(vec![]), Int(8)]),
                    List(vec![
                        List(vec![
                            Int(1),
                            List(vec![Int(6)]),
                            List(vec![Int(8), Int(7), Int(4)]),
                            Int(3),
                        ]),
                        Int(2),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(10),
                        Int(7),
                        List(vec![
                            Int(10),
                            Int(0),
                            List(vec![Int(7), Int(5), Int(6), Int(0)]),
                            Int(3),
                            List(vec![Int(0)]),
                        ]),
                        Int(0),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(8),
                        List(vec![
                            List(vec![Int(5), Int(3), Int(3), Int(10)]),
                            List(vec![Int(8), Int(9), Int(4), Int(1)]),
                            List(vec![Int(10), Int(8), Int(8), Int(3), Int(2)]),
                            List(vec![Int(10), Int(0), Int(10)]),
                            List(vec![Int(0), Int(7), Int(2)]),
                        ]),
                        List(vec![
                            Int(10),
                            List(vec![Int(5), Int(5), Int(10), Int(1), Int(4)]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(3), Int(2), List(vec![]), Int(8)]),
                    List(vec![
                        List(vec![
                            Int(9),
                            List(vec![Int(9), Int(7)]),
                            List(vec![Int(1), Int(3), Int(2)]),
                        ]),
                        Int(7),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(10)]), Int(6), Int(1), List(vec![])]),
                    List(vec![List(vec![List(vec![Int(7)])])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(0),
                        Int(2),
                        List(vec![
                            Int(10),
                            List(vec![Int(1), Int(8), Int(6)]),
                            List(vec![]),
                            List(vec![Int(8)]),
                        ]),
                        Int(2),
                    ]),
                    List(vec![Int(1)]),
                    List(vec![]),
                    List(vec![
                        List(vec![List(vec![])]),
                        List(vec![List(vec![Int(9), Int(7), Int(4)])]),
                        Int(10),
                        Int(0),
                        Int(10),
                    ]),
                ]),
                List(vec![
                    List(vec![List(vec![
                        List(vec![Int(3), Int(6), Int(5), Int(1), Int(10)]),
                        List(vec![Int(6)]),
                    ])]),
                    List(vec![
                        List(vec![List(vec![Int(3)]), Int(7), Int(4), Int(9), Int(7)]),
                        List(vec![]),
                        Int(2),
                        Int(6),
                    ]),
                    List(vec![
                        Int(5),
                        Int(4),
                        List(vec![
                            List(vec![Int(9), Int(8), Int(4), Int(1)]),
                            Int(0),
                            Int(5),
                            List(vec![Int(10), Int(6), Int(2), Int(6), Int(7)]),
                            Int(3),
                        ]),
                        List(vec![
                            Int(7),
                            List(vec![Int(1), Int(9), Int(7), Int(10)]),
                            List(vec![Int(10), Int(4)]),
                            Int(1),
                            List(vec![Int(4), Int(10), Int(2)]),
                        ]),
                        Int(0),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(4),
                        Int(0),
                        Int(5),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(8), Int(9), Int(6), Int(9), Int(4)]),
                        ]),
                    ]),
                    List(vec![
                        Int(4),
                        List(vec![
                            Int(7),
                            List(vec![Int(8), Int(2), Int(4), Int(10), Int(8)]),
                            Int(1),
                        ]),
                        Int(2),
                        Int(4),
                        Int(8),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(7),
                        List(vec![
                            Int(9),
                            Int(5),
                            List(vec![Int(5), Int(3), Int(4), Int(2)]),
                            List(vec![]),
                            Int(5),
                        ]),
                    ]),
                    List(vec![Int(7), Int(5)]),
                ]),
                List(vec![List(vec![
                    Int(2),
                    List(vec![
                        List(vec![Int(6), Int(6), Int(2), Int(4)]),
                        List(vec![]),
                        List(vec![Int(1)]),
                        Int(9),
                    ]),
                ])]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(9), Int(2), Int(8), Int(8)]),
                            List(vec![Int(6), Int(4), Int(3), Int(9), Int(0)]),
                        ]),
                        List(vec![Int(8), Int(5), Int(5)]),
                        Int(0),
                        List(vec![
                            Int(8),
                            List(vec![Int(10), Int(5), Int(5), Int(2)]),
                            List(vec![Int(7), Int(0), Int(1), Int(3), Int(5)]),
                            List(vec![Int(2), Int(2), Int(4), Int(5), Int(10)]),
                        ]),
                    ]),
                    List(vec![
                        Int(5),
                        Int(6),
                        List(vec![Int(2), Int(10), Int(1), List(vec![Int(4)])]),
                        Int(10),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(8),
                        Int(6),
                        List(vec![Int(7), Int(5)]),
                        List(vec![List(vec![Int(3), Int(1)]), Int(9)]),
                    ]),
                    List(vec![
                        List(vec![Int(10), Int(8)]),
                        List(vec![]),
                        List(vec![]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(7),
                            List(vec![Int(5), Int(8), Int(10), Int(1)]),
                            Int(8),
                            Int(3),
                        ]),
                        List(vec![List(vec![Int(2), Int(5)])]),
                        List(vec![
                            List(vec![Int(9), Int(9), Int(10), Int(5)]),
                            List(vec![Int(2), Int(1)]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        Int(1),
                        Int(4),
                        Int(5),
                        List(vec![Int(9)]),
                        List(vec![Int(7), Int(0)]),
                    ]),
                    Int(2),
                    Int(1),
                    List(vec![
                        List(vec![Int(5), Int(2)]),
                        List(vec![Int(5), Int(6)]),
                        List(vec![]),
                    ]),
                ])]),
                List(vec![
                    List(vec![List(vec![Int(9)])]),
                    List(vec![Int(0), Int(2), List(vec![Int(0), List(vec![Int(9)])])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(2), Int(4)]),
                    List(vec![
                        List(vec![List(vec![Int(4)]), List(vec![]), Int(7), Int(2)]),
                        Int(2),
                    ]),
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(4), Int(6), Int(7), Int(10)]),
                            Int(1),
                            List(vec![Int(10), Int(3), Int(8), Int(3)]),
                            Int(3),
                        ]),
                        Int(10),
                        List(vec![Int(7), Int(2), List(vec![Int(1)]), Int(7)]),
                    ]),
                    List(vec![Int(7), Int(1)]),
                    List(vec![Int(3), Int(5), Int(8), List(vec![Int(10)]), Int(10)]),
                ]),
                List(vec![
                    List(vec![List(vec![])]),
                    List(vec![
                        Int(8),
                        Int(5),
                        List(vec![
                            Int(1),
                            List(vec![Int(2), Int(6)]),
                            Int(4),
                            Int(8),
                            List(vec![Int(5), Int(3), Int(7), Int(3)]),
                        ]),
                    ]),
                    List(vec![List(vec![
                        Int(7),
                        List(vec![Int(8), Int(4)]),
                        List(vec![]),
                    ])]),
                    List(vec![Int(0)]),
                    List(vec![
                        List(vec![
                            Int(8),
                            List(vec![Int(10), Int(1), Int(6), Int(3), Int(3)]),
                            List(vec![Int(10), Int(9)]),
                        ]),
                        List(vec![Int(2), List(vec![]), Int(9), Int(10)]),
                        Int(2),
                        Int(5),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(8),
                    List(vec![
                        Int(1),
                        Int(10),
                        List(vec![Int(4), Int(10), Int(10)]),
                        List(vec![Int(1), Int(7), Int(0), Int(1), Int(3)]),
                        Int(8),
                    ]),
                    List(vec![
                        List(vec![Int(8)]),
                        List(vec![Int(9), Int(0)]),
                        Int(10),
                        Int(9),
                        Int(1),
                    ]),
                    List(vec![
                        List(vec![Int(9), Int(5), Int(1), Int(3)]),
                        List(vec![Int(7), Int(9), Int(2), Int(6), Int(5)]),
                    ]),
                    Int(4),
                ])]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(9),
                        List(vec![
                            Int(0),
                            List(vec![Int(8), Int(2), Int(8), Int(1), Int(3)]),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(2), Int(8), Int(3), Int(10), Int(2)])]),
                        Int(9),
                        List(vec![Int(5)]),
                    ]),
                    List(vec![]),
                    List(vec![List(vec![])]),
                ]),
            ),
            (
                List(vec![List(vec![Int(3), Int(1), Int(0)])]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(0),
                            List(vec![Int(6), Int(8)]),
                            List(vec![Int(2)]),
                            List(vec![]),
                        ]),
                        Int(7),
                        List(vec![List(vec![Int(0), Int(1), Int(5), Int(0)])]),
                        List(vec![List(vec![Int(1), Int(2), Int(3), Int(8)])]),
                        List(vec![Int(5), Int(5), Int(8)]),
                    ]),
                    List(vec![Int(5)]),
                    List(vec![List(vec![
                        List(vec![Int(6), Int(3), Int(5)]),
                        Int(0),
                        List(vec![Int(2), Int(1), Int(5), Int(10)]),
                        List(vec![Int(5), Int(10), Int(6), Int(5), Int(2)]),
                    ])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(2), Int(0), Int(5)]),
                            List(vec![]),
                        ]),
                        Int(7),
                        Int(4),
                        List(vec![Int(6), Int(8), Int(2)]),
                        Int(3),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![]), Int(3), Int(9), Int(0)]),
                    List(vec![List(vec![
                        Int(4),
                        Int(8),
                        List(vec![Int(5), Int(10)]),
                        List(vec![Int(9), Int(6), Int(6)]),
                        Int(0),
                    ])]),
                ]),
                List(vec![
                    List(vec![
                        Int(3),
                        List(vec![List(vec![Int(1), Int(2), Int(3)]), Int(0)]),
                        List(vec![List(vec![Int(4), Int(9), Int(2)])]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![Int(7), Int(4)]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![Int(4)]), Int(10)]),
                    List(vec![
                        Int(7),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(5), Int(1), Int(2)]),
                            Int(7),
                        ]),
                        Int(5),
                        List(vec![Int(9), Int(3)]),
                    ]),
                    List(vec![Int(8), List(vec![]), List(vec![])]),
                    List(vec![Int(10), Int(6), Int(5), Int(3), Int(2)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(2), Int(1), Int(9)]),
                            List(vec![Int(1), Int(5), Int(4), Int(6)]),
                            List(vec![Int(4), Int(4)]),
                            Int(8),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        Int(7),
                        List(vec![
                            Int(0),
                            List(vec![Int(9), Int(8)]),
                            Int(2),
                            List(vec![]),
                            Int(8),
                        ]),
                        Int(9),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(4), Int(2), Int(7)]),
                    List(vec![]),
                    List(vec![
                        Int(6),
                        Int(3),
                        List(vec![List(vec![])]),
                        List(vec![]),
                        List(vec![
                            Int(4),
                            Int(5),
                            List(vec![Int(9), Int(9), Int(1)]),
                            List(vec![Int(4), Int(3), Int(9)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(1)]),
                            List(vec![Int(7), Int(7)]),
                            Int(3),
                            List(vec![Int(4), Int(6), Int(3), Int(0)]),
                        ]),
                        List(vec![Int(4), Int(1)]),
                        List(vec![]),
                    ]),
                    List(vec![Int(2)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(6), Int(6), Int(5)]),
                            List(vec![Int(8), Int(4)]),
                            Int(10),
                            List(vec![Int(2), Int(6), Int(9), Int(2), Int(10)]),
                        ]),
                        Int(0),
                        List(vec![
                            List(vec![Int(2), Int(7), Int(0), Int(2), Int(6)]),
                            Int(0),
                            List(vec![]),
                        ]),
                        Int(2),
                    ]),
                    List(vec![List(vec![List(vec![])])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(2),
                        List(vec![Int(10)]),
                        List(vec![
                            List(vec![Int(0), Int(6)]),
                            List(vec![Int(9)]),
                            List(vec![Int(8), Int(7)]),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![List(vec![Int(5)])]),
                    List(vec![Int(5)]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![List(vec![Int(7), List(vec![Int(8), Int(9)])])]),
                    List(vec![Int(8)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(5), Int(10)]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(10), Int(7)]),
                        List(vec![
                            List(vec![Int(1)]),
                            Int(8),
                            List(vec![Int(6), Int(8), Int(2), Int(8)]),
                            Int(8),
                        ]),
                        List(vec![Int(5), List(vec![])]),
                        Int(0),
                        List(vec![]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(9),
                            Int(5),
                            Int(9),
                            List(vec![Int(10), Int(7)]),
                            Int(8),
                        ]),
                        List(vec![
                            Int(3),
                            Int(8),
                            List(vec![Int(1), Int(10), Int(10)]),
                            Int(3),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(0),
                            Int(7),
                            List(vec![Int(7), Int(7), Int(9), Int(6)]),
                            Int(5),
                            Int(8),
                        ]),
                        Int(9),
                    ]),
                    List(vec![]),
                    List(vec![Int(7), List(vec![]), Int(0)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        Int(5),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(5), Int(7)]),
                            Int(3),
                            Int(8),
                            List(vec![Int(0)]),
                        ]),
                        Int(8),
                    ]),
                    List(vec![Int(1)]),
                    List(vec![
                        List(vec![Int(5), List(vec![Int(5)])]),
                        List(vec![
                            List(vec![Int(0), Int(0), Int(8)]),
                            Int(8),
                            List(vec![Int(4)]),
                            List(vec![Int(9), Int(10)]),
                            Int(2),
                        ]),
                        List(vec![Int(4), Int(3)]),
                        List(vec![List(vec![])]),
                        List(vec![Int(10)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(0), Int(10)]),
                    List(vec![List(vec![List(vec![Int(2)])]), Int(5), Int(3)]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(3),
                            Int(3),
                            List(vec![Int(10)]),
                            List(vec![Int(2), Int(4), Int(4), Int(6)]),
                            List(vec![Int(6)]),
                        ]),
                        Int(3),
                    ]),
                    List(vec![List(vec![Int(8), List(vec![Int(4)])])]),
                    List(vec![
                        Int(4),
                        List(vec![
                            Int(5),
                            List(vec![Int(7), Int(4), Int(5), Int(4), Int(1)]),
                            Int(5),
                        ]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(3), Int(9), Int(5)]),
                            Int(8),
                        ]),
                        List(vec![]),
                        List(vec![Int(10), List(vec![Int(4), Int(10), Int(1)]), Int(4)]),
                    ]),
                    List(vec![List(vec![Int(8), Int(5)]), Int(4)]),
                    List(vec![Int(2), List(vec![Int(3)])]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(1),
                    List(vec![
                        List(vec![]),
                        Int(0),
                        Int(3),
                        List(vec![]),
                        List(vec![Int(6), Int(10)]),
                    ]),
                    List(vec![
                        Int(9),
                        List(vec![Int(10), Int(6), Int(5), Int(8)]),
                        List(vec![Int(0), Int(10), Int(2)]),
                        Int(4),
                    ]),
                    List(vec![Int(4), List(vec![]), Int(0), Int(4), Int(3)]),
                    List(vec![Int(7), List(vec![Int(2), Int(4), Int(8)])]),
                ])]),
                List(vec![
                    List(vec![Int(5), Int(8)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(4), Int(5)]),
                            List(vec![Int(3)]),
                            Int(9),
                            List(vec![Int(1), Int(7), Int(0), Int(3), Int(10)]),
                        ]),
                        Int(2),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![]), Int(4)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(8), Int(3), Int(3), Int(10)]),
                            List(vec![Int(6), Int(8)]),
                            List(vec![Int(2), Int(0), Int(7)]),
                            List(vec![]),
                            List(vec![Int(9), Int(3), Int(3), Int(1)]),
                        ]),
                        List(vec![]),
                        List(vec![List(vec![Int(10), Int(1), Int(3)])]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(5), Int(0)]),
                            Int(8),
                            List(vec![Int(4), Int(9), Int(3), Int(0)]),
                        ]),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(0), Int(2), Int(8), Int(2), Int(0)]),
                            Int(8),
                            Int(5),
                        ]),
                        List(vec![
                            List(vec![Int(2)]),
                            Int(8),
                            List(vec![]),
                            List(vec![Int(0), Int(9)]),
                            List(vec![Int(0), Int(0), Int(5)]),
                        ]),
                        List(vec![Int(2), Int(2), List(vec![Int(4)])]),
                    ]),
                    List(vec![List(vec![
                        List(vec![Int(6), Int(6), Int(5), Int(4)]),
                        List(vec![Int(8), Int(6), Int(0)]),
                        Int(4),
                        Int(8),
                    ])]),
                ]),
                List(vec![
                    List(vec![Int(1)]),
                    List(vec![
                        Int(10),
                        Int(9),
                        List(vec![
                            List(vec![Int(1)]),
                            Int(0),
                            List(vec![Int(10), Int(0), Int(3), Int(2), Int(6)]),
                        ]),
                        Int(3),
                        Int(10),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(9), Int(3), Int(8), Int(4)]),
                            List(vec![Int(6), Int(2), Int(8), Int(0)]),
                        ]),
                        Int(2),
                        List(vec![List(vec![Int(2), Int(4), Int(10), Int(6), Int(5)])]),
                        Int(1),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(9),
                        List(vec![
                            Int(0),
                            List(vec![Int(7), Int(0), Int(2)]),
                            List(vec![]),
                            Int(6),
                        ]),
                        List(vec![
                            List(vec![Int(2), Int(7)]),
                            Int(0),
                            List(vec![Int(10), Int(3), Int(10)]),
                            Int(4),
                            Int(4),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            List(vec![Int(7), Int(6), Int(1)]),
                            Int(5),
                        ]),
                        Int(6),
                        Int(4),
                    ]),
                    List(vec![Int(2)]),
                    List(vec![Int(6), Int(7), Int(10)]),
                    List(vec![
                        Int(8),
                        List(vec![
                            List(vec![Int(3), Int(6), Int(1), Int(6)]),
                            Int(0),
                            Int(5),
                            Int(5),
                        ]),
                        Int(9),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(7), Int(8), List(vec![Int(10), Int(3), Int(3)])]),
                        List(vec![List(vec![Int(9), Int(7), Int(4)])]),
                    ]),
                    List(vec![List(vec![Int(0), Int(7), List(vec![])]), Int(6)]),
                    List(vec![Int(2)]),
                    List(vec![]),
                    List(vec![Int(4)]),
                ]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        List(vec![Int(1), Int(1)]),
                        List(vec![Int(5)]),
                        List(vec![Int(7), Int(9), Int(8)]),
                        List(vec![Int(7), Int(4), Int(10), Int(0), Int(4)]),
                    ]),
                    List(vec![
                        List(vec![Int(4)]),
                        List(vec![Int(4), Int(9), Int(5)]),
                        Int(5),
                    ]),
                ])]),
                List(vec![List(vec![
                    List(vec![]),
                    List(vec![
                        Int(1),
                        Int(9),
                        List(vec![Int(7)]),
                        Int(10),
                        List(vec![Int(4)]),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![Int(0), Int(2)]),
                        Int(5),
                        List(vec![Int(0)]),
                        Int(4),
                    ]),
                    Int(8),
                    Int(8),
                ])]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(10),
                            List(vec![Int(5), Int(5), Int(8), Int(2)]),
                            List(vec![]),
                            List(vec![]),
                        ]),
                        List(vec![
                            Int(0),
                            Int(5),
                            List(vec![Int(0), Int(6), Int(6), Int(1), Int(1)]),
                            List(vec![Int(8)]),
                            Int(0),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![Int(5), Int(5), Int(1)]),
                    List(vec![
                        List(vec![
                            Int(3),
                            List(vec![Int(6), Int(2), Int(0)]),
                            List(vec![]),
                            Int(8),
                        ]),
                        List(vec![List(vec![Int(9), Int(3), Int(4), Int(5), Int(6)])]),
                        List(vec![
                            List(vec![Int(10), Int(4), Int(9)]),
                            List(vec![Int(9), Int(1)]),
                            Int(2),
                            List(vec![Int(0)]),
                            Int(10),
                        ]),
                        List(vec![
                            List(vec![Int(3)]),
                            List(vec![Int(5), Int(10)]),
                            List(vec![Int(10), Int(0), Int(7)]),
                            List(vec![Int(5), Int(8)]),
                            Int(10),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            List(vec![Int(8)]),
                            List(vec![Int(10), Int(6), Int(0)]),
                        ]),
                        Int(2),
                        Int(10),
                        Int(2),
                        List(vec![Int(3), List(vec![Int(6), Int(5), Int(4)]), Int(1)]),
                    ]),
                    List(vec![
                        Int(5),
                        List(vec![
                            List(vec![Int(6)]),
                            List(vec![Int(5), Int(4), Int(0)]),
                            List(vec![Int(7), Int(0)]),
                        ]),
                        List(vec![
                            List(vec![]),
                            Int(7),
                            List(vec![Int(8), Int(3), Int(4), Int(8)]),
                            List(vec![Int(1), Int(3)]),
                            Int(3),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(8), Int(0), Int(2)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(7), Int(10), Int(6), Int(3)]),
                            List(vec![Int(7), Int(7)]),
                        ]),
                        Int(2),
                        Int(1),
                        Int(8),
                        Int(7),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(0),
                        List(vec![]),
                        List(vec![List(vec![Int(2)]), List(vec![])]),
                        List(vec![List(vec![Int(3), Int(3), Int(5)]), Int(1), Int(1)]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(3), Int(9)]),
                            List(vec![Int(3), Int(7), Int(8), Int(4)]),
                            Int(1),
                        ]),
                        List(vec![Int(7)]),
                        Int(3),
                        Int(2),
                        Int(3),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(10),
                        Int(0),
                        Int(3),
                        List(vec![Int(6), Int(9)]),
                        List(vec![Int(3)]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(6), Int(2), Int(1), Int(7)]),
                            List(vec![Int(7), Int(10), Int(1), Int(8), Int(4)]),
                        ]),
                        List(vec![
                            List(vec![]),
                            List(vec![]),
                            List(vec![Int(7), Int(10)]),
                            Int(9),
                            Int(4),
                        ]),
                    ]),
                    List(vec![List(vec![List(vec![])])]),
                    List(vec![Int(10), Int(3), List(vec![]), Int(7), Int(3)]),
                    List(vec![
                        List(vec![]),
                        List(vec![List(vec![Int(2), Int(8)])]),
                        List(vec![Int(0), Int(3), Int(8), Int(1), Int(5)]),
                        List(vec![List(vec![Int(4), Int(1), Int(7), Int(7)])]),
                        Int(0),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(6),
                    List(vec![Int(4), Int(5)]),
                    List(vec![
                        List(vec![Int(7), Int(0), Int(9), Int(10)]),
                        Int(0),
                        Int(5),
                        Int(1),
                        Int(9),
                    ]),
                    Int(9),
                    List(vec![Int(6), List(vec![Int(10), Int(10), Int(8)])]),
                ])]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(2),
                            Int(0),
                            Int(3),
                            List(vec![Int(7), Int(2), Int(10), Int(8)]),
                            Int(0),
                        ]),
                        Int(5),
                        List(vec![
                            List(vec![Int(0)]),
                            List(vec![Int(3)]),
                            List(vec![Int(7), Int(4), Int(10)]),
                        ]),
                    ]),
                    List(vec![Int(1)]),
                    List(vec![List(vec![
                        Int(8),
                        Int(1),
                        List(vec![Int(7), Int(10), Int(10), Int(8)]),
                    ])]),
                    List(vec![List(vec![Int(7), List(vec![]), Int(2), Int(2)])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(9),
                        List(vec![Int(1), Int(2), List(vec![Int(7), Int(0), Int(9)])]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(9),
                            Int(10),
                            List(vec![Int(4), Int(3), Int(10)]),
                            Int(0),
                            List(vec![Int(2)]),
                        ]),
                        Int(2),
                        Int(6),
                        List(vec![Int(10), Int(0), Int(8)]),
                        Int(5),
                    ]),
                    List(vec![
                        List(vec![Int(10), List(vec![]), Int(3), List(vec![])]),
                        Int(4),
                        List(vec![]),
                        Int(3),
                        List(vec![Int(10), Int(0), Int(8), Int(9)]),
                    ]),
                    List(vec![List(vec![
                        Int(9),
                        Int(7),
                        Int(0),
                        List(vec![Int(9)]),
                        Int(4),
                    ])]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(9), Int(2), Int(1), Int(10), Int(0)])]),
                        Int(6),
                        List(vec![List(vec![Int(3)]), Int(2)]),
                        List(vec![Int(8), Int(9)]),
                    ]),
                    List(vec![Int(6), Int(10), Int(3)]),
                    List(vec![List(vec![List(vec![])]), Int(1)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(5),
                        Int(10),
                        List(vec![
                            List(vec![]),
                            Int(1),
                            List(vec![]),
                            Int(10),
                            List(vec![Int(6), Int(1)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(10), Int(5)]),
                        List(vec![List(vec![])]),
                        Int(0),
                        Int(10),
                    ]),
                    List(vec![Int(9)]),
                    List(vec![List(vec![Int(5)])]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(6),
                            List(vec![Int(0), Int(2), Int(1), Int(4), Int(5)]),
                        ]),
                        Int(3),
                    ]),
                    List(vec![
                        List(vec![Int(3)]),
                        List(vec![
                            List(vec![Int(1)]),
                            List(vec![Int(9), Int(3), Int(1), Int(1), Int(4)]),
                            List(vec![Int(10), Int(7), Int(4)]),
                        ]),
                        Int(7),
                        Int(2),
                    ]),
                    List(vec![]),
                    List(vec![Int(8), List(vec![List(vec![Int(3)])])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(8)]), Int(9)]),
                        List(vec![Int(2)]),
                        Int(0),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(8)]), Int(9), Int(8), Int(2)]),
                        Int(4),
                    ]),
                    List(vec![
                        Int(9),
                        Int(3),
                        List(vec![Int(0), Int(4), Int(7)]),
                        Int(2),
                        Int(4),
                    ]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(3),
                        List(vec![
                            Int(1),
                            Int(5),
                            Int(2),
                            List(vec![Int(0), Int(8), Int(5), Int(2), Int(4)]),
                            Int(5),
                        ]),
                        List(vec![List(vec![Int(10), Int(0), Int(0), Int(1), Int(2)])]),
                    ]),
                    List(vec![
                        List(vec![List(vec![]), Int(5)]),
                        List(vec![Int(10), List(vec![Int(9)]), Int(9), Int(8)]),
                        List(vec![Int(9), Int(4)]),
                        List(vec![
                            List(vec![Int(2), Int(6)]),
                            List(vec![Int(6), Int(5), Int(7), Int(8)]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![Int(5)]),
                ]),
            ),
            (
                List(vec![List(vec![Int(1)]), List(vec![Int(9)])]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![Int(10), List(vec![]), List(vec![Int(1), Int(7)])]),
                        Int(8),
                        Int(10),
                    ]),
                    List(vec![
                        Int(8),
                        List(vec![List(vec![Int(9), Int(7), Int(9), Int(10)]), Int(6)]),
                        Int(3),
                        Int(1),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![Int(8)]),
                        List(vec![
                            List(vec![Int(10), Int(4)]),
                            List(vec![Int(2), Int(0), Int(1), Int(5), Int(9)]),
                            Int(1),
                            Int(9),
                            List(vec![Int(5), Int(0), Int(1)]),
                        ]),
                    ]),
                    List(vec![
                        Int(9),
                        List(vec![Int(8), Int(2), Int(8), List(vec![Int(3), Int(1)])]),
                        Int(2),
                    ]),
                    List(vec![
                        Int(9),
                        Int(3),
                        List(vec![
                            Int(7),
                            Int(10),
                            List(vec![Int(1), Int(9)]),
                            List(vec![]),
                        ]),
                        List(vec![Int(0), Int(6), Int(10)]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(1),
                        Int(1),
                        Int(2),
                        Int(8),
                        List(vec![List(vec![Int(1)])]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8)]),
                            Int(10),
                            List(vec![Int(0), Int(9), Int(3), Int(6)]),
                        ]),
                        Int(10),
                        List(vec![
                            Int(0),
                            Int(6),
                            List(vec![Int(1), Int(2), Int(1), Int(1)]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(9), Int(1)]),
                            Int(0),
                            Int(2),
                            Int(9),
                            List(vec![Int(2), Int(5), Int(8)]),
                        ]),
                        List(vec![List(vec![Int(0), Int(0), Int(4)]), List(vec![])]),
                        List(vec![
                            List(vec![Int(10), Int(10), Int(0), Int(5), Int(7)]),
                            List(vec![]),
                            List(vec![Int(6), Int(5), Int(9), Int(6)]),
                            Int(5),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(2), Int(5), Int(2), Int(10), Int(2)]),
                            List(vec![Int(3)]),
                            Int(6),
                        ]),
                        List(vec![]),
                        Int(6),
                    ]),
                    List(vec![Int(4), Int(0), Int(8)]),
                ]),
                List(vec![
                    List(vec![Int(0), List(vec![Int(9), Int(0), Int(2)])]),
                    List(vec![
                        List(vec![List(vec![Int(7), Int(0), Int(7), Int(1), Int(4)])]),
                        List(vec![]),
                        Int(3),
                        List(vec![List(vec![Int(4)]), Int(4)]),
                    ]),
                    List(vec![Int(7)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(0), Int(6), Int(1), Int(3), Int(8)]),
                            List(vec![Int(8), Int(0), Int(0), Int(9), Int(1)]),
                        ]),
                        List(vec![Int(10), Int(5), List(vec![Int(7)]), Int(1)]),
                        List(vec![
                            List(vec![Int(8)]),
                            List(vec![]),
                            Int(4),
                            Int(1),
                            List(vec![Int(4), Int(5), Int(8), Int(9), Int(3)]),
                        ]),
                        Int(9),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            Int(5),
                            List(vec![Int(9)]),
                            List(vec![]),
                            List(vec![Int(7), Int(1), Int(7), Int(6), Int(3)]),
                        ]),
                        Int(6),
                        List(vec![
                            List(vec![Int(2), Int(8), Int(4), Int(9), Int(10)]),
                            Int(4),
                            List(vec![Int(1)]),
                            Int(10),
                        ]),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![Int(0)]),
                        List(vec![]),
                        List(vec![
                            Int(4),
                            Int(9),
                            Int(5),
                            List(vec![]),
                            List(vec![Int(2), Int(0), Int(1)]),
                        ]),
                        List(vec![Int(0), Int(8)]),
                        List(vec![
                            List(vec![Int(1), Int(6), Int(2), Int(0), Int(2)]),
                            Int(10),
                            List(vec![Int(2), Int(1), Int(3), Int(4)]),
                            Int(3),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(4),
                            Int(6),
                            List(vec![Int(8), Int(9), Int(9), Int(3)]),
                        ]),
                        List(vec![
                            Int(4),
                            List(vec![]),
                            List(vec![]),
                            List(vec![Int(1), Int(6), Int(4)]),
                        ]),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![
                            List(vec![Int(0)]),
                            List(vec![Int(6)]),
                            List(vec![Int(3), Int(5)]),
                            Int(4),
                        ]),
                        Int(7),
                        Int(4),
                    ]),
                    List(vec![
                        Int(2),
                        Int(2),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(6), Int(9), Int(7), Int(7)]),
                            Int(2),
                        ]),
                        Int(8),
                    ]),
                    List(vec![Int(3), List(vec![]), List(vec![Int(9)]), Int(2)]),
                    List(vec![
                        Int(6),
                        Int(10),
                        List(vec![
                            Int(0),
                            List(vec![Int(0), Int(6), Int(10), Int(8), Int(9)]),
                            Int(2),
                        ]),
                        List(vec![Int(7), List(vec![])]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(1),
                            List(vec![Int(9), Int(3), Int(6), Int(2)]),
                            Int(7),
                            Int(1),
                        ]),
                        List(vec![
                            Int(2),
                            List(vec![Int(5), Int(0), Int(2), Int(2)]),
                            Int(10),
                            List(vec![Int(2)]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![List(vec![Int(8), List(vec![Int(1)]), Int(3), Int(1)])]),
                ]),
                List(vec![
                    List(vec![List(vec![])]),
                    List(vec![
                        List(vec![Int(7), List(vec![Int(0), Int(9), Int(10), Int(9)])]),
                        List(vec![
                            List(vec![Int(10), Int(8), Int(7)]),
                            Int(9),
                            Int(2),
                            Int(10),
                        ]),
                        List(vec![Int(8), Int(4), Int(10), List(vec![Int(3), Int(9)])]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(6), Int(5), Int(5), Int(8)]),
                            Int(8),
                        ]),
                        List(vec![
                            List(vec![Int(7), Int(7)]),
                            List(vec![Int(9), Int(2), Int(3)]),
                            Int(7),
                            Int(3),
                        ]),
                        List(vec![Int(5), Int(9), Int(3)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(6),
                            List(vec![Int(10)]),
                            List(vec![Int(2)]),
                            List(vec![Int(0), Int(5), Int(6), Int(10), Int(7)]),
                        ]),
                        List(vec![
                            Int(5),
                            List(vec![Int(1), Int(3), Int(1)]),
                            List(vec![Int(1), Int(4), Int(0)]),
                            Int(8),
                        ]),
                        Int(1),
                        Int(0),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(0), Int(0), Int(10), Int(4), Int(8)])]),
                        Int(7),
                        Int(3),
                        List(vec![List(vec![Int(5), Int(0), Int(0), Int(9)])]),
                        List(vec![
                            Int(1),
                            List(vec![Int(5), Int(0)]),
                            Int(8),
                            Int(5),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![Int(1), Int(7), Int(7), Int(0), Int(10)]),
                            Int(4),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(1), Int(0)]),
                    List(vec![
                        Int(2),
                        List(vec![Int(5)]),
                        Int(4),
                        List(vec![List(vec![Int(1), Int(9), Int(4), Int(6), Int(9)])]),
                        Int(7),
                    ]),
                    List(vec![List(vec![]), Int(10), Int(5)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(2), Int(6), Int(10), Int(0)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(0)]),
                            List(vec![Int(3), Int(9), Int(9), Int(0)]),
                        ]),
                        List(vec![
                            List(vec![Int(6)]),
                            Int(8),
                            Int(10),
                            List(vec![Int(9), Int(10)]),
                        ]),
                    ]),
                    List(vec![
                        Int(1),
                        List(vec![Int(0), Int(2), Int(2)]),
                        List(vec![]),
                        Int(1),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(8), Int(2), Int(0)]),
                            Int(4),
                            Int(6),
                            Int(2),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(6),
                            List(vec![Int(2), Int(0)]),
                            List(vec![]),
                            Int(1),
                            Int(7),
                        ]),
                        List(vec![
                            Int(2),
                            List(vec![]),
                            List(vec![Int(7), Int(3), Int(3), Int(3), Int(8)]),
                        ]),
                        List(vec![Int(4), Int(9), Int(0), Int(9)]),
                        List(vec![Int(5), Int(10), List(vec![])]),
                    ]),
                    List(vec![Int(5)]),
                ]),
                List(vec![
                    List(vec![Int(9)]),
                    List(vec![Int(9), List(vec![]), Int(10), Int(3), Int(7)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(6), Int(6), Int(2), Int(8)]),
                            List(vec![]),
                        ]),
                        Int(2),
                        List(vec![
                            Int(9),
                            Int(9),
                            List(vec![Int(8), Int(8), Int(10), Int(4), Int(4)]),
                            Int(0),
                        ]),
                        List(vec![Int(9), List(vec![Int(0)])]),
                        Int(9),
                    ]),
                    List(vec![
                        List(vec![
                            Int(4),
                            List(vec![Int(4), Int(7), Int(8), Int(6)]),
                            Int(10),
                        ]),
                        List(vec![]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(2)]),
                            List(vec![Int(8), Int(1), Int(7)]),
                            List(vec![Int(7), Int(5)]),
                            Int(10),
                        ]),
                        Int(5),
                        Int(6),
                        List(vec![]),
                        Int(2),
                    ]),
                    List(vec![List(vec![List(vec![Int(2)]), Int(6)])]),
                    List(vec![Int(5)]),
                ]),
                List(vec![
                    List(vec![List(vec![List(vec![Int(1), Int(9), Int(9)])])]),
                    List(vec![Int(6), Int(8)]),
                    List(vec![List(vec![Int(3)]), List(vec![Int(2), Int(8)])]),
                    List(vec![List(vec![Int(0)]), Int(5), Int(8), Int(4)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![Int(6), Int(2), Int(9), Int(10)]),
                        Int(7),
                        List(vec![
                            List(vec![Int(7), Int(7), Int(6)]),
                            List(vec![Int(6), Int(0), Int(9)]),
                            Int(10),
                        ]),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(0)])]),
                        List(vec![
                            Int(9),
                            Int(10),
                            Int(9),
                            List(vec![Int(5), Int(5), Int(8), Int(0), Int(2)]),
                        ]),
                        Int(10),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(4), Int(9), Int(1), Int(7), Int(1)]),
                            List(vec![Int(4), Int(1)]),
                            Int(0),
                            List(vec![Int(3), Int(7)]),
                        ]),
                        Int(4),
                    ]),
                    List(vec![List(vec![Int(10)])]),
                    List(vec![
                        Int(2),
                        List(vec![Int(3)]),
                        List(vec![
                            List(vec![Int(4), Int(9), Int(10)]),
                            List(vec![]),
                            Int(5),
                        ]),
                        List(vec![Int(8), Int(6), Int(9), Int(6)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(0),
                        Int(8),
                        List(vec![
                            Int(6),
                            List(vec![Int(7), Int(4), Int(10), Int(9)]),
                            Int(0),
                        ]),
                        List(vec![
                            List(vec![Int(4), Int(3), Int(5), Int(10)]),
                            Int(10),
                            List(vec![Int(7), Int(0), Int(5)]),
                            Int(5),
                            Int(2),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(2),
                            List(vec![Int(8), Int(4), Int(6), Int(5)]),
                            List(vec![Int(8)]),
                        ]),
                        List(vec![
                            List(vec![Int(9), Int(1), Int(10), Int(1), Int(6)]),
                            Int(8),
                            Int(7),
                            List(vec![Int(3), Int(7)]),
                            Int(3),
                        ]),
                        List(vec![
                            List(vec![Int(2), Int(5), Int(2)]),
                            Int(7),
                            List(vec![]),
                            Int(2),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![
                        Int(4),
                        List(vec![Int(8), Int(10)]),
                        List(vec![Int(2), Int(6), Int(8)]),
                        List(vec![
                            List(vec![Int(9), Int(4), Int(1), Int(4)]),
                            Int(10),
                            Int(6),
                        ]),
                        List(vec![Int(1)]),
                    ]),
                    List(vec![List(vec![Int(6)]), Int(2)]),
                ]),
                List(vec![List(vec![Int(8)])]),
            ),
            (
                List(vec![List(vec![Int(0), Int(0)])]),
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(4), Int(9), Int(2), Int(6)]),
                            List(vec![]),
                            List(vec![Int(10), Int(1), Int(7), Int(0), Int(1)]),
                        ]),
                        Int(7),
                    ]),
                    List(vec![Int(3)]),
                    List(vec![]),
                    List(vec![List(vec![
                        List(vec![]),
                        List(vec![Int(3), Int(2)]),
                        List(vec![Int(4), Int(8)]),
                    ])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            Int(0),
                            List(vec![Int(7), Int(9), Int(5), Int(8)]),
                            Int(6),
                        ]),
                        Int(8),
                        List(vec![Int(0)]),
                        Int(1),
                        Int(4),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(9), Int(7), Int(4)]),
                    List(vec![List(vec![])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![
                        List(vec![Int(7), Int(4), Int(0)]),
                        List(vec![Int(5)]),
                        List(vec![Int(0), Int(8), Int(0)]),
                        List(vec![Int(0), Int(8), Int(9), Int(6)]),
                        List(vec![Int(3), Int(4), Int(8), Int(10)]),
                    ])]),
                    List(vec![List(vec![
                        List(vec![Int(0), Int(9), Int(9), Int(4)]),
                        Int(8),
                    ])]),
                    List(vec![
                        List(vec![Int(4)]),
                        List(vec![
                            List(vec![Int(6), Int(5), Int(10)]),
                            List(vec![Int(10), Int(5), Int(2), Int(9), Int(4)]),
                        ]),
                        Int(8),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(6)]),
                        List(vec![Int(7), Int(7), Int(6), List(vec![Int(8), Int(0)])]),
                        Int(4),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(4),
                        List(vec![
                            List(vec![Int(7), Int(3), Int(7), Int(6)]),
                            List(vec![Int(10), Int(1), Int(0), Int(5), Int(0)]),
                            Int(8),
                        ]),
                        Int(7),
                        List(vec![Int(4), List(vec![Int(8)]), Int(2), List(vec![])]),
                        List(vec![List(vec![])]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(2), Int(0)]),
                            List(vec![Int(4), Int(1), Int(8), Int(6), Int(1)]),
                            Int(10),
                        ]),
                        Int(7),
                    ]),
                    List(vec![
                        Int(9),
                        List(vec![
                            Int(1),
                            List(vec![Int(2), Int(9), Int(3), Int(0), Int(0)]),
                        ]),
                        Int(5),
                    ]),
                    List(vec![Int(9)]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![]), Int(0), Int(2), Int(3), Int(1)]),
                        List(vec![List(vec![]), Int(8), Int(6), Int(2), List(vec![])]),
                        Int(4),
                    ]),
                    List(vec![
                        Int(4),
                        List(vec![]),
                        Int(2),
                        List(vec![Int(4), List(vec![])]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![List(vec![Int(6), Int(9)]), List(vec![])]),
                List(vec![
                    List(vec![List(vec![Int(10)])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(2), Int(8)]),
                            Int(9),
                            List(vec![Int(6), Int(5)]),
                            Int(7),
                            Int(2),
                        ]),
                        Int(6),
                        Int(6),
                    ]),
                    List(vec![List(vec![Int(3), List(vec![Int(5)])])]),
                    List(vec![]),
                    List(vec![Int(5), List(vec![List(vec![])])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![]), Int(0), Int(5), Int(9)]),
                    List(vec![Int(10), Int(0), Int(4), Int(0)]),
                    List(vec![Int(6)]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(1)])]),
                    List(vec![Int(7)]),
                    List(vec![
                        Int(1),
                        Int(1),
                        List(vec![
                            List(vec![Int(7), Int(4), Int(2)]),
                            Int(6),
                            List(vec![Int(8), Int(4), Int(9)]),
                        ]),
                        List(vec![
                            List(vec![Int(10), Int(7), Int(4), Int(0), Int(0)]),
                            Int(9),
                            Int(2),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(8), Int(10)]),
                    List(vec![
                        Int(3),
                        List(vec![
                            Int(0),
                            List(vec![Int(5)]),
                            List(vec![Int(6)]),
                            List(vec![Int(1), Int(10), Int(6), Int(2)]),
                        ]),
                        Int(10),
                        Int(1),
                    ]),
                    List(vec![]),
                    List(vec![List(vec![]), List(vec![Int(6), Int(7)]), Int(6)]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![Int(7), Int(6), List(vec![]), Int(5)]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![Int(7)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![Int(9)]),
                    List(vec![
                        List(vec![
                            Int(5),
                            Int(1),
                            Int(4),
                            List(vec![Int(10), Int(3), Int(4), Int(5), Int(0)]),
                            Int(5),
                        ]),
                        List(vec![Int(1), Int(5), Int(6)]),
                        List(vec![
                            List(vec![Int(4), Int(2)]),
                            Int(2),
                            Int(6),
                            List(vec![Int(9), Int(9)]),
                        ]),
                        Int(6),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(3),
                        List(vec![
                            Int(8),
                            Int(2),
                            Int(5),
                            List(vec![Int(10), Int(10), Int(9), Int(5)]),
                            List(vec![Int(2), Int(9)]),
                        ]),
                        List(vec![List(vec![Int(1), Int(5), Int(8), Int(7), Int(3)])]),
                        List(vec![]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(3),
                        Int(7),
                        Int(2),
                        List(vec![List(vec![Int(4), Int(0), Int(2), Int(1), Int(9)])]),
                    ]),
                    List(vec![Int(3), Int(1), Int(1)]),
                    List(vec![
                        Int(10),
                        List(vec![]),
                        List(vec![Int(1), Int(5), Int(4), List(vec![Int(5)]), Int(1)]),
                    ]),
                    List(vec![]),
                    List(vec![Int(10), Int(2), Int(3), Int(6)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(8),
                            Int(10),
                            List(vec![Int(1), Int(0), Int(1), Int(9)]),
                        ]),
                        List(vec![
                            List(vec![Int(1), Int(1), Int(3), Int(2), Int(7)]),
                            Int(1),
                            List(vec![Int(4), Int(4), Int(6), Int(6)]),
                        ]),
                        Int(7),
                        Int(4),
                        Int(2),
                    ]),
                    List(vec![
                        Int(3),
                        List(vec![List(vec![Int(4), Int(8), Int(5), Int(9)])]),
                        List(vec![
                            Int(4),
                            List(vec![Int(2), Int(6), Int(4), Int(1), Int(2)]),
                            Int(9),
                        ]),
                        Int(6),
                        Int(7),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(6)]),
                            Int(0),
                            List(vec![Int(7), Int(6), Int(4), Int(3)]),
                            List(vec![Int(8), Int(1), Int(1), Int(2), Int(5)]),
                        ]),
                        Int(8),
                        Int(7),
                        List(vec![Int(9), Int(10), Int(7)]),
                    ]),
                    List(vec![Int(9)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![]), Int(5)]),
                    List(vec![List(vec![]), List(vec![]), Int(8), Int(7)]),
                ]),
                List(vec![
                    List(vec![List(vec![]), Int(8)]),
                    List(vec![Int(3), Int(2), Int(4)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(1),
                        Int(6),
                        Int(5),
                        List(vec![
                            List(vec![Int(0), Int(5)]),
                            List(vec![Int(0)]),
                            List(vec![]),
                        ]),
                        Int(3),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(2),
                        List(vec![
                            List(vec![Int(2), Int(8), Int(8), Int(7), Int(4)]),
                            List(vec![Int(6), Int(9), Int(4)]),
                            List(vec![]),
                            List(vec![Int(2), Int(10), Int(9)]),
                            Int(1),
                        ]),
                        Int(5),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![List(vec![Int(9)]), Int(2), Int(7)]),
                    ]),
                    List(vec![List(vec![
                        Int(1),
                        Int(0),
                        List(vec![Int(1), Int(6), Int(3), Int(6), Int(5)]),
                        List(vec![Int(10), Int(2)]),
                        Int(6),
                    ])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(6)]),
                            List(vec![Int(2), Int(7), Int(6), Int(3)]),
                            Int(6),
                            Int(2),
                            Int(7),
                        ]),
                        Int(10),
                        List(vec![List(vec![]), List(vec![Int(8)])]),
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), Int(1), Int(1)]),
                            Int(3),
                            List(vec![Int(7), Int(0)]),
                            Int(3),
                        ]),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(0), Int(10), Int(5)])]),
                        Int(5),
                        Int(10),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![]), Int(6), Int(9)]),
                        Int(8),
                        List(vec![Int(8), Int(2), Int(4)]),
                        Int(3),
                    ]),
                    List(vec![]),
                    List(vec![Int(9), List(vec![]), Int(4), Int(7), Int(3)]),
                    List(vec![
                        List(vec![
                            Int(2),
                            Int(5),
                            Int(4),
                            Int(9),
                            List(vec![Int(8), Int(5), Int(6), Int(1), Int(6)]),
                        ]),
                        List(vec![List(vec![]), Int(10), Int(5), Int(10)]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(2), Int(1), Int(7), Int(7), Int(9)]),
                            List(vec![Int(0), Int(3), Int(4), Int(10)]),
                            List(vec![Int(0), Int(1), Int(1), Int(7), Int(8)]),
                            Int(10),
                        ]),
                        Int(1),
                        List(vec![Int(3)]),
                    ]),
                    List(vec![
                        Int(9),
                        Int(4),
                        List(vec![
                            List(vec![]),
                            Int(9),
                            List(vec![]),
                            List(vec![Int(7)]),
                            List(vec![Int(0)]),
                        ]),
                    ]),
                    List(vec![Int(1), Int(3), Int(6), Int(3), Int(4)]),
                    List(vec![List(vec![Int(3), Int(3)]), Int(5)]),
                ]),
            ),
            (
                List(vec![List(vec![Int(6), Int(3)]), List(vec![List(vec![])])]),
                List(vec![
                    List(vec![
                        List(vec![]),
                        Int(6),
                        List(vec![
                            List(vec![Int(9)]),
                            List(vec![Int(1), Int(7), Int(0), Int(8), Int(4)]),
                            List(vec![Int(9), Int(3), Int(0), Int(8)]),
                            List(vec![Int(4)]),
                            Int(1),
                        ]),
                        Int(8),
                        Int(7),
                    ]),
                    List(vec![
                        Int(7),
                        List(vec![
                            Int(2),
                            Int(2),
                            Int(2),
                            List(vec![Int(5), Int(7), Int(8), Int(4)]),
                        ]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![List(vec![List(vec![]), Int(6)])]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(6), Int(7)]),
                            Int(9),
                            List(vec![Int(8), Int(7), Int(9)]),
                            Int(5),
                        ]),
                        Int(10),
                        Int(3),
                    ]),
                    List(vec![List(vec![Int(6), List(vec![Int(9)]), Int(9), Int(6)])]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![
                        Int(10),
                        List(vec![Int(1)]),
                        List(vec![List(vec![Int(9), Int(1), Int(10), Int(0)])]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![List(vec![
                    List(vec![Int(3), Int(5), Int(5), Int(0)]),
                    Int(3),
                    Int(8),
                    Int(9),
                ])])]),
                List(vec![List(vec![List(vec![
                    Int(9),
                    List(vec![Int(10), Int(4), Int(9)]),
                ])])]),
            ),
            (
                List(vec![
                    List(vec![Int(5), Int(0), Int(9), Int(6)]),
                    List(vec![
                        Int(9),
                        List(vec![List(vec![Int(3)]), Int(10), Int(9), Int(3)]),
                        List(vec![Int(8)]),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9), Int(4), Int(2), Int(8)]),
                            List(vec![Int(8), Int(10), Int(2)]),
                            Int(4),
                            List(vec![]),
                            Int(0),
                        ]),
                        List(vec![Int(2), List(vec![])]),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(6),
                        List(vec![Int(5), Int(5), List(vec![Int(9)])]),
                        Int(4),
                    ]),
                ]),
                List(vec![List(vec![]), List(vec![]), List(vec![])]),
            ),
            (
                List(vec![
                    List(vec![List(vec![Int(4), Int(5), List(vec![])])]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            List(vec![Int(0), Int(9)]),
                            Int(6),
                            Int(3),
                        ]),
                        Int(3),
                    ]),
                    List(vec![Int(10), List(vec![Int(4)]), Int(5), Int(8)]),
                    List(vec![List(vec![Int(4), Int(5), Int(2), Int(10), Int(5)])]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![Int(0)]),
                        Int(3),
                        List(vec![
                            Int(4),
                            List(vec![Int(10), Int(1), Int(1), Int(4)]),
                            Int(8),
                            List(vec![Int(8), Int(1)]),
                            List(vec![Int(4), Int(0), Int(5), Int(4)]),
                        ]),
                        List(vec![List(vec![Int(5)]), Int(8), List(vec![Int(9)])]),
                    ]),
                    List(vec![List(vec![
                        Int(0),
                        List(vec![]),
                        List(vec![Int(2), Int(7)]),
                    ])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![
                        Int(4),
                        List(vec![Int(8)]),
                        List(vec![Int(2), Int(3), Int(5)]),
                        List(vec![Int(10), Int(4), Int(9), Int(9), Int(3)]),
                    ])]),
                    List(vec![
                        Int(9),
                        List(vec![Int(0), Int(1), List(vec![Int(6), Int(0), Int(2)])]),
                        Int(6),
                        List(vec![List(vec![]), Int(4), Int(7), Int(0), List(vec![])]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(2),
                            Int(7),
                            List(vec![]),
                            List(vec![Int(10), Int(1), Int(4), Int(7), Int(9)]),
                            Int(0),
                        ]),
                        List(vec![]),
                        Int(7),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![
                            Int(0),
                            Int(0),
                            Int(9),
                            List(vec![Int(9), Int(5), Int(3), Int(6), Int(10)]),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![Int(9), Int(7), List(vec![Int(9), Int(10)])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(0),
                            Int(10),
                            List(vec![Int(1)]),
                            Int(3),
                            List(vec![Int(4), Int(1), Int(7), Int(6), Int(9)]),
                        ]),
                        Int(2),
                        Int(3),
                        Int(6),
                        List(vec![
                            List(vec![Int(1)]),
                            List(vec![Int(0), Int(2), Int(3)]),
                            List(vec![Int(9)]),
                        ]),
                    ]),
                    List(vec![List(vec![List(vec![]), List(vec![Int(1)])]), Int(2)]),
                    List(vec![Int(10), Int(7), List(vec![Int(5)])]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![Int(9), Int(6), Int(4), Int(4), Int(4)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(4), Int(7), Int(7), Int(10)]),
                            Int(3),
                        ]),
                        Int(0),
                        List(vec![List(vec![Int(6), Int(3), Int(5)]), List(vec![])]),
                        List(vec![
                            List(vec![Int(8), Int(6), Int(10), Int(7), Int(0)]),
                            List(vec![Int(8), Int(3), Int(7)]),
                            Int(0),
                            Int(3),
                        ]),
                        List(vec![
                            List(vec![Int(4), Int(9), Int(8), Int(4), Int(4)]),
                            Int(3),
                            List(vec![Int(4)]),
                            List(vec![Int(7), Int(5), Int(1), Int(9)]),
                            Int(2),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(8),
                            List(vec![Int(10), Int(2), Int(8)]),
                            List(vec![Int(10), Int(4), Int(0), Int(0)]),
                            Int(0),
                            List(vec![Int(7)]),
                        ]),
                        Int(7),
                        List(vec![
                            List(vec![Int(7), Int(3), Int(8), Int(2)]),
                            Int(10),
                            Int(10),
                            List(vec![Int(2)]),
                            List(vec![Int(9), Int(10), Int(10), Int(5), Int(5)]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        Int(10),
                        List(vec![
                            Int(8),
                            List(vec![Int(1), Int(2), Int(5), Int(1)]),
                            List(vec![Int(2), Int(7), Int(6), Int(7), Int(5)]),
                        ]),
                        List(vec![
                            Int(9),
                            List(vec![Int(9), Int(3), Int(9)]),
                            List(vec![Int(4), Int(8)]),
                            Int(5),
                            Int(6),
                        ]),
                        Int(1),
                    ]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            List(vec![Int(7), Int(8), Int(6), Int(4)]),
                            List(vec![Int(8), Int(8), Int(7), Int(5), Int(3)]),
                            Int(6),
                            Int(4),
                            List(vec![Int(8), Int(1), Int(7)]),
                        ]),
                        Int(1),
                    ]),
                    List(vec![Int(5), Int(4), List(vec![])]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![Int(7)]),
                    List(vec![List(vec![Int(2)])]),
                    List(vec![]),
                    List(vec![List(vec![])]),
                ]),
            ),
            (
                List(vec![List(vec![Int(5), Int(8)])]),
                List(vec![
                    List(vec![Int(7), List(vec![]), Int(7), List(vec![])]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![Int(1), Int(8)])]),
                    List(vec![Int(0), Int(6)]),
                    List(vec![Int(9), Int(6), List(vec![List(vec![])])]),
                    List(vec![Int(5), List(vec![List(vec![Int(0)])])]),
                ]),
                List(vec![
                    List(vec![List(vec![List(vec![Int(5), Int(6), Int(1)])]), Int(5)]),
                    List(vec![]),
                    List(vec![
                        Int(2),
                        List(vec![]),
                        Int(9),
                        List(vec![List(vec![])]),
                        List(vec![
                            Int(7),
                            Int(10),
                            List(vec![Int(7), Int(9)]),
                            List(vec![]),
                        ]),
                    ]),
                    List(vec![Int(3)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(1), Int(10), Int(3)]),
                    List(vec![Int(7), Int(9)]),
                    List(vec![
                        List(vec![List(vec![Int(1)]), List(vec![Int(6), Int(9)])]),
                        Int(0),
                    ]),
                    List(vec![
                        Int(4),
                        List(vec![
                            List(vec![Int(8), Int(4)]),
                            List(vec![Int(8), Int(0), Int(3), Int(0), Int(10)]),
                            List(vec![Int(0), Int(6), Int(2), Int(8), Int(10)]),
                            Int(4),
                            Int(3),
                        ]),
                        Int(3),
                        Int(7),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(8), Int(4)]),
                            List(vec![Int(6), Int(0), Int(9)]),
                            Int(0),
                        ]),
                        Int(3),
                        List(vec![
                            List(vec![Int(8)]),
                            List(vec![Int(10), Int(9), Int(6)]),
                            List(vec![]),
                            List(vec![Int(10), Int(3), Int(2), Int(0), Int(0)]),
                        ]),
                        Int(7),
                        Int(2),
                    ]),
                    List(vec![
                        Int(1),
                        Int(9),
                        Int(4),
                        List(vec![
                            Int(9),
                            List(vec![Int(7), Int(6)]),
                            Int(8),
                            List(vec![Int(3), Int(1)]),
                            Int(4),
                        ]),
                    ]),
                    List(vec![
                        Int(4),
                        List(vec![Int(6)]),
                        List(vec![
                            List(vec![Int(2), Int(7), Int(3)]),
                            List(vec![Int(9), Int(6), Int(0)]),
                            List(vec![Int(8), Int(1)]),
                            List(vec![Int(7), Int(4)]),
                        ]),
                        List(vec![
                            List(vec![Int(1), Int(4), Int(0), Int(2), Int(8)]),
                            Int(7),
                            Int(2),
                            Int(4),
                        ]),
                    ]),
                    List(vec![List(vec![List(vec![
                        Int(6),
                        Int(3),
                        Int(1),
                        Int(9),
                        Int(8),
                    ])])]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(10), Int(8), Int(1), Int(5)])]),
                        List(vec![List(vec![Int(3), Int(3), Int(5)])]),
                        Int(5),
                        Int(4),
                    ]),
                    List(vec![
                        Int(2),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(6), Int(7), Int(2), Int(7), Int(7)]),
                            Int(5),
                            Int(10),
                        ]),
                        Int(4),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(0)]), List(vec![]), List(vec![])]),
                        Int(5),
                        Int(9),
                        List(vec![Int(6)]),
                    ]),
                    List(vec![]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(8), Int(10), List(vec![Int(10), Int(8)])]),
                    List(vec![]),
                    List(vec![List(vec![List(vec![
                        Int(4),
                        Int(0),
                        Int(6),
                        Int(3),
                        Int(2),
                    ])])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(5), Int(9)]),
                            List(vec![Int(5), Int(8)]),
                            Int(2),
                        ]),
                        Int(0),
                    ]),
                    List(vec![Int(8)]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(6), Int(8)]),
                            Int(6),
                            Int(2),
                            Int(5),
                        ]),
                        Int(6),
                        Int(7),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(3), Int(5)]),
                            Int(4),
                            List(vec![Int(9), Int(10), Int(8), Int(1)]),
                        ]),
                        List(vec![List(vec![Int(8), Int(5), Int(4)]), Int(6)]),
                        List(vec![
                            List(vec![Int(8), Int(10), Int(6), Int(8)]),
                            Int(2),
                            List(vec![Int(0)]),
                            List(vec![]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(5)]),
                            List(vec![Int(9), Int(5), Int(9), Int(1)]),
                            Int(10),
                            List(vec![Int(0), Int(2), Int(6), Int(9), Int(9)]),
                            Int(3),
                        ]),
                        Int(10),
                        Int(7),
                        Int(3),
                        Int(6),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![Int(0), List(vec![Int(9)]), Int(10), Int(0)]),
                        Int(9),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(4), Int(8)]),
                            List(vec![Int(6), Int(5), Int(2)]),
                        ]),
                        Int(2),
                    ]),
                ]),
                List(vec![List(vec![
                    List(vec![
                        List(vec![Int(0), Int(0)]),
                        List(vec![Int(6)]),
                        Int(10),
                    ]),
                    Int(10),
                    List(vec![Int(4), Int(3)]),
                    Int(9),
                    Int(2),
                ])]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        List(vec![]),
                        Int(0),
                        List(vec![Int(8), Int(3)]),
                        List(vec![Int(1), Int(9), Int(0), Int(1), Int(6)]),
                    ]),
                    Int(9),
                ])]),
                List(vec![
                    List(vec![
                        Int(8),
                        Int(2),
                        List(vec![List(vec![Int(4)]), Int(0)]),
                        Int(1),
                    ]),
                    List(vec![List(vec![])]),
                    List(vec![Int(0), Int(0), Int(9)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![Int(6), List(vec![])]),
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(0), Int(0), Int(6), Int(0)]),
                            List(vec![]),
                            List(vec![Int(5), Int(4), Int(8)]),
                            List(vec![Int(7), Int(0), Int(10), Int(3), Int(0)]),
                            Int(0),
                        ]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(10), Int(3)]),
                            Int(6),
                            Int(1),
                        ]),
                        Int(2),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9)]),
                            List(vec![Int(8), Int(0), Int(10), Int(0), Int(9)]),
                            List(vec![Int(2), Int(7), Int(3)]),
                            List(vec![Int(3), Int(2)]),
                        ]),
                        Int(5),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(10),
                        List(vec![
                            Int(8),
                            List(vec![Int(6), Int(3), Int(0), Int(8), Int(0)]),
                            Int(9),
                        ]),
                    ]),
                    List(vec![Int(3)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(3),
                        List(vec![Int(0)]),
                        List(vec![
                            Int(10),
                            Int(6),
                            List(vec![Int(9), Int(5), Int(4)]),
                            List(vec![Int(1), Int(6), Int(5), Int(10)]),
                        ]),
                        Int(9),
                        Int(9),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![Int(10)]),
                        Int(9),
                        List(vec![
                            List(vec![]),
                            Int(10),
                            List(vec![Int(10), Int(4), Int(6), Int(0)]),
                            List(vec![Int(6), Int(8)]),
                            List(vec![Int(7), Int(7)]),
                        ]),
                        Int(3),
                        Int(10),
                    ]),
                    List(vec![
                        List(vec![
                            Int(1),
                            Int(7),
                            List(vec![Int(5), Int(9), Int(9)]),
                            List(vec![]),
                            Int(9),
                        ]),
                        Int(6),
                        Int(7),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(6), Int(2)]),
                    List(vec![List(vec![List(vec![]), Int(4), Int(6)])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(10),
                        Int(6),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(3), Int(8), Int(7), Int(7), Int(7)]),
                            List(vec![Int(7)]),
                            List(vec![Int(5)]),
                            Int(7),
                        ]),
                        Int(4),
                    ]),
                    List(vec![
                        Int(6),
                        List(vec![]),
                        List(vec![Int(0), Int(5), Int(1), Int(4), Int(2)]),
                        Int(8),
                    ]),
                ]),
                List(vec![List(vec![]), List(vec![Int(4)])]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(3), Int(3)])]),
                        Int(1),
                        List(vec![
                            Int(0),
                            Int(8),
                            List(vec![Int(6), Int(10), Int(7), Int(5), Int(8)]),
                            List(vec![Int(1), Int(1), Int(1), Int(10)]),
                        ]),
                        Int(5),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(5), Int(10)])]),
                        List(vec![Int(7), List(vec![Int(7), Int(2)])]),
                        List(vec![Int(10), Int(7), Int(8), List(vec![Int(4)]), Int(6)]),
                        Int(6),
                        List(vec![List(vec![Int(4)])]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(0)]),
                            Int(0),
                            List(vec![Int(10), Int(5), Int(10)]),
                            Int(1),
                        ]),
                        List(vec![
                            Int(10),
                            Int(2),
                            Int(9),
                            List(vec![Int(5), Int(7), Int(10), Int(0)]),
                            List(vec![Int(10), Int(4), Int(10), Int(6), Int(1)]),
                        ]),
                        Int(9),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(9),
                        List(vec![
                            List(vec![Int(7), Int(9), Int(9)]),
                            List(vec![Int(5), Int(8), Int(9), Int(8)]),
                            Int(2),
                            List(vec![Int(6)]),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![Int(1), Int(1), List(vec![Int(2), Int(3)])]),
                ]),
            ),
            (
                List(vec![Int(0), Int(3), Int(7), Int(3), Int(4)]),
                List(vec![Int(0), Int(3), Int(7), Int(3)]),
            ),
            (
                List(vec![
                    List(vec![Int(4), Int(2), Int(9), List(vec![Int(5), Int(6)])]),
                    List(vec![]),
                    List(vec![]),
                    List(vec![]),
                ]),
                List(vec![List(vec![Int(4), Int(4)]), List(vec![Int(9)])]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        List(vec![List(vec![Int(6), Int(0), Int(7)]), Int(0)]),
                        List(vec![Int(4), Int(0), Int(2), Int(3)]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(6),
                            Int(5),
                            List(vec![Int(10), Int(0), Int(9), Int(9)]),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![
                            Int(0),
                            Int(0),
                            List(vec![Int(8), Int(9), Int(9), Int(0)]),
                            List(vec![Int(6)]),
                        ]),
                        Int(1),
                    ]),
                ]),
                List(vec![List(vec![Int(10), Int(6), Int(5)])]),
            ),
            (
                List(vec![List(vec![Int(3)]), List(vec![])]),
                List(vec![
                    List(vec![
                        Int(6),
                        Int(7),
                        List(vec![Int(3), List(vec![Int(4), Int(4)]), Int(5), Int(0)]),
                    ]),
                    List(vec![Int(9), Int(10)]),
                    List(vec![List(vec![
                        List(vec![Int(8), Int(6), Int(5), Int(0), Int(1)]),
                        Int(6),
                        Int(1),
                    ])]),
                    List(vec![
                        List(vec![]),
                        List(vec![Int(0), Int(10)]),
                        Int(10),
                        List(vec![
                            Int(10),
                            List(vec![Int(1), Int(8), Int(1)]),
                            List(vec![]),
                        ]),
                        List(vec![
                            List(vec![Int(8)]),
                            List(vec![Int(7), Int(2), Int(10), Int(6), Int(2)]),
                            Int(6),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(1), Int(4), Int(8), Int(8), Int(0)]),
                            Int(5),
                        ]),
                        Int(6),
                        List(vec![]),
                        Int(2),
                        List(vec![List(vec![Int(2), Int(5)]), Int(3)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![]), Int(2), Int(7)]),
                    List(vec![
                        List(vec![]),
                        List(vec![
                            Int(3),
                            List(vec![Int(8), Int(7)]),
                            List(vec![Int(3), Int(6), Int(10)]),
                            Int(8),
                            List(vec![Int(9), Int(4)]),
                        ]),
                        Int(1),
                        Int(3),
                        Int(6),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(0)]),
                    List(vec![
                        List(vec![Int(6), List(vec![Int(7)])]),
                        List(vec![
                            List(vec![Int(6)]),
                            List(vec![Int(8)]),
                            Int(9),
                            Int(7),
                            List(vec![Int(9)]),
                        ]),
                        List(vec![List(vec![])]),
                        List(vec![
                            List(vec![Int(0)]),
                            List(vec![Int(7), Int(0)]),
                            List(vec![]),
                            Int(5),
                            Int(1),
                        ]),
                        List(vec![Int(4), List(vec![Int(9)]), List(vec![Int(10)])]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(8),
                            List(vec![]),
                            Int(2),
                            List(vec![Int(8), Int(1), Int(7), Int(4)]),
                        ]),
                        List(vec![
                            List(vec![]),
                            List(vec![]),
                            List(vec![Int(10), Int(4), Int(7), Int(6)]),
                            Int(2),
                            Int(2),
                        ]),
                    ]),
                    List(vec![Int(4)]),
                    List(vec![
                        List(vec![List(vec![Int(0), Int(3), Int(7)]), Int(5)]),
                        List(vec![
                            List(vec![Int(9), Int(0), Int(10), Int(7), Int(5)]),
                            Int(7),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![List(vec![]), List(vec![Int(0), Int(8), Int(3)])]),
                        List(vec![
                            Int(1),
                            List(vec![Int(9), Int(0), Int(5), Int(10), Int(7)]),
                            Int(9),
                            Int(1),
                            Int(5),
                        ]),
                        List(vec![
                            List(vec![Int(1), Int(1), Int(2)]),
                            List(vec![Int(5)]),
                            Int(1),
                            Int(8),
                            Int(0),
                        ]),
                    ]),
                    List(vec![Int(4)]),
                ]),
                List(vec![
                    List(vec![
                        Int(2),
                        List(vec![List(vec![Int(6)]), Int(4), Int(1), Int(8)]),
                        Int(7),
                        Int(0),
                    ]),
                    List(vec![
                        List(vec![Int(0), List(vec![Int(1), Int(5)]), Int(2)]),
                        List(vec![
                            List(vec![Int(4), Int(9), Int(5), Int(9), Int(1)]),
                            Int(4),
                            Int(2),
                            Int(0),
                        ]),
                    ]),
                    List(vec![List(vec![List(vec![])])]),
                    List(vec![Int(10), Int(7), List(vec![List(vec![]), Int(2)])]),
                    List(vec![
                        List(vec![Int(4), List(vec![Int(6), Int(8), Int(8), Int(7)])]),
                        List(vec![
                            List(vec![Int(3), Int(1), Int(9), Int(1), Int(7)]),
                            Int(7),
                            Int(5),
                            Int(10),
                            List(vec![Int(3), Int(5)]),
                        ]),
                        List(vec![Int(2)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(0), Int(0), Int(6), Int(2), Int(4)]),
                            List(vec![]),
                            Int(8),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(8),
                            Int(10),
                            Int(9),
                            Int(7),
                            List(vec![Int(8), Int(3), Int(5), Int(6), Int(4)]),
                        ]),
                        List(vec![
                            List(vec![Int(8), Int(10), Int(8), Int(2)]),
                            List(vec![Int(4), Int(6), Int(5), Int(0)]),
                            Int(1),
                            Int(4),
                            List(vec![Int(4), Int(6), Int(9), Int(0), Int(6)]),
                        ]),
                        List(vec![
                            List(vec![Int(7), Int(6)]),
                            List(vec![Int(1), Int(2), Int(4)]),
                            Int(3),
                            Int(8),
                            Int(1),
                        ]),
                        List(vec![Int(6), List(vec![Int(8), Int(9)]), List(vec![])]),
                    ]),
                    List(vec![Int(1), Int(1)]),
                ]),
                List(vec![
                    List(vec![
                        Int(9),
                        List(vec![List(vec![Int(2), Int(2), Int(3)])]),
                        Int(4),
                        List(vec![Int(1), Int(0)]),
                    ]),
                    List(vec![
                        Int(7),
                        Int(10),
                        Int(4),
                        List(vec![
                            List(vec![Int(9), Int(5), Int(2), Int(3)]),
                            List(vec![Int(7), Int(3)]),
                            List(vec![Int(5), Int(6), Int(2), Int(8)]),
                            List(vec![Int(8), Int(4), Int(10)]),
                            Int(8),
                        ]),
                        Int(0),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![
                        Int(8),
                        List(vec![Int(1), Int(9)]),
                        Int(2),
                        List(vec![Int(2), Int(9), Int(3), Int(8), Int(7)]),
                        List(vec![Int(1), Int(7), Int(8), Int(4), Int(2)]),
                    ])]),
                    List(vec![Int(4)]),
                ]),
                List(vec![
                    List(vec![Int(0), Int(4)]),
                    List(vec![]),
                    List(vec![
                        Int(4),
                        List(vec![]),
                        List(vec![List(vec![])]),
                        List(vec![List(vec![Int(8), Int(5), Int(7), Int(9)])]),
                    ]),
                    List(vec![
                        List(vec![Int(2)]),
                        Int(4),
                        List(vec![
                            Int(5),
                            Int(6),
                            List(vec![Int(0), Int(8), Int(8), Int(1)]),
                            List(vec![Int(7), Int(4), Int(9)]),
                            Int(8),
                        ]),
                    ]),
                    List(vec![
                        Int(1),
                        List(vec![Int(2), List(vec![Int(9), Int(5)])]),
                        List(vec![Int(0), List(vec![Int(8)])]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(6),
                        List(vec![
                            List(vec![Int(6)]),
                            Int(3),
                            Int(2),
                            List(vec![Int(10), Int(6), Int(7), Int(6)]),
                            Int(0),
                        ]),
                        Int(10),
                    ]),
                    List(vec![Int(10)]),
                    List(vec![List(vec![Int(3)]), Int(6), Int(7)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(10), Int(1)]),
                            List(vec![Int(3), Int(3), Int(1), Int(5), Int(2)]),
                        ]),
                        Int(7),
                        List(vec![Int(0), List(vec![Int(2), Int(10)])]),
                        List(vec![Int(1), List(vec![Int(1)]), Int(1), Int(5), Int(7)]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(7),
                        List(vec![]),
                        List(vec![
                            Int(6),
                            List(vec![Int(3), Int(2), Int(0), Int(8)]),
                            List(vec![Int(7)]),
                            List(vec![Int(0), Int(9), Int(9), Int(4), Int(9)]),
                        ]),
                        List(vec![
                            List(vec![Int(2)]),
                            List(vec![Int(1), Int(8), Int(6), Int(6)]),
                            List(vec![Int(1), Int(6), Int(8), Int(5), Int(1)]),
                            List(vec![Int(0)]),
                        ]),
                        List(vec![
                            Int(5),
                            Int(6),
                            List(vec![Int(2), Int(7), Int(10)]),
                            List(vec![Int(4), Int(2), Int(4)]),
                            List(vec![Int(10), Int(9)]),
                        ]),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(3),
                            List(vec![Int(1), Int(5), Int(3), Int(5)]),
                            Int(6),
                            List(vec![Int(0), Int(0)]),
                        ]),
                        List(vec![Int(0)]),
                        Int(4),
                    ]),
                    List(vec![
                        Int(8),
                        List(vec![List(vec![Int(9)]), Int(8), Int(8), Int(4)]),
                        List(vec![]),
                        Int(1),
                    ]),
                    List(vec![List(vec![
                        Int(5),
                        List(vec![Int(0), Int(8), Int(6), Int(6), Int(5)]),
                        Int(0),
                        List(vec![Int(9), Int(10), Int(1)]),
                    ])]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![]),
                        Int(10),
                        List(vec![
                            List(vec![Int(0), Int(6), Int(10), Int(0)]),
                            Int(3),
                            List(vec![Int(2), Int(1)]),
                            List(vec![Int(7), Int(4), Int(2), Int(8)]),
                            List(vec![Int(6), Int(1)]),
                        ]),
                    ]),
                    List(vec![Int(7), Int(8), Int(8), Int(6)]),
                    List(vec![
                        List(vec![Int(9), Int(2), Int(6), Int(4), Int(1)]),
                        List(vec![
                            List(vec![Int(7), Int(3), Int(8), Int(9), Int(0)]),
                            Int(7),
                            Int(3),
                        ]),
                        List(vec![
                            List(vec![]),
                            List(vec![Int(0), Int(10), Int(5), Int(9)]),
                        ]),
                        List(vec![List(vec![Int(4), Int(1), Int(0), Int(2)])]),
                    ]),
                    List(vec![List(vec![Int(6)])]),
                ]),
                List(vec![List(vec![Int(9), Int(0)])]),
            ),
            (
                List(vec![List(vec![Int(4)]), List(vec![Int(5)])]),
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(3), Int(8), Int(10)]), Int(10)]),
                        Int(9),
                        List(vec![
                            Int(7),
                            List(vec![Int(0), Int(0), Int(9), Int(5), Int(1)]),
                            Int(4),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![
                        Int(1),
                        List(vec![
                            Int(1),
                            Int(5),
                            Int(0),
                            List(vec![Int(10), Int(5), Int(9), Int(0)]),
                            Int(0),
                        ]),
                        Int(8),
                        Int(1),
                        List(vec![List(vec![Int(7)]), Int(1), Int(4)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        Int(5),
                        List(vec![
                            List(vec![Int(2)]),
                            List(vec![Int(2), Int(10), Int(1), Int(10), Int(1)]),
                        ]),
                        List(vec![]),
                        Int(3),
                    ]),
                    List(vec![]),
                    List(vec![List(vec![Int(10)]), Int(0)]),
                    List(vec![Int(6), Int(5), Int(1)]),
                    List(vec![List(vec![
                        List(vec![Int(10), Int(5), Int(4), Int(10)]),
                        List(vec![]),
                        List(vec![Int(8), Int(6), Int(0), Int(5)]),
                        Int(3),
                        Int(2),
                    ])]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![Int(6)]),
                    List(vec![
                        List(vec![Int(3), Int(1), List(vec![Int(4), Int(6)])]),
                        List(vec![
                            List(vec![Int(8), Int(9), Int(7)]),
                            List(vec![Int(3), Int(0)]),
                            Int(5),
                        ]),
                        Int(0),
                        Int(9),
                    ]),
                    List(vec![
                        List(vec![
                            Int(2),
                            List(vec![Int(7), Int(9), Int(2)]),
                            List(vec![Int(2)]),
                        ]),
                        List(vec![
                            List(vec![Int(6), Int(8), Int(4), Int(10)]),
                            Int(8),
                            Int(3),
                            Int(1),
                            List(vec![Int(5), Int(7), Int(8)]),
                        ]),
                        Int(6),
                        List(vec![Int(4)]),
                    ]),
                    List(vec![
                        List(vec![Int(6), List(vec![Int(5)])]),
                        List(vec![
                            List(vec![Int(1), Int(7), Int(9)]),
                            Int(10),
                            List(vec![Int(3), Int(9), Int(0), Int(4), Int(3)]),
                            List(vec![Int(7), Int(7), Int(8), Int(3)]),
                        ]),
                        Int(4),
                        List(vec![Int(0), Int(9), Int(2), Int(2)]),
                        Int(9),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        Int(2),
                        List(vec![]),
                        List(vec![Int(8), Int(10), Int(7)]),
                        List(vec![Int(0), Int(1)]),
                    ]),
                    Int(7),
                    List(vec![Int(8)]),
                    List(vec![List(vec![Int(8), Int(3), Int(3)])]),
                ])]),
                List(vec![
                    List(vec![Int(3)]),
                    List(vec![
                        Int(6),
                        List(vec![
                            List(vec![Int(3), Int(6)]),
                            Int(3),
                            List(vec![Int(4), Int(4), Int(6), Int(2)]),
                        ]),
                        Int(4),
                        Int(7),
                        List(vec![]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(10),
                            Int(4),
                            Int(5),
                            List(vec![Int(2), Int(8), Int(4), Int(5)]),
                            Int(4),
                        ]),
                        List(vec![Int(1)]),
                        List(vec![List(vec![Int(4), Int(5), Int(2), Int(0), Int(6)])]),
                        List(vec![List(vec![]), Int(8), Int(5)]),
                        List(vec![
                            Int(6),
                            List(vec![Int(4), Int(9), Int(10)]),
                            Int(1),
                            Int(4),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(7)]),
                        List(vec![
                            Int(3),
                            List(vec![Int(9), Int(10), Int(2), Int(1)]),
                            Int(5),
                            Int(3),
                        ]),
                        Int(10),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    Int(8),
                    Int(2),
                    List(vec![List(vec![Int(0), Int(5)]), Int(5), Int(10)]),
                ])]),
                List(vec![
                    List(vec![
                        Int(1),
                        List(vec![]),
                        List(vec![Int(7), Int(7), Int(4), Int(2), List(vec![Int(4)])]),
                        Int(8),
                    ]),
                    List(vec![
                        List(vec![]),
                        Int(5),
                        Int(4),
                        List(vec![
                            List(vec![Int(2), Int(6), Int(2)]),
                            List(vec![Int(8)]),
                            Int(0),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![Int(5), Int(8), Int(8), Int(9)]),
                    List(vec![
                        List(vec![
                            List(vec![Int(8), Int(8), Int(3), Int(5)]),
                            List(vec![Int(7), Int(10)]),
                            Int(2),
                            List(vec![Int(0), Int(1), Int(6)]),
                            Int(8),
                        ]),
                        List(vec![
                            Int(0),
                            List(vec![]),
                            Int(3),
                            List(vec![Int(8), Int(7), Int(10), Int(2)]),
                        ]),
                        List(vec![
                            Int(8),
                            Int(8),
                            List(vec![Int(4), Int(1)]),
                            Int(2),
                            List(vec![Int(8), Int(4), Int(9)]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        Int(5),
                        List(vec![Int(10), Int(10)]),
                        List(vec![Int(9), Int(6)]),
                        List(vec![]),
                        Int(9),
                    ]),
                    Int(1),
                ])]),
                List(vec![
                    List(vec![]),
                    List(vec![Int(7), List(vec![])]),
                    List(vec![]),
                    List(vec![Int(9)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(2), Int(7)]),
                    List(vec![
                        Int(3),
                        List(vec![
                            List(vec![Int(10)]),
                            List(vec![Int(6), Int(3), Int(8), Int(4), Int(1)]),
                            Int(5),
                        ]),
                        Int(1),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![]), Int(1), Int(3)]),
                    List(vec![
                        Int(6),
                        Int(1),
                        List(vec![
                            Int(5),
                            List(vec![Int(2), Int(2), Int(8)]),
                            List(vec![Int(3), Int(5), Int(5), Int(7)]),
                            Int(6),
                            List(vec![Int(6), Int(7), Int(9), Int(10)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![]),
                            List(vec![Int(5), Int(3), Int(0), Int(0)]),
                            Int(7),
                        ]),
                        List(vec![]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(8),
                            List(vec![Int(1), Int(8), Int(5)]),
                            Int(8),
                            List(vec![Int(7)]),
                        ]),
                        Int(1),
                        Int(10),
                        Int(7),
                    ]),
                    List(vec![List(vec![Int(0), Int(6), Int(2)])]),
                    List(vec![
                        List(vec![
                            List(vec![]),
                            Int(9),
                            Int(1),
                            List(vec![Int(1), Int(4), Int(3)]),
                        ]),
                        Int(9),
                    ]),
                    List(vec![
                        Int(0),
                        Int(9),
                        List(vec![
                            List(vec![Int(3)]),
                            List(vec![Int(9), Int(2), Int(2), Int(6), Int(4)]),
                        ]),
                        List(vec![List(vec![])]),
                        List(vec![Int(0), Int(3), List(vec![Int(1)])]),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(2), Int(10), Int(7)]), Int(2)]),
                        List(vec![
                            List(vec![Int(8), Int(7), Int(10), Int(5), Int(1)]),
                            Int(10),
                            Int(1),
                            Int(7),
                        ]),
                        Int(4),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(3), Int(6), Int(8), Int(9)]),
                            Int(9),
                        ]),
                        Int(10),
                        Int(6),
                    ]),
                    List(vec![
                        Int(4),
                        Int(3),
                        List(vec![
                            List(vec![Int(2), Int(1), Int(4)]),
                            List(vec![Int(10), Int(5), Int(9), Int(6)]),
                        ]),
                        Int(0),
                        List(vec![List(vec![Int(8)]), Int(10), Int(7), Int(1)]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![Int(1)]),
                    List(vec![List(vec![
                        List(vec![Int(4), Int(2), Int(10), Int(3), Int(5)]),
                        Int(9),
                        List(vec![]),
                        Int(0),
                        List(vec![Int(9)]),
                    ])]),
                    List(vec![
                        List(vec![
                            Int(3),
                            Int(5),
                            List(vec![]),
                            List(vec![Int(6), Int(9), Int(4)]),
                        ]),
                        List(vec![
                            Int(7),
                            Int(5),
                            List(vec![Int(7), Int(7), Int(5), Int(8), Int(8)]),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![List(vec![
                        Int(1),
                        List(vec![Int(1), Int(7), Int(2), Int(1), Int(2)]),
                        Int(3),
                        List(vec![Int(0), Int(4), Int(9), Int(8), Int(2)]),
                    ])]),
                    List(vec![]),
                ]),
                List(vec![
                    List(vec![
                        List(vec![
                            Int(5),
                            List(vec![Int(1), Int(0), Int(1), Int(9), Int(0)]),
                            List(vec![Int(8), Int(6), Int(2), Int(7)]),
                            Int(5),
                        ]),
                        List(vec![Int(7), Int(3), Int(4)]),
                        List(vec![
                            List(vec![Int(10), Int(2), Int(6), Int(9)]),
                            List(vec![Int(10), Int(9)]),
                            List(vec![Int(1)]),
                            List(vec![Int(3), Int(5), Int(6)]),
                            Int(7),
                        ]),
                        List(vec![List(vec![])]),
                    ]),
                    List(vec![Int(2), List(vec![]), Int(10), Int(7)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![Int(10)]),
                        List(vec![Int(4), List(vec![Int(4), Int(10), Int(1)]), Int(3)]),
                        List(vec![
                            List(vec![Int(0), Int(8), Int(10), Int(6)]),
                            List(vec![]),
                            Int(9),
                            Int(3),
                        ]),
                        List(vec![
                            List(vec![Int(7)]),
                            List(vec![Int(9), Int(2), Int(5)]),
                            List(vec![]),
                            Int(4),
                            List(vec![]),
                        ]),
                        Int(7),
                    ]),
                    List(vec![
                        List(vec![List(vec![Int(2), Int(7), Int(10), Int(10), Int(9)])]),
                        List(vec![List(vec![Int(10)]), Int(1)]),
                    ]),
                    List(vec![
                        Int(8),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(2), Int(7), Int(2), Int(4)]),
                            Int(2),
                            List(vec![Int(0)]),
                            Int(0),
                        ]),
                        Int(2),
                    ]),
                ]),
                List(vec![
                    List(vec![
                        Int(3),
                        List(vec![
                            Int(8),
                            List(vec![Int(0), Int(8), Int(0), Int(8), Int(1)]),
                            List(vec![Int(1), Int(5), Int(5), Int(0), Int(8)]),
                        ]),
                        Int(0),
                        Int(8),
                        List(vec![
                            List(vec![Int(6), Int(7), Int(7)]),
                            List(vec![Int(0), Int(0), Int(9), Int(10), Int(10)]),
                            Int(2),
                            Int(5),
                            Int(6),
                        ]),
                    ]),
                    List(vec![
                        Int(4),
                        List(vec![Int(8)]),
                        List(vec![
                            Int(8),
                            List(vec![]),
                            Int(9),
                            List(vec![Int(4), Int(9), Int(7), Int(10), Int(8)]),
                            Int(9),
                        ]),
                        List(vec![Int(2), Int(6), Int(2), List(vec![Int(3), Int(7)])]),
                        Int(7),
                    ]),
                    List(vec![
                        List(vec![Int(0)]),
                        List(vec![
                            Int(7),
                            List(vec![Int(4), Int(5)]),
                            List(vec![Int(1), Int(5), Int(0)]),
                        ]),
                        Int(7),
                        List(vec![
                            List(vec![Int(9), Int(0)]),
                            List(vec![Int(1), Int(1)]),
                            Int(2),
                            Int(6),
                        ]),
                    ]),
                    List(vec![
                        Int(6),
                        List(vec![
                            Int(3),
                            List(vec![Int(9)]),
                            List(vec![Int(2), Int(5), Int(3), Int(5)]),
                            Int(1),
                        ]),
                        List(vec![]),
                    ]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![List(vec![Int(10), Int(8)])]),
                        List(vec![
                            List(vec![Int(7)]),
                            List(vec![Int(7), Int(1), Int(3)]),
                            List(vec![Int(9), Int(6), Int(3), Int(6), Int(8)]),
                        ]),
                        List(vec![
                            List(vec![Int(6)]),
                            List(vec![Int(0), Int(4)]),
                            Int(3),
                            Int(8),
                            List(vec![Int(10), Int(1), Int(9)]),
                        ]),
                        Int(4),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(7)]),
                            List(vec![Int(4), Int(7), Int(5), Int(0), Int(3)]),
                        ]),
                        Int(8),
                        Int(10),
                    ]),
                ]),
                List(vec![
                    List(vec![Int(3), Int(1), List(vec![])]),
                    List(vec![List(vec![
                        Int(1),
                        List(vec![Int(2), Int(10), Int(9), Int(10)]),
                        List(vec![]),
                        List(vec![Int(2), Int(7), Int(10)]),
                    ])]),
                    List(vec![List(vec![List(vec![]), Int(10), Int(8), Int(8)])]),
                    List(vec![Int(5), Int(8), List(vec![Int(6)])]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![
                    List(vec![List(vec![
                        List(vec![]),
                        Int(0),
                        List(vec![Int(3), Int(2), Int(2), Int(2)]),
                        Int(10),
                    ])]),
                    List(vec![List(vec![Int(5), Int(0)])]),
                ]),
                List(vec![
                    List(vec![
                        Int(9),
                        List(vec![]),
                        Int(4),
                        List(vec![
                            List(vec![Int(2), Int(0), Int(0), Int(8)]),
                            List(vec![Int(7), Int(4)]),
                        ]),
                    ]),
                    List(vec![List(vec![Int(1)]), Int(2)]),
                    List(vec![List(vec![])]),
                    List(vec![List(vec![List(vec![]), Int(7)]), Int(4), Int(0)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            Int(8),
                            Int(3),
                            List(vec![Int(1), Int(4), Int(4), Int(4)]),
                            List(vec![Int(9), Int(1), Int(4), Int(7), Int(0)]),
                        ]),
                        List(vec![
                            Int(2),
                            List(vec![Int(6), Int(1), Int(6), Int(5), Int(3)]),
                            Int(10),
                            Int(2),
                            Int(8),
                        ]),
                        List(vec![
                            Int(0),
                            Int(9),
                            List(vec![Int(10), Int(3), Int(1), Int(4)]),
                        ]),
                    ]),
                    List(vec![
                        List(vec![Int(4), Int(5)]),
                        Int(0),
                        Int(4),
                        Int(5),
                        List(vec![
                            List(vec![Int(3), Int(4), Int(4), Int(3)]),
                            List(vec![Int(1), Int(0), Int(10)]),
                            Int(9),
                            Int(5),
                        ]),
                    ]),
                    List(vec![]),
                    List(vec![Int(4)]),
                ]),
                List(vec![
                    List(vec![List(vec![
                        List(vec![Int(2), Int(9), Int(1)]),
                        List(vec![]),
                        Int(4),
                        Int(7),
                    ])]),
                    List(vec![]),
                ]),
            ),
            (
                List(vec![List(vec![
                    List(vec![
                        Int(0),
                        Int(3),
                        Int(3),
                        List(vec![]),
                        List(vec![Int(10), Int(0), Int(8), Int(2)]),
                    ]),
                    Int(1),
                ])]),
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(4), Int(1), Int(3), Int(9), Int(0)]),
                            List(vec![Int(7), Int(0), Int(3)]),
                            Int(3),
                        ]),
                        List(vec![]),
                        List(vec![Int(9)]),
                        List(vec![
                            List(vec![Int(7)]),
                            Int(7),
                            List(vec![Int(6), Int(1), Int(4), Int(6), Int(5)]),
                        ]),
                        List(vec![Int(0)]),
                    ]),
                    List(vec![Int(10)]),
                    List(vec![Int(7), Int(4), Int(2), Int(8), Int(6)]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(9), Int(3), Int(7), Int(1)]),
                            Int(2),
                        ]),
                        List(vec![
                            List(vec![Int(10), Int(2), Int(2), Int(4), Int(6)]),
                            Int(5),
                        ]),
                        Int(10),
                        Int(0),
                        Int(5),
                    ]),
                    List(vec![Int(4), List(vec![])]),
                    List(vec![
                        List(vec![List(vec![]), Int(2)]),
                        List(vec![]),
                        List(vec![
                            List(vec![Int(6), Int(6)]),
                            Int(8),
                            Int(4),
                            List(vec![Int(10), Int(8)]),
                        ]),
                        List(vec![
                            List(vec![Int(5), Int(1), Int(2), Int(4)]),
                            Int(4),
                            List(vec![Int(9)]),
                            Int(7),
                        ]),
                    ]),
                    List(vec![
                        List(vec![
                            List(vec![Int(9)]),
                            Int(7),
                            Int(2),
                            List(vec![Int(6), Int(0), Int(9), Int(8)]),
                            List(vec![Int(4), Int(1)]),
                        ]),
                        Int(5),
                        List(vec![]),
                        List(vec![
                            Int(5),
                            Int(8),
                            List(vec![Int(5), Int(2), Int(7), Int(6), Int(5)]),
                        ]),
                        List(vec![List(vec![Int(2)])]),
                    ]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(7),
                            Int(0),
                            List(vec![Int(0)]),
                            List(vec![Int(0), Int(1), Int(2), Int(1), Int(6)]),
                        ]),
                        Int(2),
                        Int(6),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(5), Int(0), Int(9), Int(4), Int(6)]),
                            Int(8),
                            Int(2),
                        ]),
                        Int(1),
                        Int(0),
                        List(vec![
                            List(vec![Int(9), Int(4), Int(3)]),
                            List(vec![Int(10), Int(5), Int(0), Int(1)]),
                            List(vec![]),
                        ]),
                        Int(0),
                    ]),
                    List(vec![]),
                    List(vec![
                        List(vec![List(vec![]), Int(9), Int(4)]),
                        List(vec![
                            List(vec![Int(2), Int(8), Int(3)]),
                            Int(6),
                            Int(0),
                            List(vec![Int(7), Int(7), Int(5)]),
                        ]),
                        List(vec![Int(1), List(vec![Int(10)]), Int(8), List(vec![])]),
                    ]),
                    List(vec![
                        Int(4),
                        Int(2),
                        List(vec![
                            Int(4),
                            Int(5),
                            List(vec![Int(6), Int(3), Int(10), Int(1), Int(6)]),
                            List(vec![Int(10), Int(3)]),
                        ]),
                    ]),
                    List(vec![List(vec![
                        List(vec![Int(1), Int(2)]),
                        List(vec![Int(0), Int(1)]),
                        Int(7),
                    ])]),
                ]),
                List(vec![
                    List(vec![Int(5), Int(9)]),
                    List(vec![
                        Int(6),
                        List(vec![Int(9)]),
                        List(vec![
                            Int(1),
                            List(vec![Int(9), Int(8), Int(9), Int(6)]),
                            Int(9),
                            Int(6),
                            List(vec![Int(1), Int(5), Int(7), Int(6)]),
                        ]),
                        Int(7),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), Int(7), Int(2)]),
                            Int(6),
                            List(vec![Int(5), Int(9), Int(4), Int(10)]),
                            List(vec![]),
                        ]),
                    ]),
                ]),
            ),
            (
                List(vec![
                    List(vec![
                        List(vec![
                            List(vec![Int(0), Int(7), Int(7), Int(1), Int(8)]),
                            List(vec![Int(2), Int(1), Int(9)]),
                        ]),
                        Int(0),
                        Int(2),
                        Int(6),
                        Int(3),
                    ]),
                    List(vec![
                        Int(5),
                        List(vec![Int(2), List(vec![Int(10)]), Int(6)]),
                    ]),
                ]),
                List(vec![List(vec![
                    Int(7),
                    List(vec![]),
                    List(vec![List(vec![Int(1)]), List(vec![Int(4), Int(6)])]),
                    Int(7),
                    List(vec![Int(9), List(vec![Int(1)]), Int(9)]),
                ])]),
            ),
            (
                List(vec![
                    List(vec![]),
                    List(vec![]),
                    List(vec![Int(6), Int(4), Int(8)]),
                    List(vec![Int(0), List(vec![])]),
                    List(vec![List(vec![Int(7), List(vec![])]), Int(5)]),
                ]),
                List(vec![
                    List(vec![]),
                    List(vec![List(vec![Int(4), Int(2)]), Int(0), List(vec![Int(2)])]),
                    List(vec![
                        List(vec![
                            List(vec![Int(3), Int(9)]),
                            List(vec![Int(1), Int(0), Int(1)]),
                            Int(6),
                        ]),
                        Int(7),
                        List(vec![
                            List(vec![Int(2), Int(4), Int(9), Int(5)]),
                            Int(9),
                            List(vec![Int(4), Int(8), Int(6)]),
                            Int(10),
                            List(vec![Int(2), Int(2), Int(8), Int(4)]),
                        ]),
                    ]),
                    List(vec![Int(0), Int(4)]),
                ]),
            ),
        ]
    } else {
        unimplemented!("{}", s.lines().count())
    }
}
