#![warn(clippy::needless_range_loop, clippy::overly_complex_bool_expr)]

use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

aoc_lib! { year = 2017 }

#[allow(non_upper_case_globals)]
pub mod prelude {
    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub use bitmask_enum::*;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, log_enabled, trace, warn, Level::*};
    pub use ultraviolet::IVec2;

    pub use std::collections::*;

    pub use crate::Cardinal;
    pub const Norð: Cardinal = Cardinal::Norð;
    pub const Souð: Cardinal = Cardinal::Souð;
    pub const East: Cardinal = Cardinal::East;
    pub const West: Cardinal = Cardinal::West;

    pub use crate::print_with_focus;
}

use prelude::*;

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Cardinal {
    Norð,
    Souð,
    East,
    West,
}

impl Cardinal {
    pub const ALL_NO_DIAG: [Cardinal; 4] = [Norð, Souð, East, West];
    pub fn all_diag() -> [Cardinal; 8] {
        [
            Norð,
            Norð | East,
            East,
            Souð | East,
            Souð,
            Souð | West,
            West,
            Norð | West,
        ]
    }

    pub fn rev(&self) -> Self {
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

    pub fn ivec2(&self) -> IVec2 {
        (*self).into()
    }
}

impl From<Cardinal> for IVec2 {
    fn from(val: Cardinal) -> Self {
        let mut r = IVec2::zero();

        if val.contains(Norð) {
            r += IVec2::new(0, 1);
        }
        if val.contains(Souð) {
            r += IVec2::new(0, -1);
        }
        if val.contains(East) {
            r += IVec2::new(1, 0);
        }
        if val.contains(West) {
            r += IVec2::new(-1, 0);
        }

        r
    }
}

#[track_caller]
pub fn print_with_focus<T, U>(things: T, focus: impl TryInto<usize> + std::fmt::Display + Copy)
where
    T: IntoIterator<Item = U>,
    U: std::fmt::Debug,
{
    let focus: usize = focus
        .try_into()
        .unwrap_or_else(|_| unreachable!("Failed to parse {focus} into a usize"));

    print!("[");
    for (i, t) in things.into_iter().enumerate() {
        if i == focus {
            print!(" ({t:?})");
        } else {
            print!("  {t:?} ");
        }
    }
    println!(" ]");
}
