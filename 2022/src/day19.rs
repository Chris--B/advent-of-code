#![allow(clippy::collapsible_if)]

use crate::prelude::*;

#[derive(Debug)]
pub struct Blueprint {
    // Each recipe in order: ORE, CLAY, OBSIDIAN, GEODE
    ore: [i32; 4],
    clay: [i32; 4],
    obsidian: [i32; 4],
    geode: [i32; 4],
}

// const R_NAMES: [&str; 4] = ["ore", "clay", "obsidian", "geode"];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use Resource::*;

impl Resource {
    fn index(&self) -> usize {
        match *self {
            Ore => 0,
            Clay => 1,
            Obsidian => 2,
            Geode => 3,
        }
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let line = line.split(':').nth(1).unwrap();
    let recipes: [&str; 4] = iter_to_array(line.split('.').map(|s| s.trim()));

    let ore = {
        let x = scan_fmt!(recipes[0], "Each ore robot costs {} ore", i32).unwrap();
        // ORE, CLAY, OBSIDIAN, GEODE
        [x, 0, 0, 0]
    };

    let clay = {
        let x = scan_fmt!(recipes[1], "Each clay robot costs {} ore", i32).unwrap();
        [x, 0, 0, 0]
    };

    let obsidian = {
        let (x, y) = scan_fmt!(
            recipes[2],
            "Each obsidian robot costs {} ore and {} clay",
            i32,
            i32
        )
        .unwrap();
        [x, y, 0, 0]
    };

    let geode = {
        let (x, z) = scan_fmt!(
            recipes[3],
            "Each geode robot costs {} ore and {} obsidian",
            i32,
            i32
        )
        .unwrap();
        [x, 0, z, 0]
    };

    Blueprint {
        ore,
        clay,
        obsidian,
        geode,
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(parse_blueprint).collect()
}

fn try_spend(bank: &mut [i32; 4], cost: &[i32; 4]) -> bool {
    for i in 0..4 {
        if bank[i] < cost[i] {
            return false;
        }
    }

    for i in 0..4 {
        bank[i] -= cost[i]
    }

    true
}

fn compute_quality_level(bp: &Blueprint, mut build_order: &[Resource]) -> usize {
    let mut bank = [0, 0, 0, 0];
    let mut bots = [1, 0, 0, 0];

    for _m in 1..=24 {
        // println!("== Minute {_m} ==");

        // Start constructing a new bot if we can build it yet
        let mut pending_robot = None;
        if let Some(next) = build_order.first() {
            let cost = match next {
                Ore => &bp.ore,
                Clay => &bp.clay,
                Obsidian => &bp.obsidian,
                Geode => &bp.geode,
            };

            // println!("** Trying to build {:?}", next);
            if try_spend(&mut bank, cost) {
                pending_robot = Some(next.index());
                build_order = &build_order[1..];
            }
        }

        // Mine the resources

        for i in 0..4 {
            if bots[i] > 0 {
                bank[i] += bots[i];

                // let have = bank[i];
                // let bot_count = bots[i];
                // let resource = R_NAMES[i];
                // println!("{bot_count} {resource} robot collects {bot_count} {resource}; you now have {have} {resource}.");
            }
        }

        // Our new construction finished!

        if let Some(idx) = pending_robot {
            bots[idx] += 1;

            // let robot = R_NAMES[idx];
            // let count = bots[idx];
            // println!("The new {robot}-collecting robot is ready; you now have {count} of them.");
        }

        // println!();
    }

    bank[3] as usize
}

type Build = SmallVec<[Resource; 24]>;

fn append_and_clone(build: &Build, r: Resource) -> Build {
    let mut build = build.clone();
    build.push(r);
    build
}

// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|bp| {
            println!("{bp:?}");

            let mut best = 0;

            let mut builds = VecDeque::new();
            builds.push_back(smallvec![Ore]);
            builds.push_back(smallvec![Clay]);

            let mut i = 0;
            while let Some(build) = builds.pop_front() {
                i += 1;

                if build.len() >= 8 {
                    let score = compute_quality_level(bp, &build);
                    if score >= best {
                        println!("[{i}] {best} -> {score}: {build:?}");
                    }
                    best = best.max(score);
                }

                // Build new builds
                if build.len() < 10 {
                    let income = build
                        .iter()
                        .map(|r| match r {
                            Ore => bp.ore,
                            Clay => bp.clay,
                            Obsidian => bp.obsidian,
                            Geode => bp.geode,
                        })
                        .fold([0; 4], |mut acc, x| {
                            for i in 0..4 {
                                acc[i] += x[i];
                            }
                            acc
                        });

                    if !try_spend(&mut income.clone(), &bp.ore) {
                        builds.push_back(append_and_clone(&build, Ore));
                    }

                    builds.push_back(append_and_clone(&build, Clay));

                    builds.push_back(append_and_clone(&build, Obsidian));

                    builds.push_back(append_and_clone(&build, Geode));
                }
            }
            println!();

            best
        })
        .enumerate()
        .map(|(id, ql)| (id + 1) * ql)
        .sum()
}

// Part2 ========================================================================
// #[aoc(day19, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    const EXAMPLE_INPUT_BP1: &str = r"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
";

    const EXAMPLE_INPUT_BP2: &str = r"
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[rstest]
    #[case::given(33, EXAMPLE_INPUT)]
    #[case::blueprint_1(9, EXAMPLE_INPUT_BP1)]
    #[case::blueprint_2(12, EXAMPLE_INPUT_BP2)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&[Blueprint]) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse(input)), expected);
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
