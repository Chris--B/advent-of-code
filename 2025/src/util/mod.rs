use crate::prelude::*;

use std::cmp::{Ord, Reverse};
use std::str::FromStr;

pub mod cardinal;
pub use cardinal::*;

pub mod fixed_bitset;
pub use fixed_bitset::*;

pub mod framebuffer;
pub use framebuffer::*;

pub mod parse;
pub use parse::*;

pub mod vec_n_ext;
pub use vec_n_ext::*;

#[track_caller]
pub fn just_str(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

pub trait AocIteratorExt: Iterator + DoubleEndedIterator {
    fn first_position_max(self) -> Option<usize>
    where
        Self::Item: Ord;

    fn first_position_min(self) -> Option<usize>
    where
        Self::Item: Ord;
}

impl<T> AocIteratorExt for T
where
    T: DoubleEndedIterator,
{
    fn first_position_max(self) -> Option<usize>
    where
        Self::Item: Ord,
    {
        self.map(Reverse).position_min_by(Ord::cmp)
    }

    fn first_position_min(self) -> Option<usize>
    where
        Self::Item: Ord,
    {
        self.map(Reverse).position_max_by(Ord::cmp)
    }
}

pub trait Tally<T>
where
    T: Eq + std::hash::Hash,
{
    fn tally(self) -> HashMap<T, usize>;
}

impl<I, T> Tally<T> for I
where
    I: Iterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    fn tally(self) -> HashMap<T, usize> {
        let mut tally = HashMap::new();
        for elem in self {
            *tally.entry(elem).or_insert(0) += 1;
        }
        tally
    }
}

#[track_caller]
pub fn parse_or_fail<T: FromStr>(s: impl AsRef<str>) -> T {
    let s: &str = s.as_ref();
    match s.parse() {
        Ok(t) => t,
        Err(_err) => panic!(
            "Failed to parse \"{s}\" as a {}",
            std::any::type_name::<T>()
        ),
    }
}

#[track_caller]
pub fn merge_ranges(mut rs: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    debug_assert!(rs.is_sorted(), "Ranges must be sorted to be merged");

    let mut i = 0;
    while i + 1 < rs.len() {
        if rs[i].1 < rs[i + 1].0 {
            // no overlap, cannot merge, continue
            i += 1;
        } else {
            // we have some overlap, let's try and merge
            rs[i] = (rs[i].0, i64::max(rs[i].1, rs[i + 1].1));
            rs.remove(i + 1);
        }
    }

    rs
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[rstest]
    #[case::simple([(3, 5), (10, 20)], [(3, 5), (10, 14), (16, 20), (12, 18) ])]
    #[case::adjacent([(10, 30)], [(10, 20), (20, 30)])]
    #[case::superset([(1, 100)], [(1, 100), (10, 20), (30, 40)])]
    #[timeout(Duration::from_millis(1))]
    fn check_merge_ranges(
        #[case] expected: impl IntoIterator<Item = (i64, i64)>,
        #[case] ranges: impl IntoIterator<Item = (i64, i64)>,
    ) {
        let expected = expected.into_iter().collect_vec();
        let mut ranges = ranges.into_iter().collect_vec();
        ranges.sort();

        assert_eq!(expected, merge_ranges(ranges));
    }
}
