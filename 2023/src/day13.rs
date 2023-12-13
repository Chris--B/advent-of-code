#![allow(unused)]

use crate::prelude::*;

fn find_mirrored_axis(rows: &[String], cols: &[String]) -> i64 {
    for (scale, strings) in [(100, rows), (1, cols)] {
        'split: for split in 1..strings.len() {
            let (ra, rb) = strings.split_at(split);
            let n = ra.len().min(rb.len());
            if n == 0 {
                continue;
            }

            let ra = &ra[(ra.len() - n)..];
            let rb = &rb[..n];

            let mut diffs = 0;
            for (ca, cb) in ra.iter().zip(rb.iter().rev()) {
                if ca != cb {
                    diffs += 1;
                    continue 'split;
                }
            }

            if diffs == 0 {
                return scale * split as i64;
            }
        }
    }

    unreachable!()
}

// Part1 ========================================================================
#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let mut sum = 0;

    let blocks = input.lines().group_by(|l| l.trim().is_empty());
    for (is_empty, block) in &blocks {
        if is_empty {
            continue;
        }

        let mut rows: Vec<String> = vec![];
        let mut cols: Vec<String> = vec![];
        cols.resize_with(20, String::new);

        if log_enabled!(Info) {
            println!();
            println!("              10        20");
            println!("     123456789_123456789_123456789_");
        }

        for (y, line) in block.into_iter().enumerate() {
            if log_enabled!(Info) {
                println!("[{:>2}] {line}", y + 1);
            }

            if cols.len() > line.len() {
                cols.resize_with(line.len(), String::new);
            }

            // Save the row directly
            rows.push(line.to_string());

            // Save (part of) each column
            for (x, c) in line.chars().enumerate() {
                cols[x].push(c)
            }
        }
        if log_enabled!(Info) {
            println!();
        }

        sum += find_mirrored_axis(&rows, &cols);
    }

    sum
}

fn find_smudge_mirrored_axis(rows: &[String], cols: &[String]) -> i64 {
    for (scale, strings) in [(100, rows), (1, cols)] {
        'split: for split in 1..strings.len() {
            let (ra, rb) = strings.split_at(split);
            let n = ra.len().min(rb.len());
            if n == 0 {
                continue;
            }

            let ra = &ra[(ra.len() - n)..];
            let rb = &rb[..n];

            let mut diffs = 0;
            for (ca, cb) in ra.iter().zip(rb.iter().rev()) {
                for (a, b) in ca.chars().zip(cb.chars()) {
                    if a != b {
                        diffs += 1;
                    }
                }
            }

            info!("[s={split}, n={n}] diffs={diffs} w/ {ra:#?}, {rb:#?}");
            if diffs == 1 {
                return scale * split as i64;
            }
        }
    }

    unreachable!()
}

// Part2 ========================================================================
#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut sum = 0;

    let blocks = input.lines().group_by(|l| l.trim().is_empty());
    for (is_empty, block) in &blocks {
        if is_empty {
            continue;
        }

        let mut rows: Vec<String> = vec![];
        let mut cols: Vec<String> = vec![];
        cols.resize_with(20, String::new);

        if log_enabled!(Info) {
            println!();
            println!("              10        20");
            println!("     123456789_123456789_123456789_");
        }

        for (y, line) in block.into_iter().enumerate() {
            if log_enabled!(Info) {
                println!("[{:>2}] {line}", y + 1);
            }

            if cols.len() > line.len() {
                cols.resize_with(line.len(), String::new);
            }

            // Save the row directly
            rows.push(line.to_string());

            // Save (part of) each column
            for (x, c) in line.chars().enumerate() {
                cols[x].push(c)
            }
        }
        if log_enabled!(Info) {
            println!();
        }

        sum += find_smudge_mirrored_axis(&rows, &cols);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_HALF_A: &str = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";
    const EXAMPLE_INPUT_HALF_B: &str = r"
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    const EXAMPLE_INPUT: &str = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    const CASE_Y_1_2: &str = r"
###
###
...
..#
.##
#.#
";

    const CASE_X_1_2: &str = r"
##.###
##..#.
##...#
";

    #[rstest]
    #[case::given_405(405, EXAMPLE_INPUT)]
    #[case::half_a(5, EXAMPLE_INPUT_HALF_A)]
    #[case::half_b(400, EXAMPLE_INPUT_HALF_B)]
    #[case::mine_x(1, CASE_X_1_2)]
    #[case::mine_y(100, CASE_Y_1_2)]
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
    #[case::given(400, EXAMPLE_INPUT)]
    #[case::half_a(300, EXAMPLE_INPUT_HALF_A)]
    #[case::half_b(100, EXAMPLE_INPUT_HALF_B)]
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
