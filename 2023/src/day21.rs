#![allow(unused)]

use crate::prelude::*;

type Grid = Framebuffer<char>;

fn parse(input: &str) -> (IVec2, Grid) {
    let grid = Grid::parse_grid(input, |b| b);
    let start = grid
        .iter_coords()
        .find(|(x, y)| grid[(*x, *y)] == 'S')
        .unwrap()
        .into();

    (start, grid)
}

// Part1 ========================================================================
fn do_part1(steps: i64, input: &str) -> i64 {
    let (start, mut grid) = parse(input);
    grid.set_border_color(Some('#'));

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
    do_part1(64, input)
}

// Part2 ========================================================================
fn do_part2(steps: i64, input: &str) -> i64 {
    let (start, grid) = parse(input);

    let nx = grid.width() as i32;
    let ny = grid.height() as i32;

    let mut ps: Vec<IVec2> = vec![start];
    let mut seen_this_step: Vec<IVec2> = vec![];

    for step in 1..=steps {
        for p in ps.drain(..) {
            for dir in Cardinal::ALL_NO_DIAG {
                let next = p + dir.into();

                // In part 2, the map is infinitely repeating so handle that when indexing
                // TODO: Ask Framebuffer to do this for us
                fn w(a: i32, n: i32) -> i32 {
                    // Keep the indices positive
                    ((a % n) + n) % n
                }
                if grid[(w(next.x, nx), w(next.y, ny))] == '#' {
                    continue;
                }

                if !seen_this_step.contains(&next) {
                    seen_this_step.push(next);
                }
            }
        }

        // info!("[step {step:2>}] plots={}", seen_this_step.len());
        ps.append(&mut seen_this_step);

        const STEPS: &[i64] = &[1, 10, 15];
        if cfg!(test) && STEPS.contains(&step) {
            // Render a 3x3 grid of the maps
            // mx and my will be the map coordinates
            println!("step={step} ===================");
            for my in -1..2 {
                for y in grid.range_y() {
                    // "full" coordinate
                    let yy = my * ny + y;
                    for mx in -1..2 {
                        for x in grid.range_x() {
                            // "full" coordinate
                            let xx = mx * nx + x;
                            if ps.contains(&(xx, yy).into()) {
                                print!("O");
                            } else {
                                print!("{}", grid[(x, y)]);
                            }
                        }
                        print!(" ");
                    }
                    println!();
                }
                println!();
            }
            println!();
        }
    }

    ps.len() as i64
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    do_part2(26_501_365, input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;
    use std::time::Duration;

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

    fn ms(ms: u32) -> Duration {
        Duration::from_millis(ms.into())
    }
    #[rstest]
    #[case::given(6, 16, EXAMPLE_INPUT)]
    #[case::given_10(10, 50, EXAMPLE_INPUT)]
    #[case::given_50(50, 1594, EXAMPLE_INPUT)]
    #[case::given_100(100, 6536, EXAMPLE_INPUT)]
    #[case::given_500(500, 167004, EXAMPLE_INPUT)]
    #[case::given_1000(1000, 668697, EXAMPLE_INPUT)]
    #[case::given_5000(5000, 16733044, EXAMPLE_INPUT)]
    #[timeout(ms(1_000))]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] steps: i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(do_part2(steps, input), expected);
    }
}
