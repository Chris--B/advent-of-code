use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
// pub mod day03;
// pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
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

    pub use log::{debug, error, info, log_enabled, trace, warn, Level::*};

    pub use itertools::Itertools;

    pub use crate::Cardinal;
    pub const Norð: Cardinal = Cardinal::Norð;
    pub const Souð: Cardinal = Cardinal::Souð;
    pub const East: Cardinal = Cardinal::East;
    pub const West: Cardinal = Cardinal::West;

    pub use bitmask_enum::*;
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
