#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Entry {
    dims: (usize, usize),
    counts: [u8; 5],
}

type Shape = [u8; 9];

fn print_shape(mut shape: Shape) {
    if cfg!(test) {
        println!("  {}", just_str(&shape[0..3]));
        println!("  {}", just_str(&shape[3..6]));
        println!("  {}", just_str(&shape[6..9]));
        println!();
    }
}

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(input: &str) -> i64 {
    let mut shapes: Vec<Shape> = vec![];

    for (_a, b, c, d, _) in input.lines().take(30).tuples() {
        let mut shape: Shape = [0; 9];

        let b = b.as_bytes();
        shape[0..3].copy_from_slice(&b[0..3]);

        let c = c.as_bytes();
        shape[3..6].copy_from_slice(&c[0..3]);

        let d = d.as_bytes();
        shape[6..9].copy_from_slice(&d[0..3]);

        shapes.push(shape);

        if cfg!(test) {
            println!("{_a}");
            print_shape(shape);
        }
    }

    let entries: Vec<Entry> = input
        .lines()
        .skip(30)
        .map(|line| {
            let nums = line.i64s().collect_vec();
            let entry = Entry {
                dims: (nums[0] as _, nums[1] as _),
                counts: [
                    nums[2] as _,
                    nums[3] as _,
                    nums[4] as _,
                    nums[5] as _,
                    nums[6] as _,
                ],
            };
            if cfg!(test) {
                println!("  + {entry:?}");
            }

            entry
        })
        .collect_vec();

    println!("Found {} entries. good luck.", entries.len());

    entries
        .into_iter()
        .filter(
            |Entry {
                 dims: (x, y),
                 counts,
             }| { true },
        )
        .count() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[rstest]
    #[case::given(2, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
