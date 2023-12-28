#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]
#![allow(clippy::comparison_chain)]
#![allow(clippy::identity_op)]
#![warn(clippy::overly_complex_bool_expr)]
#![allow(clippy::single_element_loop)]
#![warn(clippy::if_same_then_else)]

use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day08_buf;
pub mod day09;
pub mod day10;
pub mod day11;
// pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
pub mod day19;
// pub mod day20;
pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
pub mod day25;

mod framebuffer;

aoc_lib! { year = 2023 }

// Run this function when the binary is loaded. This typically happens BEFORE MAIN.
// This is a BAD IDEA, but cargo-aoc doesn't give us hooks anywhere else. So it's this or lazy-init in EVERY solution 😬.
#[ctor::ctor]
fn init_logging() {
    use env_logger::{Builder, Env};

    let mut env = Env::default();
    if cfg!(test) || cfg!(debug_assert) {
        // Debug and test builds should log MORE
        env = env.default_filter_or("debug");
    } else {
        // Everyone else can log warn and above
        env = env.default_filter_or("warn");
    }

    Builder::from_env(env)
        .is_test(cfg!(test))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_indent(Some(4))
        .init();
}

#[allow(unused_imports, non_upper_case_globals)]
mod prelude {
    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub use bitmask_enum::bitmask;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, log_enabled, trace, warn, Level::*};
    pub use num::Complex;
    pub use num::Integer;
    pub use scan_fmt::scan_fmt;
    pub use smallvec::{smallvec, SmallVec};
    pub use ultraviolet::IVec2;

    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    pub use std::num::Wrapping;

    pub use crate::framebuffer::Framebuffer;
    pub use crate::Cardinal;
    pub const Norð: Cardinal = Cardinal::Norð;
    pub const Souð: Cardinal = Cardinal::Souð;
    pub const East: Cardinal = Cardinal::East;
    pub const West: Cardinal = Cardinal::West;

    pub use crate::fast_parse_u32;
    pub use crate::fast_parse_u64;
    pub use crate::fast_parse_u8;
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

pub fn fast_parse_u8(input: &[u8]) -> u32 {
    if cfg!(debug_assertions) {
        for c in input {
            assert!(c.is_ascii_digit());
        }
        assert!(
            input.len() < 3,
            "input expects a 2 digit int but found {:?}",
            std::str::from_utf8(input)
        );
    }
    let mut digits = [0_u32; 2];
    let mut x = 1;
    for (i, b) in input.iter().rev().enumerate() {
        digits[i] = x * (*b - b'0') as u32;
        x *= 10;
    }

    digits.into_iter().sum()
}

pub fn fast_parse_u32(input: &[u8]) -> u32 {
    let mut digits = [0_u32; 7];
    debug_assert!(
        input.len() <= digits.len(),
        "Expected {} digits but now need to support {}",
        digits.len(),
        input.len(),
    );

    let mut x = 1;
    for (i, b) in input.iter().rev().enumerate() {
        digits[i] = x * (*b - b'0') as u32;
        x *= 10;
    }

    digits.into_iter().sum()
}

pub fn fast_parse_u64<I>(input: I) -> u64
where
    I: Iterator<Item = u8> + std::iter::DoubleEndedIterator,
{
    let mut digits = [0_u64; 19];

    let mut x = 1;
    for (i, b) in input.rev().enumerate() {
        digits[i] = x * (b - b'0') as u64;
        x *= 10;
    }

    digits.into_iter().sum()
}
