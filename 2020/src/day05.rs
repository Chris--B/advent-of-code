#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};

use std::convert::TryInto;

#[derive(Debug)]
pub struct Seat {
    ticket: [u8; 10],
    id: u16,
}

impl Seat {
    fn new(s: &str) -> Self {
        let ticket: [u8; 10] = s.as_bytes().try_into().unwrap();
        let mut buf: [u8; 10] = ticket;

        // Replace the letters with binary 1s and 0s so we can parse it
        for c in &mut buf {
            match *c {
                b'R' | b'B' => *c = b'1',
                b'L' | b'F' => *c = b'0',
                _ => {
                    unreachable!()
                }
            }
        }

        // The ticket is just the id in binary, so extract that now.
        let s = unsafe { std::str::from_utf8_unchecked(&buf) };
        let id = u16::from_str_radix(s, 2).unwrap();

        Seat { ticket, id }
    }

    fn loc(&self) -> (u8, u8) {
        let row = (self.id >> 3) as u8;
        let col = (self.id & 0b111) as u8;

        (row, col)
    }

    fn id(&self) -> usize {
        self.id as usize
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Seat> {
    input.lines().map(|line| Seat::new(line.trim())).collect()
}

// Part1 ======================================================================
#[aoc(day5, part1)]
pub fn part1(input: &[Seat]) -> usize {
    input.iter().map(|s| s.id()).max().unwrap_or_default()
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

    0
}
