#![allow(unused)]

use crate::prelude::*;

fn c_to_dir(c: char) -> Cardinal {
    match c {
        '^' => Norð,
        'v' | 'V' => Souð,
        '>' => East,
        '<' => West,
        _ => unreachable!("Invalid direction character {c:?}"),
    }
}

// Part1 ========================================================================
#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut robot = IVec2::zero();
    let mut map = Framebuffer::parse_grid2(map, |info| {
        if info.c == '@' {
            robot = IVec2::new(info.x, info.y);
            '@'
        } else {
            info.c
        }
    });

    if cfg!(test) {
        println!("Initial State:");
        map.just_print();
        println!("{} moves={moves:?}", moves.len());
    }

    for c in moves.lines().flat_map(str::chars) {
        let dir: IVec2 = c_to_dir(c).into();
        let next = robot + dir;

        assert_eq!(map[robot], '@');
        assert_ne!(map[next], '@');

        if cfg!(test) {
            println!("Move {c}:");
        }
        match map[next] {
            '.' => { /* Easy move, nothing special to do */ }
            '#' => {
                // Nothing happens, do nothing.
                if cfg!(test) {
                    println!("  + BONK! Wall.");
                    println!();
                }
                continue;
            }
            'O' => {
                // We found a box! Let's see if we can move it or not.
                let mut chain = vec![];
                let mut n = next;
                while map[n] == 'O' {
                    chain.push(n);
                    n += dir;
                }
                chain.push(n);
                let end = map[n];
                if end == '.' {
                    // We can move the boxes!
                    for (next, prev) in chain.into_iter().rev().tuple_windows() {
                        map[next] = map[prev];
                    }
                } else {
                    let n = chain.len() - 1;
                    if cfg!(test) {
                        println!("  + BONK! Tried to move {n} boxes, but pushing against {end:?}",);
                        println!();
                    }
                    continue;
                }
            }
            cc => {
                println!("Bad State");
                map.just_print();
                unreachable!("Unexpected map character: {cc}");
            }
        }

        // And move the robot
        map[robot] = '.';
        map[next] = '@';
        robot = next;

        if cfg!(test) {
            map.just_print();
            println!();
        }
    }

    // Compute GPS scores
    let maxy = map.height() as i32;
    map.iter_coords()
        .filter_map(|(x, y)| -> Option<(i64)> {
            if map[(x, y)] == 'O' {
                // Our y axis is inverted from AOC's
                let y = maxy - y - 1;
                Some((x + 100 * y) as i64)
            } else {
                None
            }
        })
        .sum()
}

// Part2 ========================================================================
#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_SMALL: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE_INPUT_BIG: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[rstest]
    #[case::given(2028, EXAMPLE_INPUT_SMALL)]
    #[case::given(10092, EXAMPLE_INPUT_BIG)]
    #[trace]
    #[timeout(Duration::from_millis(100))]
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

    /*
    ##############
    ##...[].##..## GPS = 5,1
    ##...@.[]...## GPS = 7,2
    ##....[]....## GPS = 6,3
     */
    const EXAMPLE_INPUT_P2_SMALL: &str = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[rstest]
    #[case::given((5 + 100*1) + (7 + 100*2) + (6 + 100*3), EXAMPLE_INPUT_P2_SMALL)]
    #[case::given(9021, EXAMPLE_INPUT_BIG)]
    #[trace]
    #[ignore]
    #[timeout(Duration::from_millis(100))]
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
