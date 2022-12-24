use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Storm {
    pos: IVec2,
    dir: IVec2,
}

impl Storm {
    fn new(pos: impl Into<IVec2>, dir: impl Into<IVec2>) -> Self {
        Self {
            pos: pos.into(),
            dir: dir.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Day24 {
    storms: Vec<Storm>,

    max: IVec2,
}

fn parse(s: &str) -> Day24 {
    let mut storms = vec![];
    let mut y = 0_i32;

    let mut max = IVec2::zero();

    for line in s.lines() {
        let mut x = -1_i32;
        for b in line.as_bytes().iter().copied() {
            x += 1;

            let storm = match b {
                b'.' | b'#' => continue,

                b'<' => Storm::new((x, y), (-1, 0)),
                b'>' => Storm::new((x, y), (1, 0)),
                b'^' => Storm::new((x, y), (0, -1)),
                b'v' => Storm::new((x, y), (0, 1)),

                _ => unreachable!("Unexpected character found in map: {}", b as char),
            };
            storms.push(storm);
        }

        max = max.max_by_component((x, y).into());
        y += 1;
    }

    Day24 { storms, max }
}

fn print_storms(day: &Day24) {
    for x in 0..=day.max.x {
        if x != 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();

    for y in 1..day.max.y {
        print!("#");
        for x in 1..day.max.x {
            let here = IVec2::new(x, y);

            let n_storms = day.storms.iter().filter(|s| s.pos == here).count();

            match n_storms {
                0 => print!("."),
                1 => {
                    let storm = day.storms.iter().find(|s| s.pos == here).unwrap();
                    match storm.dir.as_array() {
                        [-1, 0] => print!("<"),
                        [1, 0] => print!(">"),
                        [0, -1] => print!("^"),
                        [0, 1] => print!("v"),
                        _ => unreachable!("Unexpected storm direction"),
                    };
                }
                2..=9 => print!("{n_storms}"),
                _ => print!("@"), // that's a lot of storms
            }
        }
        println!("#");
    }

    for x in 0..=day.max.x {
        if x != (day.max.x - 1) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
    println!();
}

fn step(day: &mut Day24) {
    for storm in &mut day.storms {
        storm.pos += storm.dir;

        // Storms only move NSEW, so only one of these can ever happen per step
        if storm.pos.x == 0 {
            storm.pos.x = day.max.x - 1;
        } else if storm.pos.x == day.max.x {
            storm.pos.x = 1;
        } else if storm.pos.y == 0 {
            storm.pos.y = day.max.y - 1;
        } else if storm.pos.y == day.max.y {
            storm.pos.y = 1;
        }
    }
}

// Part1 ========================================================================
#[aoc(day24, part1)]
pub fn part1(input: &str) -> i64 {
    let mut day = parse(input);
    println!("Tracking {} storms", day.storms.len());

    let mut states = vec![];
    while states.is_empty() || states[0] != day.storms {
        states.push(day.storms.clone());
        if cfg!(test) {
            print_storms(&day);
        }
        step(&mut day);
    }
    println!("[Note] Storms cycle every {} steps", states.len());

    // TODO: 3d search over x, y, and states

    0
}

// Part2 ========================================================================
// #[aoc(day24, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_TINY: &str = r"
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
";

    const EXAMPLE_INPUT_COMPLEX: &str = r"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#

";

    #[rstest]
    #[case::given(18, EXAMPLE_INPUT_COMPLEX)]
    #[case::given_tiny(-1, EXAMPLE_INPUT_TINY)] // No answer given, using for testing
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

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
