use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
    let dim = if cfg!(test) { 10 } else { 100 };
    let mut platform = Framebuffer::new(dim + 1, dim + 1);
    platform.clear('.');
    platform.set_border_color(Some('#'));

    let n_lines = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        let y = n_lines - y;
        for (x, c) in line.chars().enumerate() {
            if c == '#' || c == 'O' {
                platform[(x, y)] = c;
            }
        }
    }

    if log_enabled!(Info) {
        platform.print(|_x, _y, c| *c);
    }

    let mut load = 0;

    let dim = dim as i32;
    for y in (1..=dim).rev() {
        for x in platform.range_x() {
            if platform[(x, y)] == 'O' {
                let mut yy = y;

                // While we can, roll north
                while platform[(x, yy + 1)] == '.' {
                    // info!("Rolling 'O' ({x}, {y}) up 1");
                    platform[(x, yy)] = '.';
                    platform[(x, yy + 1)] = 'O';

                    yy += 1;
                }

                load += yy as i64;
            }
        }
    }

    if log_enabled!(Info) {
        platform.print(|_x, _y, c| *c);
    }

    load
}

// Part2 ========================================================================
#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
    let dim = if cfg!(test) { 10 } else { 100 };
    let mut platform = Framebuffer::new(dim + 1, dim + 1);
    platform.clear('.');
    platform.set_border_color(Some('#'));

    let mut rocks = vec![];
    let n_lines = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        let y = n_lines - y;
        for (x, c) in line.chars().enumerate() {
            if c == '#' || c == 'O' {
                platform[(x, y)] = c;

                if c == 'O' {
                    rocks.push((x as i32, y as i32));
                }
            }
        }
    }

    if log_enabled!(Info) {
        platform.print(|_x, _y, c| *c);
    }

    // const TIMES: usize = 1_000;
    // const TIMES: usize = 1_000_000;
    const TIMES: usize = 1_000_000_000;
    let bar = indicatif::ProgressBar::new(TIMES as _);

    for (i, dir) in [
        (0, 1),  // north
        (-1, 0), // west
        (0, -1), // south
        (1, 0),  // east
    ]
    .iter()
    .cycle()
    .take(TIMES)
    .enumerate()
    {
        if i % 1024 == 0 {
            bar.inc(1024);
        }

        for (x, y) in rocks.iter().copied() {
            if platform[(x, y)] == 'O' {
                let mut xx = x;
                let mut yy = y;

                // While we can, roll <dir>
                while platform[(xx + dir.0, yy + dir.1)] == '.' {
                    xx += dir.0;
                    yy += dir.1;
                }

                if (x != xx) || (y != yy) {
                    platform[(x, y)] = '.';
                    platform[(xx, yy)] = 'O';
                }
            }
        }
    }
    bar.finish();

    if log_enabled!(Info) {
        platform.print(|_x, _y, c| *c);
    }

    let mut load = 0;

    for y in platform.range_y() {
        for x in platform.range_x() {
            if platform[(x, y)] == 'O' {
                load += y as i64;
            }
        }
    }

    load
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[rstest]
    #[case::given(136, EXAMPLE_INPUT)]
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
    #[case::given(64, EXAMPLE_INPUT)]
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
