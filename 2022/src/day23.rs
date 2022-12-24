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

fn parse_nohash(input: &str) -> Vec<IVec2> {
    let mut elves = vec![];

    for (y, line) in input.lines().rev().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            if *b == b'#' {
                elves.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    elves
}

#[derive(Copy, Clone, Debug, Default)]
struct PerElf(u16);

const MASK_ROUND: u16 = 0b1111_1111_1111_0000;
const MASK_HAS_ELF: u16 = 0b_1000;
const MASK_MOVES: u16 = 0b_0111;

impl PerElf {
    fn new(round: u16) -> Self {
        let mut s = Self(0);
        s.set_round(round);

        s
    }

    fn round(&self) -> u16 {
        self.0 >> MASK_ROUND.trailing_zeros()
    }

    fn moves(&self) -> u16 {
        self.0 & MASK_MOVES
    }

    fn has_elf(&self, round: u16) -> bool {
        if self.round() != round {
            false
        } else {
            (self.0 & MASK_HAS_ELF) != 0
        }
    }

    fn set_round(&mut self, round: u16) {
        self.0 |= round << MASK_ROUND.trailing_zeros();
        debug_assert_eq!(self.round(), round);
    }

    fn inc_moves(&mut self) {
        debug_assert!(self.moves() + 1 < (1 << MASK_MOVES.count_ones()));
        self.0 += 1;
    }

    fn set_has_elf(&mut self) {
        self.0 |= MASK_HAS_ELF;
    }

    fn clear_has_elf(&mut self) {
        self.0 &= !MASK_HAS_ELF;
    }
}

fn run_sim_nohash(elves: &mut Vec<IVec2>, stop_round: Option<u16>) -> i32 {
    const Q: [[IVec2; 3]; 4] = [
        [DIR_N, DIR_NE, DIR_NW],
        [DIR_S, DIR_SE, DIR_SW],
        [DIR_W, DIR_NW, DIR_SW],
        [DIR_E, DIR_NE, DIR_SE],
    ];

    let mut grid = Framebuffer::<PerElf>::new_with_ranges_square(-200..200);

    for elf in elves.iter() {
        grid[*elf] = PerElf::default();
    }

    let mut final_round = 0;
    let mut maybe_moving_elves = vec![];

    if cfg!(test) {
        let (min_x, max_x) = (-3, 10);
        let (min_y, max_y) = (-3, 8);

        println!("x: {min_x}..{max_x} ({})", max_x - min_x);
        println!("y: {min_y}..{max_y} ({})", max_y - min_y);
        println!();

        println!("=== Initial State ====");
        println!("{} elves", elves.len());
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if elves.contains(&(x, y).into()) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    'rounds: for round in 1.. {
        // Check conservation of elf
        if cfg!(debug_assertions) {
            debug_assert_eq!(maybe_moving_elves.len(), 0);
            debug_assert_eq!(elves.len(), elves.iter().unique().count());
        }

        final_round = round as i32;
        if let Some(stop_round) = stop_round {
            if round > stop_round {
                if cfg!(test) {
                    println!("Hit stopping round ({stop_round}), bailing");
                }

                break 'rounds;
            }
        }

        // Reset our per elf state
        for elf in elves.iter() {
            grid[*elf] = PerElf::new(round);
            grid[*elf].set_has_elf();
        }

        // Each elf proposes a move to make
        'elves: for (elf_idx, elf) in elves.iter().copied().enumerate() {
            // If our elf has no neighbors, he doesn't do anything this round!
            if elf.full_neighbors().iter().all(|v| !grid[v].has_elf(round)) {
                continue 'elves;
            }

            // Each elf follows a fixed order to find a move.
            // This order rotates every round. We do that with index trickery
            for i in 0..Q.len() {
                let [a, b, c] = Q[(i + round as usize - 1) % Q.len()];

                if !grid[elf + a].has_elf(round)
                    && !grid[elf + b].has_elf(round)
                    && !grid[elf + c].has_elf(round)
                {
                    if grid[elf + a].round() != round {
                        // This data is stale! Reset it real quick
                        grid[elf + a] = PerElf::new(round);
                    }

                    if grid[elf + a].moves() == 0 {
                        // If we're the first here, we MIGHT move
                        // Note: We'll still have to check the move count before
                        // we can resolve this as a move.
                        maybe_moving_elves.push((elf_idx, a));
                    }

                    // Propose this move.
                    grid[elf + a].inc_moves();

                    // Because this move was valid, this elf won't look any further
                    continue 'elves;
                }

                // Our elf has no moves to make and stays put
            }
        }

        // Now that each elf has proposed their move, let's resolve
        let mut elves_that_moved = 0;
        for (elf_idx, a) in maybe_moving_elves.drain(..) {
            let elf: &mut IVec2 = &mut elves[elf_idx];
            debug_assert!(!grid[*elf + a].has_elf(round));

            // Check that we're the only one moving to this spot. If we are, we'll execute the move.
            // Otherwise nothing happens here (and the elf stays put)
            if grid[*elf + a].moves() == 1 {
                // We were the only ones to propose here, so we get it!
                elves_that_moved += 1;

                debug_assert_eq!(grid[*elf].round(), round);
                grid[*elf].clear_has_elf();
                *elf += a;
                grid[*elf].set_has_elf();
            }
        }

        if cfg!(test) {
            let (min_x, max_x) = (-3, 10);
            let (min_y, max_y) = (-3, 8);

            println!("x: {min_x}..{max_x} ({})", max_x - min_x);
            println!("y: {min_y}..{max_y} ({})", max_y - min_y);
            println!();

            println!("=== Round {round} ====");
            println!("{} elves", elves.len());
            for y in (min_y..=max_y).rev() {
                for x in min_x..=max_x {
                    if elves.contains(&(x, y).into()) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }

        if elves_that_moved == 0 {
            if cfg!(test) {
                println!("No elves moved this round ({round}), bailing");
            }

            break 'rounds;
        }
    }

    final_round
}

#[aoc(day23, part1, nohash)]
pub fn part1_nohash(input: &str) -> i32 {
    let mut elves = parse_nohash(input);
    let _round = run_sim_nohash(&mut elves, Some(10));

    let (min_x, max_x) = elves
        .iter()
        .copied()
        .map(|v| v.x)
        .minmax()
        .into_option()
        .unwrap();

    let (min_y, max_y) = elves
        .iter()
        .copied()
        .map(|v| v.y)
        .minmax()
        .into_option()
        .unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}
#[aoc(day23, part2, nohash)]
pub fn part2_nohash(input: &str) -> i32 {
    let mut elves = parse_nohash(input);
    run_sim_nohash(&mut elves, None)
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
        #[values(part1, part1_nohash)]
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
        #[values(part2, part2_nohash)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
