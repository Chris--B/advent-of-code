#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    let mut map = Framebuffer::parse_grid_char(input);
    map.set_border_color(Some('.'));

    let mut count = 0;
    for (x, y) in map.iter_coords() {
        if map[(x, y)] != '@' {
            continue;
        }
        if IVec2::new(x, y)
            .full_neighbors()
            .into_iter()
            .filter(|p| map[p] == '@')
            .count()
            < 4
        {
            count += 1;
        }
    }

    count
}

// Part2 ========================================================================
#[aoc(day4, part2)]
pub fn part2(input: &str) -> i64 {
    let mut map: Framebuffer<_> = Framebuffer::parse_grid_u8(input);
    map.set_border_color(Some(b'.'));

    let mut total = 0;

    loop {
        let mut removed = 0;
        for here @ (x, y) in map.iter_coords() {
            if map[here] != b'@' {
                continue;
            }

            let neighor_rolls = IVec2::new(x, y)
                .full_neighbors()
                .into_iter()
                .filter(|p| map[p] == b'@')
                .count();

            // Note: Typically, cellular automata like this need to double buffer to read the right state.
            // However, we can skip this because the only state we care about is neighbor counts, and it's monotonicly decreasing.
            if neighor_rolls < 4 {
                map[here] = b'.';
                removed += 1;
            } else {
                map[here] = b'@';
            }
        }

        if removed == 0 {
            break;
        }
        total += removed;
    }

    total
}

#[aoc(day4, part2, flat)]
pub fn part2_flat(input: &str) -> i64 {
    let mut map: Vec<u8> = input.as_bytes().to_vec();

    let width: usize = memchr(b'\n', &map).expect("No newline?");
    let n = width * (width + 1) - 1; // include newline, but not a trailing newline

    let down = Wrapping(width + 1);
    let right = Wrapping(1);
    let up = Wrapping(usize::MAX) - down + Wrapping(1);
    let left = Wrapping(usize::MAX) - right + Wrapping(1);
    let dirs: [Wrapping<usize>; 8] = [
        up + left,
        up,
        up + right,
        left,
        right,
        down + left,
        down,
        down + right,
    ];

    if cfg!(test) {
        println!("Initial State");
        let map = Framebuffer::parse_grid_char(just_str(&map));
        map.just_print();
        println!();
    }

    let mut total = 0;
    loop {
        let mut removed = 0;

        for i in 0..n {
            if map[i] != b'@' {
                continue;
            }

            let i: Wrapping<usize> = Wrapping(i);
            let mut count = 0;
            for dir in dirs {
                if map.get((i + dir).0) == Some(&b'@') {
                    count += 1;
                    if count >= 4 {
                        break;
                    }
                }
            }

            if count < 4 {
                map[i.0] = b'#';
                removed += 1;
            }
        }

        if cfg!(test) {
            println!("Removed {removed} rolls");
            let map = Framebuffer::parse_grid_char(just_str(&map));
            map.just_print();
            println!();
        }

        if removed == 0 {
            break;
        }
        total += removed;
    }

    if cfg!(test) {
        println!("Final State");
        let map = Framebuffer::parse_grid_char(just_str(&map));
        map.just_print();
        println!();
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[rstest]
    #[case::given(13, EXAMPLE_INPUT)]
    #[trace]
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

    #[rstest]
    #[case::given(43, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_flat)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
