#![allow(clippy::needless_range_loop)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Blueprint {
    // Each recipe in order: ORE, CLAY, OBSIDIAN, GEODE
    ore: [u8; 4],
    clay: [u8; 4],
    obsidian: [u8; 4],
    geode: [u8; 4],
}

// const R_NAMES: [&str; 4] = ["ore", "clay", "obsidian", "geode"];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
use Resource::*;

impl Resource {
    const fn index(&self) -> usize {
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
        let x = scan_fmt!(recipes[0], "Each ore robot costs {} ore", u8).unwrap();
        // ORE, CLAY, OBSIDIAN, GEODE
        [x, 0, 0, 0]
    };

    let clay = {
        let x = scan_fmt!(recipes[1], "Each clay robot costs {} ore", u8).unwrap();
        [x, 0, 0, 0]
    };

    let obsidian = {
        let (x, y) = scan_fmt!(
            recipes[2],
            "Each obsidian robot costs {} ore and {} clay",
            u8,
            u8
        )
        .unwrap();
        [x, y, 0, 0]
    };

    let geode = {
        let (x, z) = scan_fmt!(
            recipes[3],
            "Each geode robot costs {} ore and {} obsidian",
            u8,
            u8
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
pub fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(parse_blueprint).collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct SimState {
    max_minute: u8,
    minute: u8,
    bank: [u8; 4],
    bots: [u8; 4],
    bp: Blueprint,
}

impl SimState {
    fn new(bp: &Blueprint, max_minute: u8) -> Self {
        SimState {
            max_minute,
            minute: 1,
            bank: [0; 4],
            bots: [1, 0, 0, 0],
            bp: *bp,
        }
    }

    fn can_afford(&self, resource: Resource) -> bool {
        let cost = match resource {
            Ore => &self.bp.ore,
            Clay => &self.bp.clay,
            Obsidian => &self.bp.obsidian,
            Geode => &self.bp.geode,
        };

        for i in 0..4 {
            if self.bank[i] < cost[i] {
                return false;
            }
        }

        true
    }

    fn step_until(&mut self, resource: Resource) -> bool {
        // Sanity check if we can EVER afford this
        let cost = match resource {
            Ore => &self.bp.ore,
            Clay => &self.bp.clay,
            Obsidian => &self.bp.obsidian,
            Geode => &self.bp.geode,
        };

        for i in 0..4 {
            if (self.bots[i] == 0) && (cost[i] != 0) {
                return false;
            }
        }

        // Simulate until we can buy the resource
        while self.minute <= self.max_minute {
            if self.step(resource) {
                return true;
            }
        }

        // Or if we can't, let this path terminate.
        // There's at least one Blueprint where this is the right answer!
        true
    }

    /// Returns true if a bot was built. If it cannot afford one, it still steps but waits instead.
    fn step(&mut self, resource: Resource) -> bool {
        debug_assert!(self.minute <= self.max_minute);

        let mut built_bot = false;
        let mut pending_robot = None;

        if self.can_afford(resource) {
            // Start constructing a new bot if we can build it yet
            let cost = match resource {
                Ore => &self.bp.ore,
                Clay => &self.bp.clay,
                Obsidian => &self.bp.obsidian,
                Geode => &self.bp.geode,
            };

            // Pay for the bot
            for i in 0..4 {
                self.bank[i] -= cost[i];
            }

            // And finally, the construction
            pending_robot = Some(resource.index());
            built_bot = true;
        }

        // Mine the resources
        for i in 0..4 {
            self.bank[i] += self.bots[i];
        }

        // Our new construction finished!
        if let Some(idx) = pending_robot {
            self.bots[idx] += 1;
        }

        self.minute += 1;

        built_bot
    }
}

fn find_best_build(bp: &Blueprint, max_minute: u8) -> usize {
    // Never build more bots than we can spend in a single minute
    let max_ore_bots = [
        bp.clay[Ore.index()],
        bp.obsidian[Ore.index()],
        bp.geode[Ore.index()],
    ]
    .into_iter()
    .max()
    .unwrap();
    let max_clay_bots = bp.obsidian[Clay.index()];
    let max_obsidian_bots = bp.geode[Obsidian.index()];

    let mut best_ql = 0;

    let mut queue: VecDeque<SimState> = VecDeque::new();
    queue.push_back(SimState::new(bp, max_minute));

    while let Some(sim) = queue.pop_front() {
        if sim.minute > max_minute {
            best_ql = best_ql.max(sim.bank[Geode.index()]);
            continue;
        }

        // Continue searching...

        if sim.bots[Ore.index()] < max_ore_bots {
            let mut sim = sim;
            if sim.step_until(Ore) {
                // We were able to buy it, so continue this path
                queue.push_back(sim);
            }
        }

        if sim.bots[Clay.index()] < max_clay_bots {
            let mut sim = sim;
            if sim.step_until(Clay) {
                // We were able to buy it, so continue this path
                queue.push_back(sim);
            }
        }

        if sim.bots[Obsidian.index()] < max_obsidian_bots {
            let mut sim = sim;
            if sim.step_until(Obsidian) {
                // We were able to buy it, so continue this path
                queue.push_back(sim);
            }
        }

        // Always try to make a Geode! 🤑
        {
            let mut sim = sim;
            if sim.step_until(Geode) {
                // We were able to buy it, so continue this path
                queue.push_back(sim);
            }
        }
    }

    best_ql as usize
}
// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|bp| find_best_build(bp, 24))
        .enumerate()
        .map(|(i, ql)| (i + 1) * ql)
        .sum()
}

// Part2 ========================================================================
#[aoc(day19, part2)]
pub fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|bp| find_best_build(bp, 32))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
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

    const PROBLEM_INPUT: &str = r"
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian.
";

    #[rstest]
    #[case::given(33, EXAMPLE_INPUT)]
    #[case::blueprint_1(9, EXAMPLE_INPUT_BP1)]
    #[case::blueprint_2(12, EXAMPLE_INPUT_BP2)]
    #[case::problem(13, PROBLEM_INPUT)]
    #[trace]
    #[timeout(LONG_TIMEOUT)]
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

    // All of these tests are comically slow for some reason - the real input is faster (?!?!)
    /*
    #[rstest]
    #[case::blueprint_1(56, EXAMPLE_INPUT_BP1)]
    #[case::blueprint_2(62, EXAMPLE_INPUT_BP2)]
    #[case::problem(67, PROBLEM_INPUT)]
    #[trace]
    // #[timeout(LONG_TIMEOUT)]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&[Blueprint]) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse(input)), expected);
    }
    */
}
