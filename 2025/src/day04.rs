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
    let mut map: Framebuffer<char> = Framebuffer::parse_grid_char(input);
    map.set_border_color(Some('.'));
    if cfg!(test) {
        println!("Initial State");
        map.just_print();
    }

    let mut next: Framebuffer<char> = Framebuffer::new_matching_size(&map);
    next.set_border_color(Some('.'));

    let mut total = 0;

    loop {
        next.clear('.');

        let mut removed = 0;

        for here in map.iter_coords() {
            if map[here] != '@' {
                continue;
            }

            if IVec2::new(here.0, here.1)
                .full_neighbors()
                .into_iter()
                .filter(|p| map[p] == '@')
                .count()
                < 4
            {
                next[here] = '.';
                removed += 1;
            } else {
                next[here] = '@';
            }
        }

        std::mem::swap(&mut map, &mut next);
        if cfg!(test) {
            println!("Removed {removed} rolls");
            map.just_print();
            println!();
        }

        if removed == 0 {
            break;
        }
        total += removed;
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
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
