#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    let mut guard = (IVec2::zero(), Norð);
    let mut map = Framebuffer::parse_grid2(input, |info| {
        if info.c == '^' {
            guard.0 = IVec2::new(info.x, info.y);
            'X'
        } else {
            info.c
        }
    });

    loop {
        let next = guard.0 + guard.1.into();
        if let Some(c) = map.get_mut(next.x as isize, next.y as isize) {
            if *c == '.' || *c == 'X' {
                guard.0 = next;
                *c = 'X';
                continue;
            } else {
                guard.1 = guard.1.turn_right();
                continue;
            }
        } else {
            break;
        }
    }

    if cfg!(test) {
        map.just_print();

        use image::Rgb;
        let img = map.make_image(2, |c| match *c {
            '.' => Rgb([0_u8, 0, 0]),
            '#' => Rgb([0x80, 0x80, 0x80]),
            'X' => Rgb([0xFF, 0x0, 0xFF]),
            _ => unreachable!("Unexpected character: {c}"),
        });
        img.save("target/day6.png").unwrap();
    }

    map.counts()[&'X'] as i32
}

// Part2 ========================================================================
#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[rstest]
    #[case::given(41, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(6, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
