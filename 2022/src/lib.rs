#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]

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

pub mod framebuffer;

aoc_lib! { year = 2022 }

mod prelude {
    pub use crate::framebuffer::Framebuffer;

    pub use aoc_runner_derive::{aoc, aoc_generator};

    pub use itertools::Itertools;
    pub use smallstr::SmallString;
    pub use smallvec::{smallvec, SmallVec};
    pub use ultraviolet::IVec2;

    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::num::Wrapping;

    pub fn sign(x: i32) -> i32 {
        use std::cmp::Ordering::*;

        match x.cmp(&0) {
            Less => -1,
            Equal => 0,
            Greater => 1,
        }
    }

    #[allow(dead_code)]
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

    pub fn fast_parse_u32(input: &[u8]) -> u32 {
        let mut digits = [0_u32; 10];
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
