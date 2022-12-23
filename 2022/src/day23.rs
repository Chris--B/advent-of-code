#![allow(clippy::collapsible_if)]

use crate::prelude::*;

fn parse(input: &str) -> HashSet<IVec2> {
    let mut grid = HashSet::new();

    for (y, line) in input.lines().rev().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            if *b == b'#' {
                grid.insert(IVec2::new(x as i32, y as i32));
            }
        }
    }

    grid
}

const DIR_NW: IVec2 = IVec2::new(-1, 1);
const DIR_NE: IVec2 = IVec2::new(1, 1);
const DIR_N: IVec2 = IVec2::new(0, 1);

const DIR_SW: IVec2 = IVec2::new(-1, -1);
const DIR_SE: IVec2 = IVec2::new(1, -1);
const DIR_S: IVec2 = IVec2::new(0, -1);

const DIR_E: IVec2 = IVec2::new(1, 0);
const DIR_W: IVec2 = IVec2::new(-1, 0);

// Part1 ========================================================================
#[aoc(day23, part1)]
pub fn part1(input: &str) -> i32 {
    let mut grid: HashSet<IVec2> = parse(input);

    let mut queue: [[IVec2; 3]; 4] = [
        [DIR_N, DIR_NE, DIR_NW],
        [DIR_S, DIR_SE, DIR_SW],
        [DIR_W, DIR_NW, DIR_SW],
        [DIR_E, DIR_NE, DIR_SE],
    ];

    let (min_x, max_x) = (-3, 10);
    let (min_y, max_y) = (-3, 8);

    'rounds: for _round in 0..10 {
        if cfg!(test) {
            println!();
            println!("=== Round {_round} ====");

            println!();
            println!("x: {min_x}..{max_x} ({})", max_x - min_x);
            println!("y: {min_y}..{max_y} ({})", max_y - min_y);
            println!("{} elves", grid.len());
            for y in (min_y..=max_y).rev() {
                for x in min_x..=max_x {
                    if grid.contains(&(x, y).into()) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }

        let mut n_moves = 0;
        let mut moves = HashMap::<IVec2, u32>::new();

        // Each elf proposes a move to make
        'elves: for elf in grid.iter().copied() {
            // If the elf has NO neighbors, do nothing
            if elf.full_neighbors().iter().all(|x| !grid.contains(x)) {
                continue 'elves;
            }

            for [a, b, c] in &queue {
                if !grid.contains(&(elf + *a))
                    && !grid.contains(&(elf + *b))
                    && !grid.contains(&(elf + *c))
                {
                    *moves.entry(elf + *a).or_insert(0) += 1;
                    continue 'elves;
                }
            }

            // If all of that fails, our elf has no moves and stays put
        }

        let mut next_grid = HashSet::new();

        // Each elf makes that move if no one else tries to move there
        'elves: for elf in grid.iter().copied() {
            // If the elf has NO neighbors, do nothing
            if elf.full_neighbors().iter().all(|x| !grid.contains(x)) {
                next_grid.insert(elf);
                continue 'elves;
            }

            for [a, b, c] in &queue {
                if !grid.contains(&(elf + *a))
                    && !grid.contains(&(elf + *b))
                    && !grid.contains(&(elf + *c))
                {
                    // This is a valid move, so we will either move or not.
                    // We make that choice here.
                    if moves[&(elf + *a)] == 1 {
                        n_moves += 1;
                        next_grid.insert(elf + *a);
                    } else {
                        next_grid.insert(elf);
                    }

                    // And stop considering other moves
                    continue 'elves;
                }
            }

            // If all of that fails, our elf has no moves and stays put
            next_grid.insert(elf);
        }

        // "Rotate" the move-direction queue
        {
            let first = queue[0];
            queue.rotate_left(1);
            queue[3] = first;
        }

        debug_assert_eq!(
            grid.len(),
            next_grid.len(),
            "Conservation of Elf has failed!"
        );
        grid = next_grid;

        if n_moves == 0 {
            println!("After round #{_round}, no elves are moving");
            break 'rounds;
        }
    }

    let (min_x, max_x) = grid
        .iter()
        .copied()
        .map(|v| v.x)
        .minmax()
        .into_option()
        .unwrap();

    let (min_y, max_y) = grid
        .iter()
        .copied()
        .map(|v| v.y)
        .minmax()
        .into_option()
        .unwrap();

    if cfg!(test) {
        println!();
        println!("x: {min_x}..{max_x} ({})", max_x - min_x);
        println!("y: {min_y}..{max_y} ({})", max_y - min_y);
        println!("{} elves", grid.len());
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if grid.contains(&(x, y).into()) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    (max_x - min_x + 1) * (max_y - min_y + 1) - grid.len() as i32
}

// Part2 ========================================================================
#[aoc(day23, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid: HashSet<IVec2> = parse(input);

    let mut queue: [[IVec2; 3]; 4] = [
        [DIR_N, DIR_NE, DIR_NW],
        [DIR_S, DIR_SE, DIR_SW],
        [DIR_W, DIR_NW, DIR_SW],
        [DIR_E, DIR_NE, DIR_SE],
    ];

    for round in 0.. {
        if cfg!(test) {
            let (min_x, max_x) = (-3, 10);
            let (min_y, max_y) = (-3, 8);

            println!();
            println!("=== Round {round} ====");

            println!();
            println!("x: {min_x}..{max_x} ({})", max_x - min_x);
            println!("y: {min_y}..{max_y} ({})", max_y - min_y);
            println!("{} elves", grid.len());
            for y in (min_y..=max_y).rev() {
                for x in min_x..=max_x {
                    if grid.contains(&(x, y).into()) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }

        let mut n_moves = 0;
        let mut moves = HashMap::<IVec2, u32>::new();

        // Each elf proposes a move to make
        'elves: for elf in grid.iter().copied() {
            // If the elf has NO neighbors, do nothing
            if elf.full_neighbors().iter().all(|x| !grid.contains(x)) {
                continue 'elves;
            }

            for [a, b, c] in &queue {
                if !grid.contains(&(elf + *a))
                    && !grid.contains(&(elf + *b))
                    && !grid.contains(&(elf + *c))
                {
                    *moves.entry(elf + *a).or_insert(0) += 1;
                    continue 'elves;
                }
            }

            // If all of that fails, our elf has no moves and stays put
        }

        let mut next_grid = HashSet::new();

        // Each elf makes that move if no one else tries to move there
        'elves: for elf in grid.iter().copied() {
            // If the elf has NO neighbors, do nothing
            if elf.full_neighbors().iter().all(|x| !grid.contains(x)) {
                next_grid.insert(elf);
                continue 'elves;
            }

            for [a, b, c] in &queue {
                if !grid.contains(&(elf + *a))
                    && !grid.contains(&(elf + *b))
                    && !grid.contains(&(elf + *c))
                {
                    // This is a valid move, so we will either move or not.
                    // We make that choice here.
                    if moves[&(elf + *a)] == 1 {
                        n_moves += 1;
                        next_grid.insert(elf + *a);
                    } else {
                        next_grid.insert(elf);
                    }

                    // And stop considering other moves
                    continue 'elves;
                }
            }

            // If all of that fails, our elf has no moves and stays put
            next_grid.insert(elf);
        }

        // "Rotate" the move-direction queue
        {
            let first = queue[0];
            queue.rotate_left(1);
            queue[3] = first;
        }

        debug_assert_eq!(
            grid.len(),
            next_grid.len(),
            "Conservation of Elf has failed!"
        );
        grid = next_grid;

        if n_moves == 0 {
            if cfg!(debug_assertions) {
                let (min_x, max_x) = grid
                    .iter()
                    .copied()
                    .map(|v| v.x)
                    .minmax()
                    .into_option()
                    .unwrap();

                let (min_y, max_y) = grid
                    .iter()
                    .copied()
                    .map(|v| v.y)
                    .minmax()
                    .into_option()
                    .unwrap();

                println!("x: {min_x}..{max_x} ({})", max_x - min_x);
                println!("y: {min_y}..{max_y} ({})", max_y - min_y);
            }

            return round + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    const EXAMPLE_INPUT_TINY: &str = r"
.....
..##.
..#..
.....
..##.
.....
";

    #[rstest]
    #[case::given(110, EXAMPLE_INPUT)]
    #[case::given_tiny(25, EXAMPLE_INPUT_TINY)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(20, EXAMPLE_INPUT)]
    #[case::given_tiny(4, EXAMPLE_INPUT_TINY)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
