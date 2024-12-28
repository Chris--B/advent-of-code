#![allow(unused)]

use crate::prelude::*;

fn solve_with_cheat_quota(
    map: &Framebuffer<char>,
    start: IVec2,
    quota: i32,
    min_savings: i32,
) -> i64 {
    #![allow(clippy::needless_range_loop)]

    let mut path = vec![start];
    let mut i = 0;
    'search: loop {
        let curr = path[i];
        i += 1;

        for next in curr.neighbors() {
            if i > 2 && path[i - 2] == next {
                continue;
            }

            match map[next] {
                '#' => continue,
                '.' => {
                    path.push(next);
                    break;
                }
                'S' => unreachable!(),
                'E' => {
                    path.push(next);
                    break 'search;
                }
                c => unreachable!("Unrecognized map character {c}"),
            }
        }
    }

    let mut count = 0;

    for bi in 0..path.len() {
        // B is strictly after A
        let b = path[bi];
        for ai in 0..bi {
            let a = path[ai];

            let dist: i32 = (a - b).abs().as_array().iter().sum();
            if !(0 < dist && dist <= quota) {
                continue;
            }
            let bi = bi as i32;
            let ai = ai as i32;

            let savings = bi - (ai + dist);
            debug_assert!(savings >= 0);

            if savings >= min_savings {
                count += 1;
            }
        }
    }

    count
}

// Part1 ========================================================================
#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    let mut start = IVec2::zero();
    let map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| {
        match info.c {
            '#' => {}
            '.' => {}
            'S' => start = IVec2::new(info.x, info.y),
            // 'E' => end = IVec2::new(info.x, info.y),
            'E' => {}
            c => unreachable!("Unexpected map character: {c:?}"),
        }
        info.c
    });

    solve_with_cheat_quota(&map, start, 2, 100)
}

// Part2 ========================================================================
#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    let mut start = IVec2::zero();
    let map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| {
        match info.c {
            '#' => {}
            '.' => {}
            'S' => start = IVec2::new(info.x, info.y),
            // 'E' => end = IVec2::new(info.x, info.y),
            'E' => {}
            c => unreachable!("Unexpected map character: {c:?}"),
        }
        info.c
    });

    solve_with_cheat_quota(&map, start, 20, 100)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[rstest]
    #[case::given_minsavings_20(5, 20_i32, EXAMPLE_INPUT)]
    #[case::given_minsavings_64(1, 64_i32, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(750))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] min_savings: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        let mut start = IVec2::zero();
        let map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| {
            match info.c {
                '#' => {}
                '.' => {}
                'S' => start = IVec2::new(info.x, info.y),
                // 'E' => end = IVec2::new(info.x, info.y),
                'E' => {}
                c => unreachable!("Unexpected map character: {c:?}"),
            }
            info.c
        });

        let quota = 2;
        assert_eq!(
            solve_with_cheat_quota(&map, start, quota, min_savings),
            expected
        );
    }

    /*
        On Example input:
            There are 32 cheats that save 50 picoseconds.
            There are 31 cheats that save 52 picoseconds.
            There are 29 cheats that save 54 picoseconds.
            There are 39 cheats that save 56 picoseconds.
            There are 25 cheats that save 58 picoseconds.
            There are 23 cheats that save 60 picoseconds.
            There are 20 cheats that save 62 picoseconds.
            There are 19 cheats that save 64 picoseconds.
            There are 12 cheats that save 66 picoseconds.
            There are 14 cheats that save 68 picoseconds.
            There are 12 cheats that save 70 picoseconds.
            There are 22 cheats that save 72 picoseconds.
            There are  4 cheats that save 74 picoseconds.
            There are  3 cheats that save 76 picoseconds.
    */
    #[rstest]
    #[case::given_minsavings_50((32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3), 50_i32, EXAMPLE_INPUT)]
    #[case::given_minsavings_64((19 + 12 + 14 + 12 + 22 + 4 + 3), 64_i32, EXAMPLE_INPUT)]
    #[trace]
    // #[ignore]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] min_savings: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        let mut start = IVec2::zero();
        let map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| {
            match info.c {
                '#' => {}
                '.' => {}
                'S' => start = IVec2::new(info.x, info.y),
                // 'E' => end = IVec2::new(info.x, info.y),
                'E' => {}
                c => unreachable!("Unexpected map character: {c:?}"),
            }
            info.c
        });

        let quota = 20;
        assert_eq!(
            solve_with_cheat_quota(&map, start, quota, min_savings),
            expected
        );
    }
}
