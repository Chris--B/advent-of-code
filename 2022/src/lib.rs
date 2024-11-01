#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]
#![allow(clippy::comparison_chain, clippy::missing_transmute_annotations)]

use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
#[cfg(feature = "broken")]
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub mod framebuffer;
pub mod vec;

aoc_lib! { year = 2022 }

// Run this function when the binary is loaded. This typically happens BEFORE MAIN.
// This is a BAD IDEA, but cargo-aoc doesn't give us hooks anywhere else. So it's this or lazy-init in EVERY solution 😬.
// #[ctor::ctor]
pub fn init_logging() {
    use env_logger::{Builder, Env};
    use prelude::*;

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

    trace!("Hello");
    debug!("Hello");
    info!("Hello");
    warn!("Hello");
    error!("Hello");
}

pub mod prelude {
    pub use crate::framebuffer::Framebuffer;
    pub use crate::init_logging;
    pub use crate::vec::VecExt;

    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub const EZ_TIMEOUT: Duration = Duration::from_millis(1_000);
    pub const LONG_TIMEOUT: Duration = Duration::from_millis(5_000);

    pub use derive_more::*;
    pub use either::*;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, log_enabled, trace, warn, Level::*};
    pub use num::Complex;
    pub use scan_fmt::scan_fmt;
    pub use smallstr::SmallString;
    pub use smallvec::{smallvec, SmallVec};
    pub use ultraviolet::{IVec2, IVec3, Vec3};

    pub use std::collections::{HashMap, HashSet, VecDeque};
    pub use std::num::Wrapping;
    use std::time::Duration;

    pub fn force_dword_align_str(s: &str) -> &'static str {
        let l = s.as_bytes().len();

        let mut v = vec![0_u32; 4 * l];
        unsafe {
            std::ptr::copy(s.as_bytes().as_ptr(), v.as_mut_ptr() as *mut u8, l);
        }

        let boxed: &'static [u32] = Box::leak(v.into_boxed_slice());
        let aligned_s: &str = unsafe {
            let bytes = std::slice::from_raw_parts(boxed.as_ptr() as *const u8, l);
            std::str::from_utf8(bytes).unwrap()
        };

        assert_eq!(s, aligned_s);

        aligned_s
    }

    pub fn sign(x: i32) -> i32 {
        use std::cmp::Ordering::*;

        match x.cmp(&0) {
            Less => -1,
            Equal => 0,
            Greater => 1,
        }
    }

    #[inline(always)]
    pub fn find_exactly_one<T, I, P>(iter: I, p: P) -> T
    where
        I: IntoIterator<Item = T>,
        P: FnMut(&I::Item) -> bool,
    {
        let mut iter = iter.into_iter().filter(p);
        let t: T = iter
            .next()
            .expect("Expected to find one item in iterator, but found none");
        debug_assert!(
            iter.next().is_none(),
            "Expected to find one item in iterator, but found more than 1"
        );

        t
    }

    pub fn iter_to_array<T: Copy, const N: usize>(mut iter: impl Iterator<Item = T>) -> [T; N] {
        use core::mem::transmute;
        use core::mem::MaybeUninit;

        // We need Copy for this to work
        let mut arr = [MaybeUninit::uninit(); N];

        for elem in &mut arr {
            elem.write(iter.next().unwrap());
        }

        // This is just a bug
        // https://github.com/rust-lang/rust/issues/61956
        unsafe {
            let p_arr: *const [MaybeUninit<T>; N] = &arr as *const [std::mem::MaybeUninit<T>; N];
            let p_res: *const [T; N] = transmute(p_arr);
            p_res.read()
        }
    }

    pub fn iter_to_array_or<T: Copy, const N: usize>(
        mut iter: impl Iterator<Item = T>,
        default: T,
    ) -> [T; N] {
        use core::mem::transmute;
        use core::mem::MaybeUninit;

        // We need Copy for this to work
        let mut arr = [MaybeUninit::uninit(); N];

        for elem in &mut arr {
            elem.write(iter.next().unwrap_or(default));
        }

        // This is just a bug
        // https://github.com/rust-lang/rust/issues/61956
        unsafe {
            let p_arr: *const [MaybeUninit<T>; N] = &arr as *const [std::mem::MaybeUninit<T>; N];
            let p_res: *const [T; N] = transmute(p_arr);
            p_res.read()
        }
    }

    pub fn iter_to_array_or_default<T: Copy + Default, const N: usize>(
        iter: impl Iterator<Item = T>,
    ) -> [T; N] {
        iter_to_array_or(iter, T::default())
    }

    pub fn fast_parse_i32(input: &[u8]) -> i32 {
        if input[0] == b'-' {
            -(fast_parse_u32(&input[1..]) as i32)
        } else {
            fast_parse_u32(input) as i32
        }
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

    pub fn fast_parse_u8(input: &[u8]) -> u32 {
        let mut digits = [0_u32; 2];
        let mut x = 1;
        for (i, b) in input.iter().rev().enumerate() {
            digits[i] = x * (*b - b'0') as u32;
            x *= 10;
        }

        digits.into_iter().sum()
    }
}
