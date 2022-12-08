use aoc_runner_derive::aoc;

use std::collections::HashSet;

// Part1 ========================================================================
#[aoc(day8, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let mut visible = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let lines: Vec<&str> = input.lines().collect();

    // check rows
    for (y, line) in input.lines().enumerate() {
        let mut tallest: u8 = 0;
        for (x, b) in line.as_bytes().iter().enumerate() {
            if *b > tallest {
                tallest = *b;
                if !seen.contains(&(x, y)) {
                    visible += 1;
                    seen.insert((x, y));
                }
            }
            // Nothing taller
            if *b == 9 {
                break;
            }
        }

        let mut tallest: u8 = 0;
        for (x, b) in line.as_bytes().iter().enumerate().rev() {
            if *b > tallest {
                tallest = *b;
                if !seen.contains(&(x, y)) {
                    visible += 1;
                    seen.insert((x, y));
                }
            }
            // Nothing taller
            if *b == 9 {
                break;
            }
        }
    }

    let width = input.lines().next().unwrap().as_bytes().len();

    for x in 0..width {
        let mut tallest = 0;
        for (y, line) in lines.iter().enumerate() {
            let b = line.as_bytes()[x];
            if b > tallest {
                tallest = b;
                if !seen.contains(&(x, y)) {
                    visible += 1;
                    seen.insert((x, y));
                }
            }
        }

        let mut tallest = 0;
        for (y, line) in lines.iter().enumerate().rev() {
            let b = line.as_bytes()[x];
            if b > tallest {
                tallest = b;
                if !seen.contains(&(x, y)) {
                    visible += 1;
                    seen.insert((x, y));
                }
            }
        }
    }

    visible
}

fn scenic_score(forest: &[Vec<u8>], x: usize, y: usize) -> i64 {
    use std::num::Wrapping;

    let width = forest[0].len();
    let height = forest.len();

    let mut score = 1;
    let base = forest[y][x];

    // println!("(({x}, {y})");
    {
        let mut xx = Wrapping(x) + Wrapping(1);
        let yy = Wrapping(y);
        let mut visible = 0;

        while xx < Wrapping(width) {
            visible += 1;

            if forest[yy.0][xx.0] >= base {
                break;
            }

            xx += 1;
        }

        // dbg!(visible);
        score *= visible;
    }

    {
        let mut xx = Wrapping(x) - Wrapping(1);
        let yy = Wrapping(y);
        let mut visible = 0;

        while xx < Wrapping(width) {
            visible += 1;

            if forest[yy.0][xx.0] >= base {
                break;
            }

            xx -= 1;
        }

        // dbg!(visible);
        score *= visible;
    }

    {
        let xx = Wrapping(x);
        let mut yy = Wrapping(y) + Wrapping(1);
        let mut visible = 0;

        while yy < Wrapping(height) {
            visible += 1;

            if forest[yy.0][xx.0] >= base {
                break;
            }

            yy += 1;
        }

        // dbg!(visible);
        score *= visible;
    }

    {
        let xx = Wrapping(x);
        let mut yy = Wrapping(y) - Wrapping(1);
        let mut visible = 0;

        while yy < Wrapping(height) {
            visible += 1;

            if forest[yy.0][xx.0] >= base {
                break;
            }

            yy -= 1;
        }

        // dbg!(visible);
        score *= visible;
    }

    score
}

// Part2 ========================================================================
#[aoc(day8, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    let forest: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    let width = forest[0].len();
    let height = forest.len();
    let mut score = 0;

    for x in 0..width {
        for y in 0..height {
            score = score.max(scenic_score(&forest, x, y));
        }
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
30373
25512
65332
33549
35390
";

    #[rstest]
    #[case::given(21, EXAMPLE_INPUT)]
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
    #[case::given(8, EXAMPLE_INPUT)]
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
