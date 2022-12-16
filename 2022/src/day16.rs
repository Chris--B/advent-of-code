use crate::prelude::*;

use std::fmt;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Name([char; 2]);

impl Name {
    fn new(s: &str) -> Self {
        Self(iter_to_array(s.chars().take(2)))
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}{}\"", self.0[0], self.0[1])
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Valve {
    open: bool,
    rate: i32,
}

#[derive(Clone, Debug)]
struct Day16 {
    // Whether a valve is open or not
    valves: HashMap<Name, Valve>,

    // Connections between tunnels
    tunnels: HashMap<Name, Vec<Name>>,
}

fn parse(s: &str) -> Day16 {
    let mut valves = HashMap::new();
    let mut tunnels = HashMap::new();

    for line in s.lines() {
        // Example:
        //      Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

        // Skip "Valve "
        let line = &line[6..];

        // Pars valve
        let valve_name = Name::new(line);

        // Skip "AA has flow rate="
        let line = &line[17..];
        let [rate_text, line] = iter_to_array(line.split(';'));
        let rate: i32 = rate_text.parse().unwrap();

        // Skip " tunnels lead to valves "
        // OR   " tunnels lead to valve "
        let tunnel_text = &line[23..].trim();
        let these_tunnels: Vec<Name> = tunnel_text.split(", ").map(Name::new).collect();

        valves.insert(valve_name, Valve { open: false, rate });
        tunnels.insert(valve_name, these_tunnels);
    }

    Day16 { valves, tunnels }
}

// Part1 ========================================================================
#[aoc(day16, part1)]
pub fn part1(input: &str) -> i64 {
    let mut day = parse(input);
    let mut here = Name::new("AA");

    day.valves.get_mut(&here).unwrap().open = true;

    // Core idea:
    //  List all valves by pressure
    //  Reorder according to tunnel deps (e.g. JJ=20, but we need to go to II first)
    //  Simulate Move/Open steps (1 min to move, 1 min to open)
    // ... is this a minimal spanning tree?

    let mut all_pressure = 0_i32;

    for m in 1..=30 {
        println!("== Minute {m} ==");

        let open = day
            .valves
            .iter()
            .filter(|v| v.1.open)
            .filter(|v| v.0 .0 != ['A', 'A']);
        if open.clone().count() > 0 {
            print!("Valves ");
            for (name, _valve) in open.clone() {
                print!("{name:?}, ");
            }

            let pressure: i32 = open.clone().map(|v| v.1.rate).sum();
            println!(" are open, releasing {pressure} pressure");
        } else {
            println!("No valves are open.");
        }
        all_pressure += open.map(|v| v.1.rate).sum::<i32>();

        if day.valves[&here].open {
            // Move rooms
            // Who knows what's best, let's just be greedy.
            let mut options = day.tunnels[&here].clone();
            options.sort_by_key(|n| {
                if !day.valves[n].open {
                    -day.valves[n].rate
                } else {
                    i32::MAX
                }
            });

            here = options[0];
            println!("You move to valve {here:?}");
        } else {
            // Open this one
            day.valves.get_mut(&here).unwrap().open = true;
            println!("You open valve {here:?}");
        }

        println!();
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

    const EXAMPLE_INPUT: &str = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
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

    #[rstest]
    #[case::given(1651, EXAMPLE_INPUT)]
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
