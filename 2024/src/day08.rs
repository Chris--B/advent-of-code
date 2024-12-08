use crate::prelude::*;

use std::ops::{Index, IndexMut};

const MAP_DIM: usize = 1 + (b'z' - b'0') as usize;

#[derive(Clone)]
struct AntennaMap {
    map: [Vec<IVec2>; MAP_DIM],
    dims: IVec2,
}

impl AntennaMap {
    fn new() -> Self {
        Self {
            map: [const { vec![] }; MAP_DIM],
            dims: IVec2::zero(),
        }
    }

    fn parse(lines: &str) -> Self {
        let mut map = Self::new();

        for (y, line) in lines.lines().enumerate() {
            map.dims.y = map.dims.y.max(y as i32);
            for (x, c) in line.chars().enumerate() {
                map.dims.x = map.dims.x.max(x as i32);
                if c == '.' || c == '#' {
                    continue;
                }
                map.add_antenna(c, IVec2::new(x as i32, y as i32));
            }
        }

        map.dims += IVec2::new(1, 1);

        if cfg!(debug_assertions) {
            println!("Antenna within {:?} Summary", map.dims.as_array());
            for (c, a) in map.antennas() {
                println!("  + {l} '{c}' antennas", l = a.len());
            }
        }

        map
    }

    fn contains(&self, point: IVec2) -> bool {
        (0 <= point.x) && (point.x < self.dims.x) && // x-axis
        (0 <= point.y) && (point.y < self.dims.y) // y-axis
    }

    /// Returns true if this is the first antenna with this frequency
    fn add_antenna(&mut self, freq: char, pos: IVec2) -> bool {
        self.dims.x = self.dims.x.max(pos.x);
        self.dims.y = self.dims.y.max(pos.y);

        self[freq].push(pos);
        self[freq].len() == 1
    }

    fn antennas(&self) -> impl Iterator<Item = (char, &[IVec2])> {
        self.map
            .iter()
            .enumerate()
            .filter(|(_i, antennas)| !antennas.is_empty())
            .map(|(i, a)| {
                let c = (i as u8 + b'0') as char;
                assert!(c.is_alphanumeric());
                (c, a.as_slice())
            })
    }
}

impl Index<char> for AntennaMap {
    type Output = Vec<IVec2>;

    fn index(&self, idx: char) -> &Self::Output {
        assert!(
            idx.is_ascii_alphanumeric(),
            "{idx:?}' is not ASCII Alphanum?"
        );
        let idx = idx as u8;
        &self.map[(idx - b'0') as usize]
    }
}

impl IndexMut<char> for AntennaMap {
    fn index_mut(&mut self, idx: char) -> &mut Self::Output {
        assert!(
            idx.is_ascii_alphanumeric(),
            "{idx:?}' is not ASCII Alphanum?"
        );
        let idx = idx as u8;
        &mut self.map[(idx - b'0') as usize]
    }
}

// Part1 ========================================================================
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let map = AntennaMap::parse(input);
    let mut seen = vec![0; (map.dims.x * map.dims.y) as usize];

    for (_c, antennas) in map.antennas() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                let d = a - b;

                for next in [a + d, b - d] {
                    if map.contains(next) {
                        seen[(next.x + map.dims.x * next.y) as usize] = 1;
                    }
                }
            }
        }
    }

    seen.iter().sum()
}

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let map = AntennaMap::parse(input);
    let mut seen = vec![0; (map.dims.x * map.dims.y) as usize];

    for (_c, antennas) in map.antennas() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                let d = a - b;

                let mut next = a;
                while map.contains(next) {
                    seen[(next.x + map.dims.x * next.y) as usize] = 1;
                    next += d;
                }

                let mut next = b;
                while map.contains(next) {
                    seen[(next.x + map.dims.x * next.y) as usize] = 1;
                    next += -d;
                }
            }
        }
    }

    seen.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
";

    const SMOL_2A: &str = r"
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
";

    const SMOL_3A: &str = r"
..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
";

    const DONT_COUNT_DUPES: &str = r"
A..
.A.
BB.
";

    const CORNER_TOP_RIGHT: &str = r"
......#
...0...
0......
";

    const CORNER_BOT_RIGHT: &str = r"
0......
...0...
......#
";

    const CORNER_TOP_LEFT: &str = r"
#......
...0...
......0
";

    const CORNER_BOT_LEFT: &str = r"
......0
...0...
#......
";

    const DONT_COUNT_NEGATIVE: &str = r"
0...
...0
";

    #[rstest]
    #[case::given_ex(14, EXAMPLE_INPUT)]
    #[case::corners_top_right(1, CORNER_TOP_RIGHT)]
    #[case::corners_bot_right(1, CORNER_BOT_RIGHT)]
    #[case::corners_top_left(1, CORNER_TOP_LEFT)]
    #[case::corners_bot_left(1, CORNER_BOT_LEFT)]
    #[case::given_smol(2, SMOL_2A)]
    #[case::given_smol(4, SMOL_3A)]
    #[case::tiny_with_dupes(1, DONT_COUNT_DUPES)]
    #[case::dont_count_negative(0, DONT_COUNT_NEGATIVE)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[notrace]
        #[case]
        input: &str,
    ) {
        init_logging();

        let input = input.trim();
        println!("input=\n{input}");
        assert_eq!(p(input), expected);
    }

    const DIAG_EXACT: &str = r"
0...............
...0............
......#.........
.........#......
............#...
...............#
";

    const DIAG_EXACT_ONE_LESS: &str = r"
0..............
...0...........
......#........
.........#.....
............#..
...............
";

    const DIAG_EXACT_ONE_MORE: &str = r"
0................
...0.............
......#..........
.........#.......
............#....
...............#.
";

    #[rstest]
    #[case::given(34, EXAMPLE_INPUT)]
    #[case::diag(6, DIAG_EXACT)]
    #[case::diag(5, DIAG_EXACT_ONE_LESS)]
    #[case::diag(6, DIAG_EXACT_ONE_MORE)]
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
