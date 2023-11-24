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
const START_X: i32 = if cfg!(test) { 2 * CUBE_SIDE } else { 50 };

// Part1 ========================================================================
fn do_steps_p1(grid: &mut Framebuffer<Tile>, here: &mut IVec2, dir: IVec2, steps: u32) {
    for _ in 0..steps {
        let mut next = *here + dir;
        match resolve_step_p1(grid, &mut next, dir) {
            Wall => {}
            Ground => *here = next,

            Void => unreachable!("resolve_step_p1() resolved to Void, which shouldn't happen"),
        }
        debug_assert_eq!(grid[*here], Ground);
    }

    // Step once and figure out what kind of tile is there
    fn resolve_step_p1(grid: &Framebuffer<Tile>, next_point: &mut IVec2, dir: IVec2) -> Tile {
        // println!("Resolving {point:?} moving {dir:?}");
        match grid[*next_point] {
            Wall => Wall,
            Ground => Ground,
            Void => {
                // println!("Found Void at {point:?}");
                // We're currently already on Void, so step backwards once
                *next_point -= dir;

                // And walk backwards until we find Void again
                while grid[*next_point] != Void {
                    // and then use 1 step forward from there.
                    *next_point -= CUBE_SIDE * dir;
                }
                *next_point += dir;

                debug_assert_ne!(grid[*next_point], Void);
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

            debug_assert_ne!(dir, IVec2::zero());
            debug_assert_eq!(group.next(), None);
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
fn do_steps_p2(grid: &mut Framebuffer<Tile>, here: &mut IVec2, dir: IVec2, steps: u32) {
    for _ in 0..steps {
        let mut next = *here + dir;
        match resolve_step_p2(grid, &mut next, dir) {
            Wall => {
                // Cannot walk into wall, do nothing
            }

            Ground => {
                // Can walk onto ground, take the step
                *here = next;
            }

            Void => unreachable!("resolve_step_p2() resolved to Void, which shouldn't happen"),
        }
        debug_assert_eq!(grid[*here], Ground);
    }

    // Step once and figure out what kind of tile is there
    fn resolve_step_p2(grid: &Framebuffer<Tile>, next_point: &mut IVec2, dir: IVec2) -> Tile {
        // println!("Resolving {point:?} moving {dir:?}");
        match grid[*next_point] {
            Wall => Wall,
            Ground => Ground,
            Void => {
                println!("Found Void at {next_point:?}");
                // We're currently already on Void, so step backwards once
                *next_point -= dir;

                // And walk backwards until we find Void again
                while grid[*next_point] != Void {
                    // and then use 1 step forward from there.
                    *next_point -= CUBE_SIDE * dir;
                }
                *next_point += dir;

                debug_assert_ne!(grid[*next_point], Void);
                grid[*next_point]
            }
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
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
            do_steps_p2(&mut grid, &mut here, dir, steps);
        } else {
            let rot = *group.next().unwrap() as char;

            if rot == 'R' {
                dir.y = -dir.y;
                std::mem::swap(&mut dir.x, &mut dir.y);
            } else {
                dir.x = -dir.x;
                std::mem::swap(&mut dir.x, &mut dir.y);
            }

            debug_assert_ne!(dir, IVec2::zero());
            debug_assert_eq!(group.next(), None);
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
