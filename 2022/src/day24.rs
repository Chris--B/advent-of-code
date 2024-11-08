use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Day24 {
    #[allow(dead_code)]
    storms: Vec<Storm>,
    all_storms: Vec<HashSet<IVec2>>,

    max: IVec2,
}

impl Day24 {
    fn is_storm_at(&self, pt: IVec2, z: i32) -> bool {
        debug_assert!(z >= 0);
        let z = z as usize % self.all_storms.len();
        self.all_storms[z].contains(&pt)
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

    let mut all_storms = vec![];

    // Storms are predictable so the dimensions dictate the cycle length
    let cycle_len = num::integer::lcm(max.y - 1, max.x - 1);

    for _ in 0..cycle_len {
        let mut this_storm = HashSet::new();
        for storm in &storms {
            this_storm.insert(storm.pos);
        }
        all_storms.push(this_storm);

        step(&mut storms, max);
    }

    info!("Tracking {} storms", storms.len());
    info!("[Note] Storms cycle every {} steps", all_storms.len());

    Day24 {
        storms,
        all_storms,
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

fn step(storms: &mut [Storm], max: IVec2) {
    for storm in storms {
        storm.pos += storm.dir;

        // Storms only move NSEW, so only one of these can ever happen per step
        if storm.pos.x == 0 {
            storm.pos.x = max.x - 1;
        } else if storm.pos.x == max.x {
            storm.pos.x = 1;
        } else if storm.pos.y == 0 {
            storm.pos.y = max.y - 1;
        } else if storm.pos.y == max.y {
            storm.pos.y = 1;
        }
    }
}

pub fn find_path(
    day: &Day24,
    start: impl Into<IVec2>,
    end: impl Into<IVec2>,
    z_offset: i32,
) -> HashMap<IVec3, i32> {
    let start = start.into();
    let end = end.into();

    let mut dist_map: HashMap<IVec3, i32> = HashMap::new();
    dist_map.insert(IVec3::new(start.x, start.y, 0), 0);

    let mut points_to_explore_from = VecDeque::new();
    points_to_explore_from.push_back(IVec3::new(start.x, start.y, 0));

    while let Some(prev) = points_to_explore_from.pop_front() {
        info!("Exploring {prev:?} dist={}", dist_map[&prev]);

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
            if (here.xy() != start)
                && (here.xy() != end)
                && (here.x <= 0 || here.x >= day.max.x || here.y <= 0 || here.y >= day.max.y)
            {
                info!("    [{i}/5] Can't explore {here:?} due to walls");
                continue;
            }

            // If we would land in a storm, we can't explore here
            if day.is_storm_at(here.xy(), here.z + z_offset) {
                info!("    [{i}/5] Can't explore {here:?} due to storms");
                continue;
            }

            // TODO: Improve this
            if here.z as usize > (5 * day.all_storms.len()) {
                info!("    [{i}/5] Giving up waiting at {here:?} since it's been too long");
                continue;
            }

            // Distance the last path took to get here.
            // If no one has been here, we'll use a huge distance value and
            // our new distance (which exists!) will win out.
            let new_dist = dist_map[&prev] + 1;

            let here_dist = dist_map.entry(here).or_insert(i32::MAX);
            let old_dist = *here_dist;

            // If we found a better way to get here, continue exploring!
            if new_dist < old_dist {
                *here_dist = new_dist;
                points_to_explore_from.push_back(here);
            }
        }
    }

    dist_map
}

// Part1 ========================================================================
#[aoc(day24, part1)]
pub fn part1(input: &str) -> i32 {
    let day = parse(input);

    let start: IVec2 = (1, 0).into();
    let end: IVec2 = (day.max.x - 1, day.max.y).into();

    let dist_map = find_path(&day, start, end, 0);

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
                debug!("{k:?}");
            }
        })
        .min_by_key(|k| k.z)
        .unwrap()
        .z
}

// Part2 ========================================================================
#[aoc(day24, part2)]
pub fn part2(input: &str) -> i32 {
    let day = parse(input);

    let start: IVec2 = (1, 0).into();
    let end: IVec2 = (day.max.x - 1, day.max.y).into();

    // Map all the points from the start to the end once
    let dist_map_1 = find_path(&day, start, end, 0);
    let z_offset_1 = dist_map_1
        .iter()
        .filter_map(|(k, _v)| {
            if k.x == day.max.x - 1 && k.y == day.max.y {
                Some(k)
            } else {
                None
            }
        })
        .min_by_key(|k| k.z)
        .unwrap()
        .z;

    let dist_map_2 = find_path(&day, end, start, z_offset_1);
    let z_offset_2 = dist_map_2
        .iter()
        .filter_map(|(k, _v)| if k.x == 1 && k.y == 0 { Some(k) } else { None })
        .min_by_key(|k| k.z)
        .unwrap()
        .z;

    let dist_map_3 = find_path(&day, start, end, z_offset_1 + z_offset_2);
    let z_offset_3 = dist_map_3
        .iter()
        .filter_map(|(k, _v)| {
            if k.x == day.max.x - 1 && k.y == day.max.y {
                Some(k)
            } else {
                None
            }
        })
        .min_by_key(|k| k.z)
        .unwrap()
        .z;

    z_offset_1 + z_offset_2 + z_offset_3
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
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
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(54, EXAMPLE_INPUT_COMPLEX)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
