use crate::prelude::*;

use std::fmt;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Name([char; 2]);

impl Name {
    fn new(s: &str) -> Self {
        Self(iter_to_array(s.chars().take(2)))
    }

    fn as_string(&self) -> String {
        self.0.iter().collect()
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}{}\"", self.0[0], self.0[1])
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Valve {
    open: bool,
    rate: i32,
}

impl Valve {
    fn new(rate: i32) -> Self {
        // "All of the valves begin **closed**"
        Self { rate, open: false }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Tunnel {
    dest: Name,
    minutes: u32,
}

impl Tunnel {
    fn new_with_dest(dest: Name) -> Self {
        Self { dest, minutes: 1 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Day16 {
    // Whether a valve is open or not
    valves: HashMap<Name, Valve>,

    // Connections between tunnels
    tunnels: HashMap<Name, Vec<Tunnel>>,
}

impl Day16 {
    fn print(&self) {
        if cfg!(debug_assertions) {
            println!();
            info!(
                "Loaded {n_tunnels} tunnels between {n_valves} valves",
                n_valves = self.valves.len(),
                n_tunnels = self.tunnels.values().map(|ts| ts.len()).sum::<usize>(),
            );

            let mut names: Vec<_> = self.valves.keys().collect();
            names.sort();

            for name in &names {
                println!("{name:?}: {}", self.valves[name].rate);
            }
            for name in &names {
                println!("{name:?} -> {:?}", &self.tunnels[name]);
            }
            println!();
        }
    }
}

fn parse(s: &str) -> Day16 {
    let mut valves = HashMap::new();
    let mut tunnels = HashMap::new();

    for line in s.lines() {
        // Example:
        //      "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"

        // Skip "Valve "
        let line = &line[6..];

        // Parse valve name
        let valve_name = Name::new(line);

        // Skip "AA has flow rate="
        let line = &line[17..];
        let [rate_text, line] = iter_to_array(line.split(';'));
        let rate: i32 = rate_text.parse().unwrap();

        // Skip " tunnels lead to valves "
        // OR   " tunnels lead to valve "
        let tunnel_text = &line[23..].trim();
        let these_tunnels: Vec<Tunnel> = tunnel_text
            .split(", ")
            .map(Name::new)
            .map(Tunnel::new_with_dest)
            .collect();

        valves.insert(valve_name, Valve::new(rate));
        tunnels.insert(valve_name, these_tunnels);
    }

    Day16 { valves, tunnels }
}

// Starting valve name
const AA: Name = Name(['A', 'A']);

// Remove Valves with a flow rate of 0, and instead link things directly
fn simplify_tunnels(day: Day16) -> Day16 {
    let mut new = Day16 {
        valves: day
            .valves
            .iter()
            .filter_map(|(&name, &valve)| {
                if valve.rate != 0 || name == AA {
                    Some((name, valve))
                } else {
                    None
                }
            })
            .collect(),
        tunnels: HashMap::new(),
    };

    // Find all valves that can be reached in either 1 step, or any number of steps across 0-rate valves
    for &name in day.valves.keys() {
        let mut seen = HashSet::new();
        let mut queue: Vec<(Name, u32)> = vec![(name, 0)];

        while let Some((next, mut minutes)) = queue.pop() {
            // TODO: Might need to check if this is a faster path than what we last saw
            if seen.contains(&next) {
                continue;
            }
            seen.insert(next);

            // Visiting any valve cost time
            minutes += 1;

            // See which valves are reachable from here at all
            let mut tunnels = vec![];
            for tunnel in &day.tunnels[&next] {
                let dest = tunnel.dest;
                let cost = day.valves[&tunnel.dest].rate;

                // If the dest has a rate of 0, we need to continue exploring it. This means enqueueing it:
                if cost == 0 {
                    queue.push((tunnel.dest, minutes));
                } else if dest != name {
                    // This is a terminus, so let's mark this tunnel as reachable
                    tunnels.push(Tunnel { dest, minutes });
                }
            }

            tunnels.sort_by_key(|t| t.dest);
            if tunnels.len() > 1 {
                for (&a, &b) in std::iter::zip(&tunnels, &tunnels[1..]) {
                    assert_ne!(a.dest, b.dest, "{:?} shows up twice: {:?}", a.dest, [a, b]);
                }
            }
            new.tunnels.insert(name, tunnels);
        }
    }

    new
}

// Part1 ========================================================================
#[aoc(day16, part1)]
#[allow(unreachable_code, unused)]
pub fn part1(input: &str) -> i64 {
    init_logging();

    let day = parse(input);
    let mut day = simplify_tunnels(day);
    day.print();

    // Start at and open AA
    let mut here: Name = AA;
    day.valves.get_mut(&here).unwrap().open = true;

    // Core idea:
    //  List all valves by pressure
    //  Reorder according to tunnel deps (e.g. JJ=20, but we need to go to II first)
    //  Simulate Move/Open steps (1 min to move, 1 min to open)
    // ... is this a minimal spanning tree?
    let mut all_pressure = 0_i32;

    // Run for 30 minutes
    for m in 1..=30 {
        info!("== Minute {m} ==");

        let open = day
            .valves
            .iter()
            .filter(|(_name, valve)| valve.open)
            .filter(|(name, _valve)| **name != AA);
        if log_enabled!(Info) {
            if open.clone().count() > 0 {
                let names: String = open
                    .clone()
                    .map(|(name, _valve)| name.as_string())
                    .join(", ");

                let pressure: i32 = open.clone().map(|(_name, valve)| valve.rate).sum();
                info!("Valves {names} are open, releasing {pressure} pressure");
            } else {
                info!("No valves are open.");
            }
        }
        all_pressure += open.map(|(_name, valve)| valve.rate).sum::<i32>();

        if day.valves[&here].open {
            // Move rooms
            // Who knows what's best, let's just be greedy.
            let mut options: Vec<Tunnel> = day.tunnels[&here].clone();
            options.sort_by_key(|tunnel| {
                if !day.valves[&tunnel.dest].open {
                    -day.valves[&tunnel.dest].rate
                } else {
                    i32::MAX
                }
            });

            here = options[0].dest;
            info!("You move to valve {here:?}");
        } else {
            // Open this one
            day.valves.get_mut(&here).unwrap().open = true;
            info!("You open valve {here:?}");
        }

        info!("");
    }

    all_pressure as i64
}

// Part2 ========================================================================
// #[aoc(day16, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn check_parse() {
        const AA: Name = Name(['A', 'A']);
        const BB: Name = Name(['B', 'B']);
        const CC: Name = Name(['C', 'C']);
        const DD: Name = Name(['D', 'D']);
        const EE: Name = Name(['E', 'E']);
        const FF: Name = Name(['F', 'F']);
        const GG: Name = Name(['G', 'G']);
        const HH: Name = Name(['H', 'H']);
        const II: Name = Name(['I', 'I']);
        const JJ: Name = Name(['J', 'J']);

        // Note: When comparing these two, HashMap ordering is RANDOM, so if anything is wrong the entire thing gets noisey!
        assert_eq!(
            parse(EXAMPLE_INPUT),
            Day16 {
                valves: [
                    (AA, Valve::new(0)),
                    (BB, Valve::new(13)),
                    (CC, Valve::new(2)),
                    (DD, Valve::new(20)),
                    (EE, Valve::new(3)),
                    (FF, Valve::new(0)),
                    (GG, Valve::new(0)),
                    (HH, Valve::new(22)),
                    (II, Valve::new(0)),
                    (JJ, Valve::new(21)),
                ]
                .into_iter()
                .collect::<HashMap<Name, Valve>>(),

                tunnels: [
                    (AA, vec![DD, II, BB]),
                    (BB, vec![CC, AA]),
                    (CC, vec![DD, BB]),
                    (DD, vec![CC, AA, EE]),
                    (EE, vec![FF, DD]),
                    (FF, vec![EE, GG]),
                    (GG, vec![FF, HH]),
                    (HH, vec![GG]),
                    (II, vec![AA, JJ]),
                    (JJ, vec![II]),
                ]
                .into_iter()
                .map(|(name, tunnel_names)| {
                    (
                        name,
                        tunnel_names
                            .into_iter()
                            .map(Tunnel::new_with_dest)
                            .collect_vec(),
                    )
                })
                .collect::<HashMap<Name, Vec<Tunnel>>>(),
            }
        );
    }

    #[rstest]
    #[case::given(1651, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(EZ_TIMEOUT)]
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
    // #[timeout(EZ_TIMEOUT)]
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
