use aoc_runner_derive::aoc;

use smallvec::{smallvec, SmallVec};

#[derive(Copy, Clone, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

// See: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.trim_ascii_start
// Using this for stable
const fn trim_ascii_start(mut bytes: &[u8]) -> &[u8] {
    // Note: A pattern matching based approach (instead of indexing) allows
    // making the function const.
    while let [first, rest @ ..] = bytes {
        if first.is_ascii_whitespace() {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}

fn fast_parse_u32(input: &[u8]) -> u32 {
    debug_assert!(input.len() <= 2);

    let mut digits = [0_u32; 10];
    let mut x = 1;
    for (i, b) in input.iter().rev().enumerate() {
        digits[i] = x * (*b - b'0') as u32;
        x *= 10;
    }

    digits.into_iter().sum()
}

fn parse_to_move(input: &str) -> State {
    let input = input.as_bytes();

    let crate_lines: Vec<&[u8]> = input
        .split(|b| *b == b'\n')
        .filter(|l| trim_ascii_start(l).starts_with(b"["))
        .rev()
        .collect();

    let moves: Vec<Move> = input
        .split(|b| *b == b'\n')
        .filter(|line| line.starts_with(b"move"))
        .map(|line| {
            let mut parts = line.split(|b| *b == b' ');

            let _ = parts.next(); // "move"
            let count = fast_parse_u32(parts.next().unwrap()) as usize;

            let _ = parts.next(); // "from"
            let from = fast_parse_u32(parts.next().unwrap()) as usize;

            let _ = parts.next(); // "to"
            let to = fast_parse_u32(parts.next().unwrap()) as usize;

            // This is a nonsense move, and could break some of our logic below.
            debug_assert_ne!(from, to);

            Move { count, from, to }
        })
        .collect();

    let mut stacks = vec![vec![]; crate_lines.len() + 1];

    for line in crate_lines {
        for (i, c) in line.chunks(4).enumerate() {
            let c = c[1] as char;

            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    State { stacks, moves }
}

// Part1 ========================================================================
#[aoc(day5, part1, move_crates)]
#[inline(never)]
pub fn part1(input: &str) -> String {
    let mut state = parse_to_move(input);

    for Move { count, from, to } in state.moves {
        debug_assert!(state.stacks[from - 1].len() >= count);

        // Copy `count` items from the top of one stack, onto the other, reversing their order.
        for _ in 0..count {
            let c = state.stacks[from - 1].pop().unwrap();
            state.stacks[to - 1].push(c);
        }
    }

    state
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

fn track_crates(input: &str, is_part1: bool) -> String {
    let input = input.as_bytes();

    let (crates_max_height, stack_count) = input
        .split(|b| *b == b'\n')
        .enumerate()
        .find(|(_i, l)| l.starts_with(b" 1") || l.starts_with(b" 0"))
        .map(|(i, l)| (i, l.iter().filter(|b| **b != b' ').count()))
        .unwrap();

    assert_ne!(stack_count, 0);

    #[derive(Copy, Clone, Debug)]
    struct Crate {
        stack: usize,
        idx: usize,
    }

    let mut crates: SmallVec<[Crate; 10]> = smallvec![];
    for stack in 0..stack_count {
        crates.push(Crate { stack, idx: 0 })
    }

    let mut stacks = vec![vec![]; stack_count];

    for line in input.split(|b| *b == b'\n').take(crates_max_height) {
        for (stack, letter) in line.chunks(4).enumerate() {
            let c = letter[1];
            if c == b' ' {
                continue;
            }

            stacks[stack].push(c as char);
        }
    }

    let moves = input
        .split(|b| *b == b'\n')
        .rev()
        .filter(|line| !line.is_empty())
        .take_while(|line| !line.starts_with(b" 1"))
        .map(|line| {
            let mut parts = line.split(|b| *b == b' ');

            let _ = parts.next(); // "move"
            let count = fast_parse_u32(parts.next().unwrap()) as usize;

            let _ = parts.next(); // "from"
            let from = fast_parse_u32(parts.next().unwrap()) as usize - 1;

            let _ = parts.next(); // "to"
            let to = fast_parse_u32(parts.next().unwrap()) as usize - 1;

            debug_assert_ne!(from, to);

            Move { count, from, to }
        });

    for Move { count, from, to } in moves {
        for c in &mut crates {
            if c.stack == to {
                if c.idx >= count {
                    // Crate doesn't move, adjust position
                    c.idx -= count;
                } else {
                    // Crate moves
                    c.stack = from;
                    if is_part1 {
                        c.idx = count - c.idx - 1;
                    }
                }
            } else if c.stack == from {
                // Crate doesn't move, adjust position
                c.idx += count;
            }
        }
    }

    crates
        .into_iter()
        .map(|c| {
            if let Some(c) = stacks[c.stack].get(c.idx) {
                *c
            } else {
                '_'
            }
        })
        .collect()
}

#[aoc(day5, part1, track_crates)]
#[inline(never)]
pub fn part1_track(input: &str) -> String {
    track_crates(input, true)
}

#[aoc(day5, part2, track_crates)]
#[inline(never)]
pub fn part2_track(input: &str) -> String {
    track_crates(input, false)
}

// Part2 ========================================================================
#[aoc(day5, part2, move_crates)]
#[inline(never)]
pub fn part2(input: &str) -> String {
    let mut state = parse_to_move(input);

    for Move { count, from, to } in state.moves {
        debug_assert!(state.stacks[from - 1].len() >= count);

        // Because both stacks are in the same object (state.stacks), we cannot statically
        // convince the compiler that they don't alias are aren't allowed to create two &mut that
        // live at the same time.
        // We know however, that they do not overlap so we'll skip the borrow checker and create
        // our own mutable references.
        unsafe {
            let stacks_from = &mut *state.stacks.as_mut_ptr().add(from - 1);
            let stacks_to = &mut *state.stacks.as_mut_ptr().add(to - 1);

            // Copy `count` items from the top of one stack, onto the other, preserving their order.
            // (Note: Part 1 reverses their order)
            let t = stacks_from.len() - count;
            stacks_to.extend(&stacks_from[t..]);
            stacks_from.resize(t, '@');
        }
    }

    state
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[rstest]
    #[case::given("CMZ", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_track)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given("MCD", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_track)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }
}
