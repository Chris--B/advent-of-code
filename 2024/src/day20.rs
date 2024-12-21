use crate::prelude::*;

struct Day20Graph {
    quota: i32,
    map: Framebuffer<char>,
    dist: HashMap<IVec3, i64>,
    prev: HashMap<IVec3, IVec3>,
}

impl Graph for Day20Graph {
    type Vert = IVec3;

    fn verts(&self) -> impl Iterator<Item = IVec3> {
        #![allow(unreachable_code)]
        todo!();
        [].into_iter()
    }

    fn neighbors(&self, vert: &IVec3) -> impl Iterator<Item = IVec3> + 'static {
        let mut verts: SmallVec<[IVec3; 8]> = smallvec![];

        for v in vert.xy().neighbors() {
            for dz in [-1, 0] {
                let v = IVec3::new(v.x, v.y, vert.z + dz);
                if self.edge_weight(vert, &v).is_some() {
                    verts.push(v);
                }
            }
        }

        verts.into_iter()
    }

    fn edge_weight(&self, from: &IVec3, to: &IVec3) -> Option<i64> {
        if !self.map.in_bounds(from.xy()) || !self.map.in_bounds(to.xy()) {
            return None;
        }

        let [x1, y1, z1] = from.as_array();
        let [x2, y2, z2] = to.as_array();

        assert!(z1 <= self.quota);
        assert!(z2 <= self.quota);
        if z1 < 0 || z2 < 0 {
            return None;
        }

        let diff = (*from - *to).xy().abs();
        if diff.x + diff.y == 1 {
            if [z1, z2] == [self.quota, self.quota] || [z1, z2] == [0, 0] {
                // Not in a cheat, check the map
                if self.map[(x1, y1)] == '.' && self.map[(x2, y2)] == '.' {
                    return Some(1);
                }
            } else if z1 == z2 + 1 {
                // In a cheat, ignore walls
                return Some(1);
            }
        } else {
            // Never expect to ask for an edge weight for non-adjacent states
            unreachable!()
        }

        None
    }

    fn distance_get(&self, vert: Self::Vert) -> Option<i64> {
        self.dist.get(&vert).copied()
    }

    fn distance_set(&mut self, vert: Self::Vert, dist: i64) {
        self.dist.insert(vert, dist);
    }

    fn prev_set(&mut self, vert: Self::Vert, prev: Self::Vert) {
        self.prev.insert(vert, prev);
    }
}

// Part1 ========================================================================
#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    #![allow(unused)]

    let dims = if cfg!(test) {
        let m = Framebuffer::parse_grid_u8(input);
        IVec2::new(m.width() as i32, m.height() as i32)
    } else {
        IVec2::new(141, 141)
    };

    let mut start = IVec2::zero();
    let mut end = IVec2::zero();

    let mut map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| match info.c {
        '#' => '#',
        '.' => '.',
        'S' => {
            start = IVec2::new(info.x, info.y);
            '.'
        }
        'E' => {
            end = IVec2::new(info.x, info.y);
            '.'
        }
        c => unreachable!("Unexpected map character: {c:?}"),
    });

    if cfg!(test) {
        println!("start={start:?}");
        println!("end  ={end:?}");
        map[start] = 'S';
        map[end] = 'E';
        map.just_print();
        map[start] = '.';
        map[end] = '.';
    }

    let mut graph = AocGridGraph::new(map);
    dijkstra(&mut graph, start, Some(end));
    let shortest_dist_no_cheating = graph.distance_get(end).unwrap();

    let quota = 2;
    // let quota = 20;
    let mut cheat_graph = Day20Graph {
        quota,
        map: graph.map.clone(),
        dist: HashMap::new(),
        prev: HashMap::new(),
    };
    dijkstra(&mut cheat_graph, IVec3::new(start.x, start.y, quota), None);

    if cfg!(test) {
        println!("Best paths WITH cheating:");
        let end_states = (0..=quota)
            .map(|z| IVec3::new(end.x, end.y, z))
            .collect_vec();

        for e in end_states {
            let dist = cheat_graph.distance_get(e).unwrap();
            let savings = shortest_dist_no_cheating - dist;
            let [x, y, z] = e.as_array();
            println!("  + end=({x}, {y}) ({z:>2} cheat left): saves {savings:>4} picoseconds ({shortest_dist_no_cheating} -> {dist})");
        }
    }

    let min_savings = if cfg!(test) { 20 } else { 100 };
    let cost_to_ignore = shortest_dist_no_cheating - min_savings;

    // TODO:
    //      1. BFS from the end, stopping when (cheat_graph.distance_get(curr) > cost_to_ignore) or curr == start
    //      2. De-duplicate based on nodes in path with z==19 and (firt) z == 0
    //      3. Tally savings
    //      4. Profit!

    0
}

// Part2 ========================================================================
#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    #![allow(unused)]

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(750))]
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
        On Example input:
            There are 32 cheats that save 50 picoseconds.
            There are 31 cheats that save 52 picoseconds.
            There are 29 cheats that save 54 picoseconds.
            There are 39 cheats that save 56 picoseconds.
            There are 25 cheats that save 58 picoseconds.
            There are 23 cheats that save 60 picoseconds.
            There are 20 cheats that save 62 picoseconds.
            There are 19 cheats that save 64 picoseconds.
            There are 12 cheats that save 66 picoseconds.
            There are 14 cheats that save 68 picoseconds.
            There are 12 cheats that save 70 picoseconds.
            There are 22 cheats that save 72 picoseconds.
            There are  4 cheats that save 74 picoseconds.
            There are  3 cheats that save 76 picoseconds.
    */
    #[rstest]
    #[case::given(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
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
