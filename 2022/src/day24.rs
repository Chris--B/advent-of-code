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
pub struct Day24 {
    storms: Vec<Storm>,
    all_storms: Vec<Vec<Storm>>,

    max: IVec2,
}

impl Day24 {
    fn storm_at(&self, z: i32) -> &[Storm] {
        debug_assert!(z >= 0);
        let z = z as usize % self.all_storms.len();
        &self.all_storms[z]
    }
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

    Day24 {
        storms,
        all_storms: vec![],
        max,
    }
}

fn _print_storms(day: &Day24) {
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

pub fn find_path(
    day: &Day24,
    start: impl Into<IVec3>,
    end: impl Into<IVec2>,
) -> HashMap<IVec3, i64> {
    let start = start.into();
    let end = end.into();
    let mut dist_map = HashMap::new();
    dist_map.insert(start, 0);

    let mut points_to_explore_from: Vec<IVec3> = Vec::new();
    points_to_explore_from.push(start);

    while let Some(prev) = points_to_explore_from.pop() {
        // println!("Exploring {prev:?} dist={}", dist_map[&prev]);

        // Check in all directions for low distance paths
        for (i, dir) in [
            // East or West
            (1, 0, 1),
            (-1, 0, 1),
            // North or South
            (0, -1, 1),
            (0, 1, 1),
            // Wait in place
            (0, 0, 1),
        ]
        .into_iter()
        .enumerate()
        {
            let i = i + 1;
            let dir: IVec3 = dir.into();
            let here = prev + dir;

            // If this direction puts us out of bounds, skip it
            if (here != start)
                && (here.xy() != end)
                && (here.x <= 0 || here.x >= day.max.x || here.y <= 0 || here.y >= day.max.y)
            {
                if cfg!(test) {
                    println!("    [{i}/5] Can't explore {here:?} due to walls");
                }
                continue;
            }

            // If we would land in a storm, we can't explore here
            if day
                .storm_at(here.z)
                .iter()
                .map(|s| s.pos)
                .contains(&here.xy())
            {
                if cfg!(test) {
                    println!("    [{i}/5] Can't explore {here:?} due to storms");
                }
                continue;
            }

            // TODO: Improve this
            if here.z as usize > (5 * day.all_storms.len()) {
                if cfg!(test) {
                    println!("    [{i}/5] Giving up waiting at {here:?} since it's been too long");
                }
                continue;
            }

            // Distance the last path took to get here.
            // If no one has been here, we'll use a huge distance value and
            // our new distance (which exists!) will win out.
            let new_dist = dist_map[&prev] + 1;

            let here_dist = dist_map.entry(here).or_insert(i64::MAX);
            let old_dist = *here_dist;

            // If we found a better way to get here, continue exploring!
            if new_dist < old_dist {
                *here_dist = new_dist;
                points_to_explore_from.push(here);
            }
        }
    }

    dist_map
}

// Part1 ========================================================================
#[aoc(day24, part1)]
pub fn part1(input: &str) -> i64 {
    let mut day = parse(input);
    println!("Tracking {} storms", day.storms.len());

    while day.all_storms.is_empty() || day.all_storms[0] != day.storms {
        day.all_storms.push(day.storms.clone());
        if cfg!(test) {
            // _print_storms(&day);
        }
        step(&mut day);
    }
    println!("[Note] Storms cycle every {} steps", day.all_storms.len());

    let dist_map = find_path(&day, (1, 0, 0), (day.max.x - 1, day.max.y));

    if cfg!(test) {
        for y in 0..=day.max.y {
            print!("#");
            for x in 1..day.max.x {
                let here = IVec2::new(x, y);

                if dist_map.keys().filter(|xyz| xyz.xy() == here).count() > 1 {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }
        println!();
    }

    dist_map
        .iter()
        .filter_map(|(k, _v)| {
            if k.x == day.max.x - 1 && k.y == day.max.y {
                Some(k)
            } else {
                None
            }
        })
        .inspect(|k| {
            if cfg!(test) {
                println!("{k:?}");
            }
        })
        .min_by_key(|k| k.z)
        .unwrap()
        .z as i64
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

    const _EXAMPLE_INPUT_TINY: &str = r"
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
    #[case::given_tiny(10, _EXAMPLE_INPUT_TINY)]
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
