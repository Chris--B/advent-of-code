use crate::prelude::*;

// The chamber is 7 units wide, so we'll use u8 as a bit set with bits 0..=6
type Row = u8;

#[inline(always)]
fn print_board(rows: &[Row]) {
    if !cfg!(debug_assertions) || rows.len() > 25 {
        return;
    }

    for (y, row) in rows.iter().enumerate().skip(1).rev() {
        if row & 0b1000_0000 != 0 {
            print!(" ! |");
        } else {
            print!("   |");
        }
        for b in (0..7).rev() {
            if row & (1 << b) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("| {y}");
        println!();
    }
    println!("   +-------+");
    println!();
}

fn add_shape(rows: &mut [Row], x: i8, y: usize, shape: &[Row]) {
    for (yy, line_without_any_shifts) in shape.iter().copied().enumerate() {
        if line_without_any_shifts == 0 {
            continue;
        }

        let y = y + yy;
        if y == rows.len() {
            break;
        }

        let l = line_without_any_shifts << x;
        debug_assert_ne!(l, 0);
        debug_assert_eq!(l.count_ones(), line_without_any_shifts.count_ones());

        debug_assert_eq!(rows[y] & l, 0, "\nyy={yy}\n0b{l:07b}\n0b{:07b}", rows[y],);

        rows[y] |= l;
    }
}

fn run_sim(input: &[u8], times: usize) -> i64 {
    let mut rows: Vec<Row> = Vec::with_capacity(1_024);

    // Init the rows with a solid floor
    rows.push(0b0111_1111);

    // Shapes take many forms but all are as far to the right as they can be.
    const SHAPES: &[((usize, usize), [Row; 4])] = &[
        // ####
        ((4, 1), [0b_1111, 0, 0, 0]),
        // .#.
        // ###
        // .#.
        ((3, 3), [0b_010, 0b_111, 0b_010, 0]),
        // ..#
        // ..#
        // ###
        ((3, 3), [0b_111, 0b_001, 0b_001, 0]), // Note: Bottom row stored first
        // #
        // #
        // #
        // #
        ((1, 4), [0b_1, 0b_1, 0b_1, 0b_1]),
        // ##
        // ##
        ((2, 2), [0b_11, 0b_11, 0, 0]),
    ];

    let mut jets = input.iter().copied().cycle();

    let mut seen_runs = HashMap::<Vec<_>, (usize, usize)>::new();

    let mut times_left = times;
    let mut sum = 0;

    for (time, ((sx, sy), shape)) in SHAPES.iter().copied().cycle().enumerate() {
        if times_left == 0 {
            break;
        }
        times_left -= 1;

        let sx = sx as i8;

        // Each rock appears so that:
        //      its left edge is two units away from the left wall,
        //      its bottom edge is three units above the highest rock in the room
        //          (or the floor, if there isn't one)
        let mut x: i8 = (7 - 2) - sx; // steps from the right
        let mut y = rows.len() + 3; // steps from the floor

        // Pad out empty rows so we can index freely
        while rows.len() < (y + sy) {
            rows.push(0);
        }

        if cfg!(debug_assertions) {
            println!("A rock begins falling:");
            let mut rows = rows.clone();
            add_shape(&mut rows, x, y, &shape);
            while rows.last() == Some(&0) {
                rows.pop();
            }

            print_board(&rows);
        }

        // Check if we can move down
        'falling: loop {
            // Oh no! The jet pushed our rock, but not through the walls
            {
                let jet = jets.next().unwrap();
                debug_assert!(jet == b'<' || jet == b'>');

                let old_x = x;
                if jet == b'<' {
                    x = (x + 1).clamp(0, 7 - sx);
                } else {
                    x = (x - 1).clamp(0, 7 - sx);
                }

                if cfg!(debug_assertions) {
                    print!("({x}, {y}) Jet of gas pushes rock ");
                    if jet == b'<' {
                        print!("left");
                    } else {
                        print!("right");
                    }
                }

                // The shift is in bounds, but we need to check this row for new collisions too.

                let can_be_placed =
                    shape
                        .iter()
                        .enumerate()
                        .take(sy)
                        .all(|(yy, line_without_any_shifts)| {
                            let y = y + yy;
                            let l = line_without_any_shifts << x;
                            debug_assert_eq!(
                                l.count_ones(),
                                line_without_any_shifts.count_ones(),
                                "Shifting moved it off the edge of the board!"
                            );

                            // If we placed this line of the shape, would it overlap with anything?
                            // If no, this is 0 and we report this line as "can fall".
                            (rows[y] & l) == 0
                        });
                if !can_be_placed {
                    x = old_x;
                }

                if cfg!(debug_assertions) {
                    if x == old_x {
                        print!(", but nothing happens");
                    } else {
                        print!(":");
                    }
                    println!();
                }

                if cfg!(debug_assertions) {
                    let mut rows = rows.clone();
                    add_shape(&mut rows, x, y, &shape);
                    print_board(&rows);
                }
            }

            let can_fall =
                shape
                    .iter()
                    .enumerate()
                    .take(sy)
                    .all(|(yy, line_without_any_shifts)| {
                        let y = y + yy - 1;
                        let l = line_without_any_shifts << x;
                        debug_assert_eq!(
                            l.count_ones(),
                            line_without_any_shifts.count_ones(),
                            "Shifting moved it off the edge of the board!"
                        );

                        // If we placed this line of the shape, would it overlap with anything?
                        // If no, this is 0 and we report this line as "can fall".
                        (rows[y] & l) == 0
                    });

            if can_fall {
                // Rock falls 1 unit
                y -= 1;

                if cfg!(debug_assertions) {
                    println!("Rock falls 1 unit:");
                    let mut rows = rows.clone();
                    add_shape(&mut rows, x, y, &shape[..sy]);
                    print_board(&rows);
                }

                continue 'falling;
            } else {
                // Rock comes to rest
                // println!("Rock falls 1 unit, causing it to come to rest:");
                break 'falling;
            }
        }

        // Place the falling rock
        add_shape(&mut rows, x, y, &shape[..sy]);

        // Check if the last 100 runs have been seen before
        if rows.len() > 100 && sum == 0 {
            use std::collections::hash_map::Entry::Occupied;

            let len = rows.len() - 1;
            let t = rows.len() - 100;
            let run = rows[t..].to_owned();
            let e = seen_runs.entry(run);

            if let Occupied(oe) = e {
                let (start_time, start_len) = *oe.get();
                let (end_time, end_len) = (time, len);
                let (times_per_cycle, len_per_cycle) = (end_time - start_time, end_len - start_len);

                let n_cycles = (times - start_time) / (times_per_cycle);

                let times_from_cycles = n_cycles * times_per_cycle; // integer rounding makes this != times
                let times_after_cycles = times - times_from_cycles - start_time - 1;

                // dbg!(
                //     start_time,
                //     end_time,
                //     start_len,
                //     end_len,
                //     times_per_cycle,
                //     n_cycles,
                //     len_from_cycles,
                //     times_from_cycles,
                //     times_after_cycles,
                // );

                sum = (len_per_cycle * (n_cycles - 1)) as i64;
                times_left = times_after_cycles;
            } else {
                e.or_insert((time, len));
            }
        }

        // Clear any empty rows on top, our placement logic depends on it
        while rows.last() == Some(&0) {
            rows.pop();
        }
    }

    // We pad our length with a fake floor, so adjust here
    sum + rows.len() as i64 - 1
}

// Part1 ========================================================================
#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    run_sim(input.as_bytes(), 2022)
}

// Part2 ========================================================================
#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    run_sim(input.as_bytes(), 1_000_000_000_000)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[rstest]
    #[case::given(3068, EXAMPLE_INPUT)]
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

    #[rstest]
    #[case::given(1_514_285_714_288, EXAMPLE_INPUT)]
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
