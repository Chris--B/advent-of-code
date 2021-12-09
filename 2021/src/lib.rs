#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]

use aoc_runner_derive::aoc_lib;

mod day01;
mod day02;
#[allow(unused_variables)]
mod day03;
mod day04;
#[allow(unused_variables)]
mod day05;
#[allow(unused_variables)]
mod day06;
mod day07;
mod day08;
mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;

pub mod framebuffer;

aoc_lib! { year = 2021 }

#[inline(always)]
pub(crate) fn find_exactly_one<T>(mut iter: impl Iterator<Item = T>) -> T {
    let t: T = iter.next().unwrap();

    debug_assert!(iter.next().is_none());

    t
}
