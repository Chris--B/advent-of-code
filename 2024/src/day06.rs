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
fn loops(map: &Framebuffer<char>, start: (IVec2, Cardinal)) -> bool {
    let mut guard = start;
    let mut history: HashSet<(IVec2, Cardinal)> = [start].into_iter().collect();

    loop {
        let next = guard.0 + guard.1.into();
        if history.contains(&(next, guard.1)) {
            return true;
        }

        if let Some(c) = map.get(next.x as isize, next.y as isize) {
            if *c == '#' || *c == 'O' {
                guard.1 = guard.1.turn_right();
            } else {
                guard.0 = next;

                history.insert(guard);
            }
        } else {
            break;
        }
    }

    false
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    let mut start = (IVec2::zero(), Norð);
    let mut map = Framebuffer::parse_grid2(input, |info| {
        if info.c == '^' {
            start.0 = IVec2::new(info.x, info.y);
        }
        info.c
    });

    let start = start;
    let mut guard = start;

    // Run once to get our history
    let mut history = HashSet::new();
    history.insert(start.0);
    loop {
        let next = guard.0 + guard.1.into();

        if let Some(c) = map.get_mut(next.x as isize, next.y as isize) {
            if *c == '#' || *c == 'O' {
                guard.1 = guard.1.turn_right();
            } else {
                guard.0 = next;
                history.insert(guard.0);
            }
        } else {
            break;
        }
    }

    // And then for each spot we visisted, we'll check if an obstacle there triggers a loop.
    let percent = 100. * (history.len() as f64) / (map.width() * map.height()) as f64;
    println!(
        "Checking {} locations ({percent:.1}% of map)",
        history.len()
    );

    let mut options = 0;
    for step in history {
        // Add our new obstacle
        map[step] = 'O';
        if loops(&map, start) {
            options += 1;
        }
        // And un-add it
        map[step] = '.';
    }

    options
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
