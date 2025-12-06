#![allow(
    clippy::assign_op_pattern,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::identity_op,
    clippy::inconsistent_digit_grouping,
    clippy::iter_nth_zero,
    clippy::nonminimal_bool,
    clippy::redundant_pattern_matching,
    clippy::single_element_loop
)]
#![warn(
    clippy::overly_complex_bool_expr,
    clippy::if_same_then_else,
    clippy::never_loop
)]

use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

aoc_lib! { year = 2025 }

pub fn init_logging() {
    static LOGGING: std::sync::Once = std::sync::Once::new();

    LOGGING.call_once(|| {
        println!("logging init'd");
        use env_logger::{Builder, Env};

        let mut env = Env::default();
        if cfg!(test) || cfg!(debug_assertions) {
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
    });
}

#[allow(non_upper_case_globals)]
pub mod prelude {
    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub use bitmask_enum::bitmask;
    pub use image::{Rgb, RgbImage};
    pub use itertools::Itertools;
    pub use log::{Level::*, debug, error, info, log_enabled, trace, warn};
    pub use memchr::*;
    pub use num::*;
    pub use regex::{Regex, RegexBuilder};
    pub use scan_fmt::scan_fmt;
    pub use smallvec::SmallVec;
    pub use smallvec::smallvec;
    pub use ultraviolet::{IVec2, IVec3, UVec2};

    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    pub use std::num::Wrapping;
    pub use std::time::Duration;

    pub const Norð: Cardinal = Cardinal::Norð;
    pub const Souð: Cardinal = Cardinal::Souð;
    pub const East: Cardinal = Cardinal::East;
    pub const West: Cardinal = Cardinal::West;

    pub const AOC_BLUE: Rgb<u8> = Rgb([0x0f, 0x0f, 0x23]);
    pub const AOC_GOLD: Rgb<u8> = Rgb([0xff, 0xff, 0x66]);
    pub const AOC_LIGHT_GREEN: Rgb<u8> = Rgb([0x00, 0xcc, 0x00]);
    pub const AOC_DARK_GREEN: Rgb<u8> = Rgb([0x00, 0x99, 0x00]);
    pub const AOC_DARK_GRAY: Rgb<u8> = Rgb([0x52, 0x52, 0x5b]);
    pub const AOC_LIGHT_GRAY: Rgb<u8> = Rgb([0xcc, 0xcc, 0xcc]);
    pub const START_GREEN: Rgb<u8> = Rgb([0x66, 0xc2, 0xa5]);
    pub const FINAL_RED: Rgb<u8> = Rgb([0x9e, 0x01, 0x42]);

    pub use crate::init_logging;
    pub use crate::util::*;
}

pub mod util;
