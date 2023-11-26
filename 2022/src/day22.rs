use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    #[default]
    Void = 0,
    Wall = 1,
    Ground = 2,
}
use Tile::*;

// Test and given problems are different enough
const CUBE_SIDE: i32 = if cfg!(test) { 4 } else { 50 };

/// # Starting position
/// From the problem description:
/// > You begin the path in the leftmost open tile of the top row of tiles.
/// Note that this is differnet between example and problem!
const START_X: i32 = if cfg!(test) { 2 * CUBE_SIDE } else { 50 } + 1;

// Part1 ========================================================================
fn do_steps_p1(grid: &mut Framebuffer<Tile>, here: &mut IVec2, dir: IVec2, steps: u32) {
    for _ in 0..steps {
        let mut next = *here + dir;
        match resolve_step_p1(grid, &mut next, dir) {
            Wall => {}
            Ground => *here = next,

            Void => unreachable!("resolve_step_p1() resolved to Void, which shouldn't happen"),
        }
        assert_eq!(grid[*here], Ground);
    }

    // Step once and figure out what kind of tile is there
    fn resolve_step_p1(grid: &Framebuffer<Tile>, next_point: &mut IVec2, dir: IVec2) -> Tile {
        match grid[*next_point] {
            Wall => Wall,
            Ground => Ground,
            Void => {
                // We're currently already on Void, so step backwards once
                *next_point -= dir;

                // And walk backwards until we find Void again
                while grid[*next_point] != Void {
                    // and then use 1 step forward from there.
                    *next_point -= CUBE_SIDE * dir;
                }
                *next_point += dir;

                assert_ne!(grid[*next_point], Void);
                grid[*next_point]
            }
        }
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let moves: &[u8] = input.lines().last().unwrap().as_bytes();
    let map_lines = input.lines().take_while(|l| !l.is_empty());

    // Rows & columns start from 1
    let max_x = 1 + map_lines.clone().map(|l| l.len()).max().unwrap_or_default() as i32;
    let max_y = 1 + map_lines.clone().count() as i32;

    let mut grid: Framebuffer<Tile> = Framebuffer::new_with_ranges(1..max_x, 1..max_y);
    grid.set_border_color(Some(Void));

    for (y, line) in map_lines.enumerate() {
        let y = y + 1;
        for (x, &b) in line.as_bytes().iter().enumerate() {
            let x = x + 1;

            if b == b'#' {
                grid[(x, y)] = Wall;
            } else if b == b'.' {
                grid[(x, y)] = Ground;
            }
        }
    }

    let mut here = IVec2::new(START_X, 1);
    let mut dir = IVec2::new(1, 0);

    for (is_digit, mut group) in &moves.iter().group_by(|b| b.is_ascii_digit()) {
        if is_digit {
            let steps = group
                .copied()
                .fold(0_u32, |acc, x| 10 * acc + (x - b'0') as u32);
            do_steps_p1(&mut grid, &mut here, dir, steps);
        } else {
            let rot = *group.next().unwrap() as char;

            if rot == 'R' {
                dir.y = -dir.y;
                std::mem::swap(&mut dir.x, &mut dir.y);
            } else {
                dir.x = -dir.x;
                std::mem::swap(&mut dir.x, &mut dir.y);
            }

            assert_ne!(dir, IVec2::zero());
            assert_eq!(group.next(), None);
        }
    }

    let row = here.y as i64;
    let col = here.x as i64;
    let facing = match dir.as_array() {
        [1, 0] => 0,
        [0, 1] => 1,
        [-1, 0] => 2,
        [0, -1] => 3,
        _ => unreachable!("??? {dir:?}"),
    };
    let password = 1_000 * row + 4 * col + facing;

    if cfg!(test) {
        println!("row={row}, col={col}, facing={facing}, password={password}");
    }

    password
}

// Part2 ========================================================================

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}
use Dir::*;

struct Map {
    grid: Framebuffer<Tile>,
    wrapping: Framebuffer<Option<(IVec2, IVec2)>>,
    history: Framebuffer<Option<IVec2>>,
}

impl Map {
    fn new(grid: Framebuffer<Tile>) -> Self {
        let mut wrapping = Framebuffer::new_matching_size(&grid);
        let history = Framebuffer::new_matching_size(&grid);

        generate_wrapping_map(&grid, &mut wrapping);

        // Control printing a debug map like this:
        //
        //  0 |         @@@@
        //  1 |        @...#@
        //  2 |        @.#..@
        //  3 |        @#...@
        //  4 | @@@@@@@@....@
        //  5 |@...#.......#@
        //  6 |@........#...@
        //  7 |@..#....#....@
        //  8 |@..........#.@@@@
        //  9 | @@@@@@@@...#....@
        // 10 |        @.....#..@
        // 11 |        @.#......@
        // 12 |        @......#.@
        // 13 |         @@@@@@@@
        //
        const DEBUG_MAP_WITH_WARPS: bool = cfg!(test) || cfg!(debug_assertions);
        if DEBUG_MAP_WITH_WARPS {
            for y in wrapping.range_y() {
                print!("{y:3} |");
                for x in wrapping.range_x().clone() {
                    let mut c;

                    c = match grid[(x, y)] {
                        Ground => '.',
                        Wall => '#',
                        Void => ' ',
                    };

                    if wrapping[(x, y)].is_some() {
                        c = '@';
                    };

                    print!("{c}");
                }
                println!();
            }
            println!();
        }

        Self {
            wrapping,
            grid,
            history,
        }
    }

    // Warps and rotates point
    fn wrap_point(&self, pt: &mut IVec2, dir: &mut IVec2) {
        let maybe = self.wrapping[*pt];

        let (wpt, wdir) = maybe.expect("Wrapping an invalid point?");

        assert_ne!(*pt, wpt, "These are not supposed to match.");
        assert_ne!(
            self.grid[wpt], Void,
            "Wrapping {pt:?}->{wpt:?}, but found Void instead of Ground or Wall"
        );

        *pt = wpt;
        *dir = wdir;
    }

    fn print(&self, here: IVec2) {
        println!("{}", self.print_to(Some(here)));
    }

    fn print_to(&self, here: Option<IVec2>) -> String {
        let mut s = String::new();

        for y in self.wrapping.range_y() {
            // s += &format!("{y:2} |");
            for x in self.wrapping.range_x().clone() {
                let xy = IVec2::new(x, y);
                let mut c;

                c = match self.grid[(x, y)] {
                    Ground => '.',
                    Wall => '#',
                    Void => ' ',
                };

                if let Some(dir) = self.history[(x, y)] {
                    let dir: (i32, i32) = dir.into();
                    let dir = match dir {
                        (0, -1) => North,
                        (0, 1) => South,
                        (1, 0) => East,
                        (-1, 0) => West,
                        _ => unreachable!("Unexpected dir: {dir:#?}"),
                    };
                    c = match dir {
                        North => '^',
                        South => 'v',
                        East => '>',
                        West => '<',
                    };
                }

                // "You Are Here" marker
                if here.map(|v| v == xy).unwrap_or(false) {
                    c = 'O';
                }

                s.push(c);
            }
            s.push('\n');
        }
        s.push('\n');

        s
    }
}

#[allow(clippy::identity_op, unused)]
fn generate_wrapping_map(
    grid: &Framebuffer<Tile>,
    wrapping: &mut Framebuffer<Option<(IVec2, IVec2)>>,
) {
    #[derive(Copy, Clone, Debug)]
    struct LinePair {
        a: ((i32, i32), (i32, i32)),
        a_dir: Dir,

        b: ((i32, i32), (i32, i32)),
        b_dir: Dir,
    }
    // Example and Given inputs are different and need to be handled differently. :(
    // Note: However, both have exactly 7 pairs.
    let line_pairs: [LinePair; 7] = if cfg!(test) {
        // Hard-code the lines that match. We'll walk along each and fill in `wrapping` using the 'other' line.
        [
            // NOTE: The ordering of the cordinates in lines matters. We're treating these lines like finite rays, that is DIRECTIONAL.
            // And we use the order they're listed to determine that direction!
            // This means, if they share a coordiante, they should all END with that coordiante.
            // ALSO: This is 1-indexed.

            //        !...#
            //        @.#..
            //        @#...
            //     !@@@....
            // ...#.......#
            // ........#...
            // ..#....#....
            // ..........#.
            //         ...#....
            //         .....#..
            //         .#......
            //         ......#.
            LinePair {
                a: ((9, 1), (9, 5)),
                a_dir: West, // 'problem side'

                b: ((5, 5), (9, 5)),
                b_dir: North,
            },
            //         ...#!
            //         .#..@
            //         #...@
            //         ....@
            // ...#.......#
            // ........#...
            // ..#....#....
            // ..........#.
            //         ...#....@
            //         .....#..@
            //         .#......@
            //         ......#.!
            LinePair {
                a: ((12, 1), (12, 5)),
                a_dir: East,

                b: ((16, 12), (16, 8)),
                b_dir: East,
            },
            //         ...#
            //         .#..
            //         #...
            //         ....
            // ...#.......#!
            // ........#...@
            // ..#....#....@
            // ..........#.@@@!
            //         ...#....
            //         .....#..
            //         .#......
            //         ......#.
            LinePair {
                a: ((12, 5), (12, 9)),
                a_dir: East,

                b: ((16, 9), (12, 9)),
                b_dir: North,
            },
            //         !@@@
            //         ...#
            //         .#..
            //         #...
            // @@@!    ....
            // ...#.......#
            // ........#...
            // ..#....#....
            // ..........#.
            //         ...#....
            //         .....#..
            //         .#......
            //         ......#.
            LinePair {
                a: ((9, 1), (13, 1)),
                a_dir: North,

                b: ((4, 5), (0, 5)),
                b_dir: North,
            },
            //          ...#
            //          .#..
            //          #...
            //          ....
            // !...#.......#
            // @........#...
            // @..#....#....
            // @..........#.
            //          ...#....
            //          .....#..
            //          .#......
            //          ......#.
            //              @@@!
            LinePair {
                a: ((1, 5), (1, 9)),
                a_dir: West,

                b: ((16, 12), (12, 12)),
                b_dir: South,
            },
            //         ...#
            //         .#..
            //         #...
            //         ....
            // ...#.......#
            // ........#...
            // ..#....#....
            // ..........#.
            // !@@@    ...#....
            //         .....#..
            //         .#......
            //         ......#.
            //         @@@!
            LinePair {
                a: ((1, 8), (5, 8)),
                a_dir: South,

                b: ((12, 12), (8, 12)),
                b_dir: South,
            },
            //         ...#
            //         .#..
            //         #...
            //         ....
            // ...#.......#
            // ........#...
            // ..#....#....
            // ..........#.
            //     @@@!...#....
            //        @.....#..
            //        @.#......
            //        @......#.
            LinePair {
                a: ((8, 8), (4, 8)),
                a_dir: South,

                b: ((8, 8), (8, 12)),
                b_dir: South,
            },
        ]
    } else {
        [
            //     ........
            //     ........
            //     ........
            //     ........
            //    !....
            //    @....
            //    @....
            // !@@@....
            // ........
            // ........
            // ........
            // ........
            // ....
            // ....
            // ....
            // ....
            LinePair {
                a: ((51, 51), (51, 101)),
                a_dir: West,

                b: ((1, 101), (51, 101)),
                b_dir: North,
            },
            //     ........
            //     ........
            //     ........
            //     ........
            //     ....
            //     ....
            //     ....
            //     ....
            // ........
            // ........
            // ........
            // ........
            // ....@@@!
            // ....@
            // ....@
            // ....!
            LinePair {
                a: ((100, 150), (50, 150)),
                a_dir: South,

                b: ((50, 200), (50, 150)),
                b_dir: East,
            },
            //     ........
            //     ........
            //     ........
            //     ........
            //     ....!@@@
            //     ....@
            //     ....@
            //     ....@
            // ........
            // ........
            // ........
            // ........
            // ....
            // ....
            // ....
            // ....
            LinePair {
                a: ((101, 50), (151, 50)),
                a_dir: South,

                b: ((100, 51), (100, 101)),
                b_dir: East,
            },
            //     ........@
            //     ........@
            //     ........@
            //     ........!
            //     ....
            //     ....
            //     ....
            //     ....
            // ........!
            // ........@
            // ........@
            // ........@
            // ....
            // ....
            // ....
            // ....
            LinePair {
                a: ((150, 50), (150, 0)),
                a_dir: East,

                b: ((100, 101), (100, 151)),
                b_dir: East,
            },
            //         @@@!
            //     ........
            //     ........
            //     ........
            //     ........
            //     ....
            //     ....
            //     ....
            //     ....
            // ........
            // ........
            // ........
            // ........
            // ....
            // ....
            // ....
            // ....
            // @@@!
            LinePair {
                a: ((150, 1), (100, 1)),
                a_dir: North,

                b: ((50, 200), (0, 200)),
                b_dir: South,
            },
            //      @@@!
            //      ........
            //      ........
            //      ........
            //      ........
            //      ....
            //      ....
            //      ....
            //      ....
            //  ........
            //  ........
            //  ........
            //  ........
            // @....
            // @....
            // @....
            // !....
            LinePair {
                a: ((1, 200), (1, 150)),
                a_dir: West,

                b: ((100, 1), (50, 1)),
                b_dir: North,
            },
            //     !........
            //     @........
            //     @........
            //     @........
            //      ....
            //      ....
            //      ....
            //      ....
            // @........
            // @........
            // @........
            // !........
            //  ....
            //  ....
            //  ....
            //  ....
            LinePair {
                a: ((51, 1), (51, 51)),
                a_dir: West,

                b: ((1, 150), (1, 100)),
                b_dir: West,
            },
        ]
    };

    for LinePair { a, a_dir, b, b_dir } in line_pairs {
        let a = (IVec2::new(a.0 .0, a.0 .1), IVec2::new(a.1 .0, a.1 .1));
        let b = (IVec2::new(b.0 .0, b.0 .1), IVec2::new(b.1 .0, b.1 .1));

        // Make sure the lines are both equal-length
        {
            let da = a.1 - a.0;
            let db = b.1 - b.0;

            // Make sure exactly one axis is 0
            assert!(
                (da.x == 0) ^ (da.y == 0),
                "da doesn't have exactly 1 zero-axis. da={da:?}"
            );
            assert!(
                (db.x == 0) ^ (db.y == 0),
                "db doesn't have exactly 1 zero-axis. db={db:?}"
            );

            // Should be exactly one cube-side in length
            assert_eq!(
                da.x.abs() + da.y.abs(),
                CUBE_SIDE,
                "da has a magnitude of {} instead of {CUBE_SIDE}",
                da.x.abs() + da.y.abs()
            );
            assert_eq!(
                db.x.abs() + db.y.abs(),
                CUBE_SIDE,
                "db has a magnitude of {} instead of {CUBE_SIDE}",
                db.x.abs() + db.y.abs()
            );
        }

        let da = (a.1 - a.0) / CUBE_SIDE;
        let db = (b.1 - b.0) / CUBE_SIDE;
        assert_ne!(da, IVec2::zero());
        assert_ne!(db, IVec2::zero());

        let oa: IVec2 = match a_dir {
            North => (0, -1).into(),
            South => (0, 1).into(),
            East => (1, 0).into(),
            West => (-1, 0).into(),
        };
        let ob: IVec2 = match b_dir {
            North => (0, -1).into(),
            South => (0, 1).into(),
            East => (1, 0).into(),
            West => (-1, 0).into(),
        };

        for i in 0..CUBE_SIDE {
            let aa = a.0 + i * da;
            let bb = b.0 + i * db;

            let da: IVec2 = match a_dir {
                North => (0, 1),
                South => (0, -1),
                East => (-1, 0),
                West => (1, 0),
            }
            .into();
            let db: IVec2 = match b_dir {
                North => (0, 1),
                South => (0, -1),
                East => (-1, 0),
                West => (1, 0),
            }
            .into();

            // The "index" of wrapping is the space that *triggers* the warp. We add an offset because this is Out Of Bounds
            // the value there needs to be in bounds, so we do NOT offset that!
            wrapping[aa + oa] = Some((bb, db));
            wrapping[bb + ob] = Some((aa, da));
        }
    }
}

fn do_steps_p2(map: &mut Map, here: &mut IVec2, dir: &mut IVec2, steps: u32) {
    for _ in 0..steps {
        let mut next = *here + *dir;
        // Save a dir here, so we can reference the old one below (mostly for history recording)
        let mut next_dir = *dir;
        match resolve_step_p2(map, &mut next, &mut next_dir) {
            Wall => { /* Cannot walk into wall, do nothing */ }

            Ground => {
                // Can walk onto ground, take the step
                map.history[*here] = Some(*dir);
                *here = next;
                *dir = next_dir;
            }

            Void => unreachable!("resolve_step_p2() resolved to Void, which shouldn't happen"),
        }

        // map.print(*here);
        assert_eq!(map.grid[*here], Ground);
    }

    // Step once and figure out what kind of tile is there
    fn resolve_step_p2(map: &mut Map, next_point: &mut IVec2, dir: &mut IVec2) -> Tile {
        match map.grid[*next_point] {
            Wall => Wall,
            Ground => Ground,
            Void => {
                // Save the wrapped point
                map.wrap_point(next_point, dir);

                // Adjust our direction to account for the warp

                map.grid[*next_point]
            }
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let moves: &[u8] = input.lines().last().unwrap().as_bytes();
    let map_lines = input.lines().take_while(|l| !l.is_empty());

    // +1: Rows & columns start from 1
    // +1: Add padding around our loaded cells, for our wrapping spaces
    let max_x = 1 + 1 + map_lines.clone().map(|l| l.len()).max().unwrap_or_default() as i32;
    let max_y = 1 + 1 + map_lines.clone().count() as i32;

    let mut grid: Framebuffer<Tile> = Framebuffer::new_with_ranges(0..max_x, 0..max_y);
    grid.set_border_color(Some(Void));

    for (y, line) in map_lines.enumerate() {
        let y = y + 1;
        for (x, &b) in line.as_bytes().iter().enumerate() {
            let x = x + 1;

            if b == b'#' {
                grid[(x, y)] = Wall;
            } else if b == b'.' {
                grid[(x, y)] = Ground;
            }
        }
    }

    let mut here = IVec2::new(START_X, 1);
    let mut dir = IVec2::new(1, 0);

    let mut map = Map::new(grid);

    for (is_digit, mut group) in &moves.iter().group_by(|b| b.is_ascii_digit()) {
        if is_digit {
            let steps = group
                .copied()
                .fold(0_u32, |acc, x| 10 * acc + (x - b'0') as u32);

            do_steps_p2(&mut map, &mut here, &mut dir, steps);
        } else {
            let rot = *group.next().unwrap() as char;

            if rot == 'R' {
                dir.y = -dir.y;
                std::mem::swap(&mut dir.x, &mut dir.y);
            } else {
                dir.x = -dir.x;
                std::mem::swap(&mut dir.x, &mut dir.y);
            }

            assert_ne!(dir, IVec2::zero());
            assert_eq!(group.next(), None);
        }
    }

    let row = here.y as i64;
    let col = here.x as i64;
    let facing = match dir.as_array() {
        [1, 0] => 0,
        [0, 1] => 1,
        [-1, 0] => 2,
        [0, -1] => 3,
        _ => unreachable!("??? {dir:?}"),
    };
    let password = 1_000 * row + 4 * col + facing;

    if cfg!(debug_assertions) || cfg!(test) {
        map.print(here);
        println!("row={row}, col={col}, facing={facing}, password={password}");
    }

    password
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    #[rstest]
    #[case::given(6032, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(5031, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }
}
