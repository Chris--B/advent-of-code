#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]
#![allow(clippy::comparison_chain)]
#![allow(clippy::identity_op)]

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

mod framebuffer;

aoc_lib! { year = 2023 }

// Run this function when the binary is loaded. This typically happens BEFORE MAIN.
// This is a BAD IDEA, but cargo-aoc doesn't give us hooks anywhere else. So it's this or lazy-init in EVERY solution 😬.
#[ctor::ctor]
fn init_logging() {
    use env_logger::{Builder, Env};
    use prelude::*;

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

    trace!("Hello");
    debug!("Hello");
    info!("Hello");
    warn!("Hello");
    error!("Hello");
}

#[allow(unused_imports)]
mod prelude {

    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub use bitmask_enum::bitmask;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, log_enabled, trace, warn, Level::*};
    pub use num::Complex;
    pub use num::Integer;
    pub use scan_fmt::scan_fmt;

    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    pub use std::num::Wrapping;

    pub use crate::fast_parse_u32;
    pub use crate::fast_parse_u64;
    pub use crate::fast_parse_u8;
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
