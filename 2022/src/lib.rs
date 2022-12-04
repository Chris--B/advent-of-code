#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(stdsimd))]

use aoc_runner_derive::aoc_lib;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub mod framebuffer;

aoc_lib! { year = 2022 }

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn find_exactly_one<T, I, P>(iter: I, p: P) -> T
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
