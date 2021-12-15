#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]

use aoc_runner_derive::aoc_lib;

use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

pub mod day01;
pub mod day02;
#[allow(unused_variables)]
pub mod day03;
pub mod day04;
#[allow(unused_variables)]
pub mod day05;
#[allow(unused_variables)]
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;

pub mod framebuffer;

/// Configure whether images are written to disk or not
static SAVE_IMG: AtomicBool = AtomicBool::new(false);

fn saving_images() -> bool {
    SAVE_IMG.load(SeqCst)
}

aoc_lib! { year = 2021 }

#[inline(always)]
pub(crate) fn find_exactly_one<T>(mut iter: impl Iterator<Item = T>) -> T {
    let t: T = iter.next().unwrap();

    debug_assert!(iter.next().is_none());

    t
}
