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

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use Resource::*;

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
                let idx = match next {
                    Ore => 0,
                    Clay => 1,
                    Obsidian => 2,
                    Geode => 3,
                };
                pending_robot = Some(idx);
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

// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(blueprints: &[Blueprint]) -> usize {
    let known_good_build = [Clay, Clay, Clay, Obsidian, Clay, Obsidian, Geode, Geode];

    blueprints
        .iter()
        .map(|bp| compute_quality_level(bp, &known_good_build))
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
