use crate::prelude::*;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph {
    names: Vec<Name>,
    rates: Vec<i64>,
    tunnels: Vec<u64>,

    // TODO: Fast cache from Floyd-Warshall?
    cache: RefCell<Vec<u8>>,
}

impl Graph {
    /// Parses from Input
    fn new(s: &str) -> Self {
        // O(n) but we only need it for parsing
        fn get_id(name: Name, names: &mut Vec<Name>) -> usize {
            if let Some(id) = names.iter().position(|&n| n == name) {
                id
            } else {
                debug_assert!(names.len() < 63);
                names.push(name);
                names.len() - 1
            }
        }

        let mut names = vec![];
        let mut rates = vec![];
        let mut tunnels = vec![];

        // Make sure AA is always id=0
        let aa_id = get_id(Name::new("AA"), &mut names);
        debug_assert_eq!(aa_id, 0);

        // Eachg line is parsable in isolation
        for line in s.lines() {
            // Example:
            //      "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"

            // Skip "Valve "
            let line = &line[6..];

            // Parse valve
            let valve_name = Name::new(line);
            let id = get_id(valve_name, &mut names);
            if id >= rates.len() {
                rates.resize(id + 1, 0);
                tunnels.resize(id + 1, 0);
            }

            // Skip "AA has flow rate="
            let line = &line[17..];
            let [rate_text, line] = iter_to_array(line.split(';'));
            rates[id] = rate_text.parse().unwrap();

            // Skip " tunnels lead to valves "
            // OR   " tunnels lead to valve "
            let tunnel_text = &line[23..].trim();
            let tunnel: u64 = tunnel_text
                .split(", ")
                .map(Name::new)
                .map(|n| get_id(n, &mut names))
                .fold(0_u64, |acc, id| (acc | (1 << id)));
            debug!("Neighbors of {valve_name}: 0b{tunnel:016b}");
            tunnels[id] = tunnel;
        }

        let cache = vec![u8::MAX; names.len() * names.len()].into();

        Graph {
            names,
            rates,
            tunnels,
            cache,
        }
    }

    /// Recreates the given input to validate that it parsed it right
    #[cfg(test)]
    fn input(&self) -> String {
        debug_assert_eq!(self.names.len(), self.rates.len());
        debug_assert_eq!(self.names.len(), self.tunnels.len());
        info!("names={:?}", self.names);

        let mut lines: Vec<String> = vec![];

        for id in 0..self.names.len() {
            let name = self.names[id];
            let rate = self.rates[id];

            let tunnel_names = self
                .valves_with_mask(self.tunnels[id])
                .map(|dest| self.name_of(dest))
                .collect_vec();
            let plural = tunnel_names.len() != 1;
            let tunnel_info = tunnel_names.into_iter().join(", ");

            let line = if plural {
                format!("Valve {name} has flow rate={rate}; tunnels lead to valves {tunnel_info}")
            } else {
                format!("Valve {name} has flow rate={rate}; tunnel leads to valve {tunnel_info}")
            };
            lines.push(line);
        }

        lines.sort();
        lines.join("\n")
    }

    fn save_dot(&self, dir: &str, opened: Option<u64>) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        use std::io::Write;

        let filename = format!("{dir}/Day16_{}.dot", self.names[0]);
        info!("Saving dot file to {:?}", std::path::absolute(&filename));

        let file = File::create(&filename)?;
        let mut w = BufWriter::new(file);
        writeln!(w, "digraph Day16_{} {{", self.names[0])?;

        for valve in self.valves() {
            // Note: We're bundling the flow rate in with the node
            // dot is better than this, but I am not.
            let name = format!("{}_{:>02}", self.name_of(valve), self.rate_of(valve));

            // Comment per node
            writeln!(
                w,
                "    # {name} 0b{:0b} ({} neighbors)",
                self.tunnels[valve],
                self.tunnels[valve].count_ones()
            )?;

            // Extra per-node stuff
            if let Some(opened) = opened {
                if ((1 << valve) & opened) != 0 {
                    writeln!(w, "    {name} [color=blue];")?;
                }
            }

            // Add an arrow for each tunnel
            for dest in self.valves_with_mask(self.tunnels[valve]) {
                let dest_name = format!("{}_{:>02}", self.name_of(dest), self.rate_of(dest));
                // Main connection
                writeln!(w, "    {name} -> {dest_name};")?;
            }
            writeln!(w)?;
        }

        writeln!(w, "}}")?;
        Ok(())
    }

    #[track_caller]
    fn name_of(&self, id: usize) -> Name {
        self.names[id]
    }

    #[track_caller]
    fn rate_of(&self, id: usize) -> i64 {
        self.rates[id]
    }

    fn valves(&self) -> impl Iterator<Item = usize> {
        0..self.names.len()
    }

    fn valves_with_mask(&self, mask: u64) -> impl Iterator<Item = usize> {
        self.valves().filter(move |v| ((1 << v) & mask) != 0)
    }

    fn visit_cost(&self, a: usize, b: usize) -> u8 {
        // TODO: Fast cache from Floyd-Warshall
        if a == b {
            return 0;
        }

        let mut _cache = self.cache.borrow_mut();
        let best_so_far = &mut _cache[a * self.names.len()..][..self.names.len()];

        let mut queue = vec![(a, 0)];
        while let Some((curr, cost)) = queue.pop() {
            best_so_far[curr] = cost;
            if curr == b {
                break;
            }

            for adj_valve in self.valves_with_mask(self.tunnels[curr]) {
                // If moving to adj_valve would be better than the last time we saw this, do it
                if (cost + 1) <= best_so_far[adj_valve] {
                    best_so_far[adj_valve] = cost + 1;
                    // Re-explore from there, since it's better now (or tied)
                    queue.push((adj_valve, cost + 1));
                }
            }

            queue.sort();
        }
        debug_assert_ne!(best_so_far[b], u8::MAX);

        best_so_far[b]
    }

    fn pressure_delta(&self, open: u64) -> i64 {
        self.valves_with_mask(open).map(|v| self.rate_of(v)).sum()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    curr: usize,
    opened: u64,
    minute: u8,
    pressure: i64,
    history: Vec<(u8, u64)>,

    g: Graph,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let curr = { format!("{}", self.g.name_of(self.curr)) };

        let visited_ids = {
            format!(
                "0b{:016b} ({}) {:?}",
                self.opened,
                self.opened.count_ones(),
                self.g
                    .valves_with_mask(self.opened)
                    .map(|v| { self.g.name_of(v) })
                    .collect_vec()
            )
        };

        let history = {
            let mut hist = vec![];
            let mut prev: u64 = 0;
            for (minute, opened) in &self.history {
                let changed = opened ^ prev;
                let names = self
                    .g
                    .valves_with_mask(changed)
                    .map(|v| self.g.name_of(v))
                    .next()
                    .unwrap();
                hist.push(format!("[{minute:>2}, {names:?}]"));
                prev = *opened;
            }
            hist.join("; ")
        };

        f.debug_struct("State")
            .field("curr", &curr)
            .field("visited_ids", &visited_ids)
            .field("minute", &self.minute)
            .field("pressure", &self.pressure)
            .field("history", &history)
            .finish()
    }
}

impl State {
    fn new(curr: usize, g: &Graph) -> Self {
        State {
            curr,

            opened: 0,
            minute: 0,
            pressure: 0,
            history: vec![],

            g: g.clone(),
        }
    }

    fn has_visited(&self, other_id: usize) -> bool {
        self.opened & (1 << other_id as u64) != 0
    }

    fn visit_count(&self) -> usize {
        self.opened.count_ones() as usize
    }

    fn sort_key(&self) -> i64 {
        // bigger is handled sooner
        self.pressure
    }
}

// Part1 ========================================================================
#[aoc(day16, part1)]
pub fn part1(input: &str) -> i64 {
    init_logging();

    const TIME_LIMIT: u8 = 30;
    let g = Graph::new(input);
    let visit_limit = g.valves().filter(|&v| g.rate_of(v) != 0).count();

    let mut queue = BinaryHeap::new();
    queue.push(State::new(0, &g)); // AA is always 0

    let mut end = vec![];
    while let Some(state) = queue.pop() {
        // info!("[{}] Handling state = {state:?}", queue.len());
        if state.visit_count() == visit_limit || state.minute == TIME_LIMIT {
            end.push(state);
            continue;
        }

        let mut did_enqueue = false;
        for v in g.valves() {
            let time_left = TIME_LIMIT - state.minute;
            let visit_cost = g.visit_cost(state.curr, v);

            if
            // Only useful valves
            (g.rate_of(v) > 0)
                // That we haven't seen
                && !state.has_visited(v)
                // But could
                && (visit_cost < time_left)
            {
                // Note: We're "jumping" to the valve and eating the computed cost.
                // This means we never "visit" rate=0 nodes between here and there.

                // Visit it (takes N minute), and then open the valve (takes 1 minute)
                let minute = state.minute + visit_cost + 1;
                let mut pressure = state.pressure;
                pressure += (visit_cost as i64 + 1) * g.pressure_delta(state.opened);
                let opened = state.opened | (1 << v);

                let mut history = state.history.clone();
                history.push((minute, opened));

                // On then next step, this vavle will count
                let next = State {
                    curr: v,
                    minute,
                    pressure,
                    opened,
                    history,
                    g: g.clone(),
                };

                queue.push(next);
                did_enqueue = true;
            }
        }

        if !did_enqueue {
            end.push(state);
            continue;
        }
    }

    // Run the remaining states to completion
    for state in &mut end {
        debug_assert!(
            state.minute <= TIME_LIMIT,
            "state.minute = {} for some reason",
            state.minute
        );

        // I bet this triggers on bigger input, where you run out of time before exploring everything.
        debug_assert_eq!(
            state.opened.count_ones(),
            g.valves().filter(|&v| g.rate_of(v) != 0).count() as u32,
        );

        // This isn't free but also won't change anymore.
        let delta = g.pressure_delta(state.opened);
        let time_left = TIME_LIMIT - state.minute;
        state.pressure += delta * time_left as i64;
        state.minute += time_left;

        debug_assert_eq!(state.minute, TIME_LIMIT, "state is bad: {state:#?}");
    }

    let best = end
        .into_iter()
        .max_by_key(|state| state.pressure)
        .expect("No states finished?");

    if cfg!(debug_assertions) {
        info!("best = {best:#?}");
        g.save_dot("target/", Some(best.opened)).unwrap();
    }

    best.pressure
}

// Part2 ========================================================================
#[aoc(day16, part2)]
pub fn part2(input: &str) -> i64 {
    init_logging();

    const TIME_LIMIT: u8 = 26;
    let g = Graph::new(input);
    let visit_limit = g.valves().filter(|&v| g.rate_of(v) != 0).count() / 2;

    let mut queue = BinaryHeap::new();
    queue.push(State::new(0, &g)); // AA is always 0

    let mut end = vec![];
    while let Some(state) = queue.pop() {
        // info!("[{}] Handling state = {state:?}", queue.len());
        if state.visit_count() == visit_limit || state.minute == TIME_LIMIT {
            end.push(state);
            continue;
        }

        let mut did_enqueue = false;
        for v in g.valves() {
            let time_left = TIME_LIMIT - state.minute;
            let visit_cost = g.visit_cost(state.curr, v);

            if
            // Only useful valves
            (g.rate_of(v) > 0)
                // That we haven't seen
                && !state.has_visited(v)
                // But could
                && (visit_cost < time_left)
            {
                // Note: We're "jumping" to the valve and eating the computed cost.
                // This means we never "visit" rate=0 nodes between here and there.

                // Visit it (takes N minute), and then open the valve (takes 1 minute)
                let minute = state.minute + visit_cost + 1;
                let mut pressure = state.pressure;
                pressure += (visit_cost as i64 + 1) * g.pressure_delta(state.opened);
                let opened = state.opened | (1 << v);

                let mut history = state.history.clone();
                history.push((minute, opened));

                // On then next step, this vavle will count
                let next = State {
                    curr: v,
                    minute,
                    pressure,
                    opened,
                    history,
                    g: g.clone(),
                };

                queue.push(next);
                did_enqueue = true;
            }
        }

        if !did_enqueue {
            end.push(state);
            continue;
        }
    }

    // Run the remaining states to completion
    for state in &mut end {
        debug_assert!(
            state.minute <= TIME_LIMIT,
            "state.minute = {} for some reason",
            state.minute
        );

        // This isn't free but also won't change anymore.
        let delta = g.pressure_delta(state.opened);
        let time_left = TIME_LIMIT - state.minute;
        state.pressure += delta * time_left as i64;
        state.minute += time_left;

        debug_assert_eq!(state.minute, TIME_LIMIT, "state is bad: {state:#?}");
    }

    let mut maybe = vec![];
    let l = end.len();
    for i in 0..l {
        for j in (i + 1)..l {
            let a = &end[i];
            let b = &end[j];
            if (a.opened & b.opened) == 0 {
                maybe.push([a, b]);
            }
        }
    }

    let [a, b] = maybe
        .into_iter()
        .max_by_key(|[a, b]| a.pressure + b.pressure)
        .expect("No states finished?");
    info!("Found two best states:");
    info!("a = {a:#?}");
    info!("b = {b:#?}");

    a.pressure + b.pressure
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves AA, CC
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves AA, CC, EE
Valve EE has flow rate=3; tunnels lead to valves DD, FF
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn check_parse() {
        assert_eq!(
            Graph::new(EXAMPLE_INPUT.trim_start()).input(),
            EXAMPLE_INPUT
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

    #[rstest]
    #[case::given(1707, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(EZ_TIMEOUT)]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
