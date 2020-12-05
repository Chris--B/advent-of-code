#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};

use std::convert::TryInto;

#[derive(Debug)]
pub struct Seat {
    ticket: [u8; 10],
    row: u8,
    col: u8,
}

impl Seat {
    fn new(s: &str) -> Self {
        let ticket: [u8; 10] = s.as_bytes().try_into().unwrap();
        let mut buf: [u8; 10] = ticket;

        // Replace the letters with binary numbers
        for c in &mut buf {
            match *c {
                b'R' | b'B' => *c = b'1',
                b'L' | b'F' => *c = b'0',
                _ => {}
            }
        }

        // And parse like numbers
        let (row, col) = buf.split_at(7);
        let row = unsafe { std::str::from_utf8_unchecked(&row) };
        let col = unsafe { std::str::from_utf8_unchecked(&col) };

        Seat {
            ticket,
            row: u8::from_str_radix(row, 2).unwrap(),
            col: u8::from_str_radix(col, 2).unwrap(),
        }
    }

    fn loc(&self) -> (u8, u8) {
        (self.row, self.col)
    }

    fn id(&self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Seat> {
    input.lines().map(|line| Seat::new(line.trim())).collect()
}

// Part1 ======================================================================
#[aoc(day5, part1)]
pub fn part1(input: &[Seat]) -> usize {
    input.iter().map(|s| s.id()).max().unwrap()
}

#[test]
fn check_part1_ex_passes() {
    assert_eq!(Seat::new("FBFBBFFRLR").loc(), (44, 5));
    assert_eq!(Seat::new("BFFFBBFRRR").loc(), (70, 7));
    assert_eq!(Seat::new("FFFBBBFRRR").loc(), (14, 7));
    assert_eq!(Seat::new("BBFFBBFRLL").loc(), (102, 4));
}

// Part2 ======================================================================
#[aoc(day5, part2)]
pub fn part2(input: &[Seat]) -> usize {
    let mut seats = [false; 1_024];

    for seat in input {
        seats[seat.id()] = true;
    }

    for id in 1..(seats.len()) {
        // If this seat is taken, keep going
        if seats.get(id) == Some(&true) {
            continue;
        }

        // Check our neighbors' seats - they should be full.
        let (a, b) = (id - 1, id + 1);
        if let (Some(true), Some(true)) = (seats.get(a), seats.get(b)) {
            return id;
        }
    }

    unreachable!("Didn't find a solution")
}
