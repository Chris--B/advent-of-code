use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Num {
    order: usize,
    val: i32,
}

// Part1 ========================================================================
#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    let mut xs: VecDeque<Num> = input
        .lines()
        .enumerate()
        .map(|(i, l)| Num {
            order: i,
            val: l.parse().unwrap(),
        })
        .collect();

    let mut order = 0;
    // println!("Initial arrangement:");
    // for x in &xs {
    //     print!("{}, ", x.val);
    // }
    // println!();

    while order < xs.len() {
        while xs[0].order != order {
            xs.rotate_left(1);
        }
        order += 1;

        let x = xs.pop_front().unwrap();

        if x.val > 0 {
            for _ in 0..x.val {
                xs.rotate_left(1);
            }
        } else if x.val < 0 {
            for _ in 0..(x.val.unsigned_abs()) {
                xs.rotate_right(1);
            }
        }

        xs.insert(0, x);
    }

    let zero = xs.iter().position(|x| x.val == 0).unwrap();

    xs.rotate_left(zero);

    let a = 1_000 % xs.len();
    let b = 2_000 % xs.len();
    let c = 3_000 % xs.len();

    (xs[a].val + xs[b].val + xs[c].val) as i64
}

// Part2 ========================================================================
// #[aoc(day20, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
1
2
-3
3
-2
0
4
";

    #[rstest]
    #[case::given(3, EXAMPLE_INPUT)]
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
    // #[case::given(999_999, EXAMPLE_INPUT)]
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
