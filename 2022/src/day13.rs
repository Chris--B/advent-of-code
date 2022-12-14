use crate::prelude::*;

use json::JsonValue;

fn compare(a: &JsonValue, b: &JsonValue) -> Option<bool> {
    use JsonValue::*;

    match (a, b) {
        (Number(_), Number(_)) => {
            let aa = a.as_f32().unwrap();
            let bb = b.as_f32().unwrap();

            if aa == bb {
                None
            } else {
                Some(aa < bb)
            }
        }
        (Number(_), Array(_)) => {
            let mut a_as_arr = JsonValue::new_array();
            a_as_arr.push(a.clone()).unwrap();

            compare(&a_as_arr, b)
        }
        (Array(_), Number(_)) => {
            let mut b_as_arr = JsonValue::new_array();
            b_as_arr.push(b.clone()).unwrap();

            compare(a, &b_as_arr)
        }
        (Array(aa), Array(bb)) => {
            for (x, y) in aa.iter().zip(bb.iter()) {
                let res = compare(x, y);
                if res.is_some() {
                    return res;
                }
            }

            if aa.len() == bb.len() {
                None
            } else {
                Some(aa.len() < bb.len())
            }
        }
        _ => unreachable!(),
    }
}

// Part1 ========================================================================
#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| json::parse(l).unwrap())
        .tuples()
        .map(|(a, b)| compare(&a, &b).unwrap())
        .enumerate()
        // .inspect(|(i, b)| {
        //     println!("Pair {}: {}", i + 1, b);
        // })
        .filter_map(|(i, b)| if b { Some(i as i64 + 1) } else { None })
        .sum()
}

// Part2 ========================================================================
// #[aoc(day13, part2)]
// pub fn part2(_input: &str) -> i64 {
//     0
// }

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

    // #[rstest]
    // #[case::given(140, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
