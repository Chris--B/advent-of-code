use aoc_runner_derive::aoc;

use scan_fmt::scan_fmt;

use std::collections::HashSet;
use std::fmt::Write;

#[derive(Copy, Clone, Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}
use Fold::*;

pub fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let dots: HashSet<_> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();
            assert_eq!(iter.next(), None);

            (x, y)
        })
        .collect();

    let folds: Vec<_> = input
        .lines()
        .skip_while(|line| !line.starts_with('f'))
        .map(|line| {
            let (axis, dim) = scan_fmt!(line, "fold along {}={}", char, usize).unwrap();
            match axis {
                'x' => X(dim),
                'y' => Y(dim),
                _ => unreachable!("{}={}", axis, dim),
            }
        })
        .collect();

    (dots, folds)
}

// Part1 ======================================================================
#[aoc(day13, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let (mut dots, folds) = parse_input(input);

    let mut after_fold = HashSet::new();

    match dbg!(folds[0]) {
        X(fold_x) => {
            for (x, y) in dots.drain() {
                assert_ne!(x, fold_x);
                if x < fold_x {
                    // Copy verbatim
                    after_fold.insert((x, y));
                    continue;
                }

                let dx = x - fold_x;
                after_fold.insert((fold_x - dx, y));
            }
        }
        Y(fold_y) => {
            for (x, y) in dots.drain() {
                assert_ne!(y, fold_y);
                if y < fold_y {
                    // Copy verbatim
                    after_fold.insert((x, y));
                    continue;
                }

                let dy = y - fold_y;
                after_fold.insert((x, fold_y - dy));
            }
        }
    }

    after_fold.len()
}

// Part2 ======================================================================
#[aoc(day13, part2)]
#[inline(never)]
pub fn part2(input: &str) -> String {
    let (mut dots, mut folds) = parse_input(input);

    let mut dots_post = HashSet::new();

    for fold in folds.drain(..) {
        match fold {
            X(fold_x) => {
                for (x, y) in dots.drain() {
                    assert_ne!(x, fold_x);
                    if x < fold_x {
                        // Copy verbatim
                        dots_post.insert((x, y));
                        continue;
                    }

                    let dx = x - fold_x;
                    dots_post.insert((fold_x - dx, y));
                }
            }
            Y(fold_y) => {
                for (x, y) in dots.drain() {
                    assert_ne!(y, fold_y);
                    if y < fold_y {
                        // Copy verbatim
                        dots_post.insert((x, y));
                        continue;
                    }

                    let dy = y - fold_y;
                    dots_post.insert((x, fold_y - dy));
                }
            }
        }

        dots.extend(dots_post.drain());
    }

    let min_x = dots.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = dots.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap();

    let mut code = String::with_capacity(max_x * max_y);

    // aoc runner starts us indented a little, so make sure we stay aligned
    code.push('\n');

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                write!(code, "#").unwrap();
            } else {
                write!(code, " ").unwrap();
            }
        }
        code.push('\n');
    }

    code
}

#[test]
fn check_example_1() {
    let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    assert_eq!(part1(input), 17);
}
