#![allow(unused)]

use crate::prelude::*;

type Grid = Framebuffer<char>;

fn parse(input: &str) -> (IVec2, Grid) {
    let mut grid = Grid::parse_grid(input, |b| b);
    grid.set_border_color(Some('#'));

    let start = grid
        .iter_coords()
        .find(|(x, y)| grid[(*x, *y)] == 'S')
        .unwrap()
        .into();

    (start, grid)
}

// Part1 ========================================================================
fn do_part1(steps: i64, input: &str) -> i64 {
    let (start, grid) = parse(input);

    let mut ps: Vec<IVec2> = vec![start];
    let mut seen_this_step: Vec<IVec2> = vec![];

    for step in 1..=steps {
        for p in ps.drain(..) {
            for dir in Cardinal::ALL_NO_DIAG {
                let next = p + dir.into();

                if grid[next] == '#' {
                    continue;
                }

                if !seen_this_step.contains(&next) {
                    seen_this_step.push(next);
                }
            }
        }

        ps.append(&mut seen_this_step);

        if cfg!(test) {
            println!("=== step={step}, plots={}", ps.len());
            grid.print(|x, y, c| if ps.contains(&(x, y).into()) { 'O' } else { *c });
            println!();
        } else {
            info!("[step {step:2>}] plots={}", ps.len());
        }
    }

    ps.len() as i64
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let plots = do_part1(64, input);

    if !cfg!(test) {
        assert!(plots < 591890, "too high");
    }

    plots
}

// Part2 ========================================================================
#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[rstest]
    #[case::given(6, 16, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] steps: i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(do_part1(steps, input), expected);
    }

    #[ignore]
    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
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
