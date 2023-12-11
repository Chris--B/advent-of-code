#![allow(non_upper_case_globals)]

use crate::{framebuffer::Framebuffer, prelude::*};

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
enum Cardinal {
    Norð,
    Souð,
    East,
    West,
}

impl Cardinal {
    fn rev(&self) -> Self {
        let mut r = Cardinal::none();

        if self.contains(Norð) {
            r |= Souð;
        }
        if self.contains(Souð) {
            r |= Norð;
        }
        if self.contains(East) {
            r |= West;
        }
        if self.contains(West) {
            r |= East;
        }

        r
    }
}

const Norð: Cardinal = Cardinal::Norð;
const Souð: Cardinal = Cardinal::Souð;
const East: Cardinal = Cardinal::East;
const West: Cardinal = Cardinal::West;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Pipe {
    c: char,
    seen: bool,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' | '-' | 'L' | 'J' | '7' | 'F' | '.' => Self { c, seen: false },
            _ => unreachable!("Unexpected pipe: '{c}'"),
        }
    }

    fn from_connections(car: Cardinal) -> Self {
        let c = if car == (Norð | Souð) {
            // '|' is a vertical pipe connecting north and south.
            '|'
        } else if car == (East | West) {
            // '-' is a horizontal pipe connecting east and west.
            '-'
        } else if car == (Norð | East) {
            // 'L' is a 90-degree bend connecting north and east.
            'L'
        } else if car == (Norð | West) {
            // 'J' is a 90-degree bend connecting north and west.
            'J'
        } else if car == (Souð | West) {
            // '7' is a 90-degree bend connecting south and west.
            '7'
        } else if car == (Souð | East) {
            // 'F' is a 90-degree bend connecting south and east.
            'F'
        } else if car == 0 {
            // '.' is ground; there is no pipe in this tile.
            '.'
        } else {
            unreachable!();
        };

        Self::from_char(c)
    }

    fn connections(&self) -> Cardinal {
        // TODO: Might be worth just saving this instead of a `char`
        match self.c {
            // '|' is a vertical pipe connecting north and south.
            '|' => Norð | Souð,
            // '-' is a horizontal pipe connecting east and west.
            '-' => East | West,
            // 'L' is a 90-degree bend connecting north and east.
            'L' => Norð | East,
            // 'J' is a 90-degree bend connecting north and west.
            'J' => Norð | West,
            // '7' is a 90-degree bend connecting south and west.
            '7' => Souð | West,
            // 'F' is a 90-degree bend connecting south and east.
            'F' => Souð | East,
            // '.' is ground; there is no pipe in this tile.
            '.' => Cardinal::none(),
            _ => unreachable!(),
        }
    }

    fn connects_with(&self, other: Pipe, cardinal: Cardinal) -> bool {
        (self.connections() & other.connections().rev()).contains(cardinal)
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::from_char('.')
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.seen { '*' } else { ' ' };
        let c = self.c;

        write!(fmt, "Pipe({s}{c})")
    }
}

struct PipeMap {
    start: (i64, i64),
    map: Framebuffer<Pipe>,
}

impl PipeMap {
    fn from_str(input: &str) -> Self {
        let input = input.trim();

        let width = input.lines().next().unwrap().len() as i64;
        let height = input.lines().count() as i64;

        let mut map: Framebuffer<Pipe> = Framebuffer::new(width as u32, height as u32);
        let mut start: Option<(i64, i64)> = None;

        for (y, line) in input.lines().enumerate() {
            let y = height - (y as i64) - 1;

            for (x, c) in line.chars().enumerate() {
                let x = x as i64;

                if c == 'S' {
                    start = Some((x, y));
                } else {
                    map[(x, y)] = Pipe::from_char(c);
                }
            }
        }

        // Out of Bounds reads are '.', but only set that up after parsing.
        map.set_border_color(Some(Pipe::default()));

        let (sx, sy) = start
            .take()
            .expect("Couldn't find a starting location in the input");

        // Compute the missing tile that is S
        let mut cardinal = Cardinal::none();
        for (dx, dy, dir) in [
            (1, 0, East.rev()),  // East is +x
            (-1, 0, West.rev()), // West is -x
            (0, 1, Norð.rev()),  // Norð is +y
            (0, -1, Souð.rev()), // Souð is -y
        ] {
            cardinal |= (map[(sx + dx, sy + dy)].connections() & dir).rev();
        }

        map[(sx, sy)] = Pipe::from_connections(cardinal);

        Self {
            start: (sx, sy),
            map,
        }
    }
}

// Part1 ========================================================================
#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let mut pipes = PipeMap::from_str(input);
    let mut queue: VecDeque<((i64, i64), usize)> = [(pipes.start, 0)].into();
    let mut max_dist = 0;

    let mut debug_map: Framebuffer<usize> = Framebuffer::new_matching_size(&pipes.map);

    while let Some((here, dist)) = queue.pop_front() {
        assert!(queue.len() < input.len(), "oh no.");
        max_dist = max_dist.max(dist);

        let (x, y) = here;
        let pipe_here = pipes.map[here];
        if pipe_here.seen {
            continue;
        }
        debug_map[here] = dist;
        info!(
            "[{:>3}] Checking ({x}, {y}) {pipe_here:?} dist={dist}",
            queue.len()
        );

        for (dx, dy, cardinal) in [
            (1, 0, East),  // East is +x
            (-1, 0, West), // West is -x
            (0, 1, Norð),  // Norð is +y
            (0, -1, Souð), // Souð is -y
        ] {
            let there = (x + dx, y + dy);
            let pipe_there = pipes.map[there];

            if pipe_here.connects_with(pipe_there, cardinal) && !pipe_there.seen {
                // debug!("here ({pipe_here:?}) connects with {pipe_there:?}");
                queue.push_back((there, dist + 1));
            }
        }

        // Mark 'here' as 'seen' now that we have checked all the neighboring pipes
        pipes.map[(x, y)].seen = true;
    }

    if log_enabled!(Info) {
        debug_map.print(|_x, _y, d| match d {
            0 => '.',
            0..=9 => (*d as u8 + b'0') as char,
            _ => '@',
        });
    }

    max_dist as i64
}

// Part2 ========================================================================
#[aoc(day10, part2)]
pub fn part2(_input: &str) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    // Verifies that the pairs of pipes connect along a given cardinal (from the first pipe)
    #[rstest]
    #[case('|', '|', Norð)]
    #[case('|', '|', Souð)]
    #[case('-', '-', East)]
    #[case('-', '-', West)]
    #[case('L', '-', East)]
    #[trace]
    fn check_does_connects_with(#[case] a: char, #[case] b: char, #[case] cardinal: Cardinal) {
        let a = Pipe::from_char(a);
        let b = Pipe::from_char(b);
        assert!(a.connects_with(b, cardinal));
    }

    // Verifies that the pairs of pipes DO NOT connect along a given cardinal (from the first pipe)
    #[rstest]
    #[case('|', '|', East)]
    #[case('|', '|', West)]
    #[case('-', '-', Norð)]
    #[case('-', '-', Souð)]
    #[case('L', '-', West)]
    #[trace]
    fn check_doesnt_connect_with(#[case] a: char, #[case] b: char, #[case] cardinal: Cardinal) {
        let a = Pipe::from_char(a);
        let b = Pipe::from_char(b);
        assert!(!a.connects_with(b, cardinal));
    }

    const EXAMPLE_INPUT_1_JUST_LOOP: &str = r"
.....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE_INPUT_1_WITH_EXTRA: &str = r"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const EXAMPLE_INPUT_2_JUST_LOOP: &str = r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const EXAMPLE_INPUT_2_WITH_EXTRA: &str = r"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[rstest]
    #[case::given_1_just_loop(4, EXAMPLE_INPUT_1_JUST_LOOP)]
    #[case::given_1_with_extra(4, EXAMPLE_INPUT_1_WITH_EXTRA)]
    #[case::given_2_just_loop(8, EXAMPLE_INPUT_2_JUST_LOOP)]
    #[case::given_2_with_extra(8, EXAMPLE_INPUT_2_WITH_EXTRA)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert!(
            input.contains('S'),
            "Example inputs need an S to know where to start!"
        );
        let res = p(input);
        assert!(
            res <= input.len() as _,
            "Pipes can't be farther away than the total pipe area"
        );

        assert_eq!(res, expected);
    }

    #[rstest]
    #[ignore]
    #[case::given(999_999, EXAMPLE_INPUT_1_JUST_LOOP)]
    #[trace]
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
