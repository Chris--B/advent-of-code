#![allow(
    clippy::comparison_chain,
    clippy::iter_nth_zero,
    clippy::identity_op,
    clippy::single_element_loop
)]
#![warn(clippy::overly_complex_bool_expr, clippy::if_same_then_else)]

use std::mem::MaybeUninit;
use std::str::FromStr;

use aoc_runner_derive::aoc_lib;

pub mod day01;
// pub mod day02;
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

aoc_lib! { year = 2024 }

// Run this function when the binary is loaded. This typically happens BEFORE MAIN.
// This is a BAD IDEA, but cargo-aoc doesn't give us hooks anywhere else. So it's this or lazy-init in EVERY solution 😬.
#[ctor::ctor]
fn init_logging() {
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
    pub use ultraviolet::{IVec2, IVec3};

    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    pub use std::num::Wrapping;

    pub use crate::Cardinal;
    pub const Norð: Cardinal = Cardinal::Norð;
    pub const Souð: Cardinal = Cardinal::Souð;
    pub const East: Cardinal = Cardinal::East;
    pub const West: Cardinal = Cardinal::West;

    pub use crate::parse_list;
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

// TODO: Use Pattern when it's stable, https://doc.rust-lang.org/std/str/pattern/index.html
pub fn parse_list<const N: usize, T>(s: &str, pattern: &str) -> [T; N]
where
    T: FromStr + Copy + Sized,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let ty_name = std::any::type_name::<T>();
    let mut list: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut iter = s.split(pattern).enumerate();
    for (i, t_s) in (&mut iter).take(N) {
        list[i] = match t_s.parse() {
            Ok(t) => MaybeUninit::new(t),
            Err(e) => {
                error!("While splitting \"{s}\" by \"{pattern}\", failed to parse {i}th elem \"{t_s}\" as {ty_name}: {e:?}");
                unreachable!()
            }
        };
    }

    let rem = iter.count();
    if rem != 0 {
        error!(
            str=s,
            pattern=pattern;
                "Trying to parse exactly {N} values of {ty_name}, but found {rem} more!",

        );
    }

    unsafe { std::mem::transmute_copy::<_, [T; N]>(&list) }
}

#[cfg(test)]
mod util_tests {
    use crate::parse_list;

    #[test]
    fn check_parse_list() {
        {
            let a: [i32; 1] = parse_list("10", ",");
            assert_eq!(a, [10]);
        }

        {
            let a: [i32; 3] = parse_list("10-100000-1", "-");
            assert_eq!(a, [10, 100000, 1]);
        }
    }
}
