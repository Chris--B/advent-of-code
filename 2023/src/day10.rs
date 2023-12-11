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
        let seen = false;
        match c {
            '|' | '-' | 'L' | 'J' | '7' | 'F' | '.' => Self { c, seen },
            ' ' => Self { c: '.', seen },
            _ => unreachable!("Unexpected pipe: '{c}'"),
        }
    }

    fn from_cardinals(car: Cardinal) -> Self {
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
        } else if car == East || car == West {
            // Duplicate it into East-West
            '-'
        } else if car == Norð || car == Souð {
            // Duplicate it into North-South
            '|'
        } else if car == 0 {
            // '.' is ground; there is no pipe in this tile.
            '.'
        } else {
            unreachable!("Unhandled: {car:?}");
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

    fn cardinals_with(&self, other: Pipe) -> Cardinal {
        self.connections() & other.connections().rev()
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

#[derive(Clone, PartialEq, Eq)]
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

        map[(sx, sy)] = Pipe::from_cardinals(cardinal);

        Self {
            start: (sx, sy),
            map,
        }
    }

    fn count_tiles(&self) -> usize {
        self.map.range_y().count() * self.map.range_x().count()
    }

    fn pretty_print_pipes(&self) {
        let mut debug_map: Framebuffer<char> = Framebuffer::new_matching_size(&self.map);

        for y in debug_map.range_y() {
            for x in debug_map.range_x() {
                let c = match self.map[(x, y)].c {
                    // '|' is a vertical pipe connecting north and south.
                    '|' => '┃',
                    // '-' is a horizontal pipe connecting east and west.
                    '-' => '━',
                    // 'L' is a 90-degree bend connecting north and east.
                    'L' => '┗',
                    // 'J' is a 90-degree bend connecting north and west.
                    'J' => '┛',
                    // '7' is a 90-degree bend connecting south and west.
                    '7' => '┓',
                    // 'F' is a 90-degree bend connecting south and east.
                    'F' => '┏',
                    // '.' is ground; there is no pipe in this tile.
                    '.' => '.',
                    _ => unreachable!(),
                };
                debug_map[(x, y)] = c;
            }
        }

        debug_map.print(|_x, _y, c| *c);
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
                queue.push_back((there, dist + 1));
            }
        }

        // Mark 'here' as 'seen' now that we have checked all the neighboring pipes
        pipes.map[(x, y)].seen = true;
    }

    if log_enabled!(Info) {
        pipes.pretty_print_pipes()
    }

    max_dist as i64
}

fn stretch_map(map: Framebuffer<Pipe>) -> Framebuffer<Pipe> {
    let mut big_map = Framebuffer::new(2 * map.width() as u32, 2 * map.height() as u32);

    for y in map.range_y() {
        for x in map.range_x() {
            // NW corner copies verbatim, the rest just need to connect
            let here = map[(x, y)];
            let f = |x: i32, y: i32, car: Cardinal| {
                Pipe::from_cardinals(here.cardinals_with(map[(x, y)]) & car)
            };

            big_map[(2 * x + 0, 2 * y + 0)] = f(x + 0, y + 0, Norð);
            big_map[(2 * x + 1, 2 * y + 0)] = f(x + 1, y + 0, East);
            big_map[(2 * x + 0, 2 * y + 1)] = f(x + 0, y + 1, West);
            big_map[(2 * x + 1, 2 * y + 1)] = f(x + 1, y + 1, Souð);
        }
    }

    big_map
}

// Part2 ========================================================================
#[aoc(day10, part2)]
#[allow(unused)]
pub fn part2(input: &str) -> i64 {
    let mut pipes = PipeMap::from_str(input);
    let mut seen: HashSet<(i64, i64)> = HashSet::new();

    pipes.map = stretch_map(pipes.map);

    // There are four types of tiles in our map how:
    //      #1. Ground, reachable from the edges.                               Not a candidate for the nest.
    //      #2. Pipes, *only in the loop*.                                      Not a candidate for the nest.
    //      #3. Ground, surrounded by pipes, but reachable via "squeezing".     Not a candidate for the nest.
    //      #4. Ground, surrounded by pipes, NOT reachable via "squeeing".      Candidate for the nest!
    //  We need to count the number of type 4 tiles. We'll do this by counting types #1, #2 and #3, then complementing.
    let mut candidate_tiles = pipes.count_tiles();

    // Big Ol' Loop to count #2, "pipes"
    // We do this first so further things can ignore the whole "ground tile" thing and just use `seen`
    {
        let mut queue: VecDeque<((i64, i64))> = [(pipes.start)].into();

        while let Some(here) = queue.pop_front() {
            assert!(queue.len() < input.len(), "oh no.");

            let (x, y) = here;
            let pipe_here = pipes.map[here];
            if pipe_here.seen {
                continue;
            }

            for (dx, dy, cardinal) in [
                (1, 0, East),  // East is +x
                (-1, 0, West), // West is -x
                (0, 1, Norð),  // Norð is +y
                (0, -1, Souð), // Souð is -y
            ] {
                let there = (x + dx, y + dy);
                let pipe_there = pipes.map[there];

                if pipe_here.connects_with(pipe_there, cardinal) && !pipe_there.seen {
                    queue.push_back(there);
                }
            }

            // Mark 'here' as 'seen' now that we have checked all the neighboring pipes
            pipes.map[(x, y)].seen = true;
            seen.insert((x, y));
        }
    }

    // Flood fill to count #1, "ground reachable from the edges"
    {
        let start_x = (pipes.map.range_x().start - 1) as _;
        let end_x = (pipes.map.range_x().end + 1) as _;

        let start_y = (pipes.map.range_y().start - 1) as _;
        let end_y = (pipes.map.range_y().end + 1) as _;

        let xs = start_x..end_x;
        let ys = start_y..end_y;

        let mut queue: VecDeque<((i64, i64))> = [(start_x, start_y)].into();
        while let Some((x, y)) = queue.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));

            assert!(queue.len() < pipes.count_tiles());

            // info!(
            //     "[{:>3}, {candidate_tiles:>3}] Checking ({x}, {y})",
            //     queue.len()
            // );

            for (dx, dy, _cardinal) in [
                (1, 0, East),  // East is +x
                (-1, 0, West), // West is -x
                (0, 1, Norð),  // Norð is +y
                (0, -1, Souð), // Souð is -y
            ] {
                let x = (x + dx);
                let y = (y + dy);

                // Don't reexplore
                if seen.contains(&(x, y)) {
                    continue;
                }

                // Don't explore out of bounds
                if !xs.contains(&x) || !ys.contains(&y) {
                    continue;
                }

                queue.push_back((x, y));
            }

            // Use the real bounds for this
            if pipes.map.range_x().contains(&(x as _)) && pipes.map.range_y().contains(&(y as _)) {
                candidate_tiles -= 1;
            }
        }
    }

    // Ground, surrounded by pipes, but squeezable
    // We'll do this by scaling the entire map and extending each tile with its connectors
    // e.g. 'J' -> 'J.' and '|' -> '|.'
    //             '..'            '|.'

    if log_enabled!(Info) {
        {
            let mut pipes = PipeMap::from_str(input);
            pipes.map.print(|_x, _y, pipe| {
                match pipe.c {
                    // '|' is a vertical pipe connecting north and south.
                    '|' => "┃",
                    // '-' is a horizontal pipe connecting east and west.
                    '-' => "━",
                    // 'L' is a 90-degree bend connecting north and east.
                    'L' => "┗",
                    // 'J' is a 90-degree bend connecting north and west.
                    'J' => "┛",
                    // '7' is a 90-degree bend connecting south and west.
                    '7' => "┓",
                    // 'F' is a 90-degree bend connecting south and east.
                    'F' => "┏",
                    // '.' is ground; there is no pipe in this tile.
                    '.' => ".",
                    _ => unreachable!(),
                }
            });
        }
        // pipes.pretty_print_pipes();

        let mut debug_map: Framebuffer<char> = Framebuffer::new_matching_size(&pipes.map);
        debug_map.print(|x, y, _c| {
            match pipes.map[(x, y)].c {
                // '|' is a vertical pipe connecting north and south.
                '|' => "┃",
                // '-' is a horizontal pipe connecting east and west.
                '-' => "━",
                // 'L' is a 90-degree bend connecting north and east.
                'L' => "┗",
                // 'J' is a 90-degree bend connecting north and west.
                'J' => "┛",
                // '7' is a 90-degree bend connecting south and west.
                '7' => "┓",
                // 'F' is a 90-degree bend connecting south and east.
                'F' => "┏",
                // '.' is ground; there is no pipe in this tile.
                '.' => {
                    if seen.contains(&(x as _, y as _)) {
                        "."
                    } else {
                        /*
                           Black        0;30     Dark Gray     1;30
                           Red          0;31     Light Red     1;31
                           Green        0;32     Light Green   1;32
                           Brown/Orange 0;33     Yellow        1;33
                           Blue         0;34     Light Blue    1;34
                           Purple       0;35     Light Purple  1;35
                           Cyan         0;36     Light Cyan    1;36
                           Light Gray   0;37     White         1;37
                        */
                        "\x1b[0;35m#\x1b[0m"
                    }
                }
                _ => unreachable!(),
            }
        });

        //     pipes.pretty_print_pipes()
    }

    // Sanity bounds on my input
    if input.len() > 100 * 100 {
        assert!(candidate_tiles > 512);
        assert!(candidate_tiles < 14640);
    }

    candidate_tiles as i64
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

    const EXAMPLE_INPUT_PART2_1_NO_SQUEEZING: &str = r"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE_INPUT_PART2_1_SQUEEZING: &str = r"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    const EXAMPLE_INPUT_PART2_2: &str = r"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE_INPUT_PART2_3: &str = r"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[rstest]
    #[case::given(4, EXAMPLE_INPUT_PART2_1_NO_SQUEEZING)]
    #[case::given(4, EXAMPLE_INPUT_PART2_1_SQUEEZING)]
    #[case::given(8, EXAMPLE_INPUT_PART2_2)]
    #[case::given(8, EXAMPLE_INPUT_PART2_3)]
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

    const EXAMPLE_INPUT_PART2_1_SQUEEZING_BIG: &str = r"
...................
...................
..S-------------7..
..|.............|..
..|.F---------7.|..
..|.|.........|.|..
..|.|.........|.|..
..|.|.........|.|..
..|.|.........|.|..
..|.|.........|.|..
..|.L---7.F---J.|..
..|.....|.|.....|..
..|.....|.|.....|..
..|.....|.|.....|..
..L-----J.L-----J..
...................
...................
...................
";

    #[rstest]
    #[case(EXAMPLE_INPUT_PART2_1_SQUEEZING, EXAMPLE_INPUT_PART2_1_SQUEEZING_BIG)]
    fn check_part2_stretch_map(#[case] map: &str, #[case] big_map: &str) {
        let mut pipes = PipeMap::from_str(map);
        let big_pipes = PipeMap::from_str(big_map);

        pipes.map = stretch_map(pipes.map);

        pipes.pretty_print_pipes();
        big_pipes.pretty_print_pipes();

        if pipes != big_pipes {
            panic!("maps don't match");
        }
    }
}
