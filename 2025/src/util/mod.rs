use crate::prelude::*;

use std::cmp::{Ord, Reverse};
use std::str::FromStr;

pub mod cardinal;
pub use cardinal::*;

pub mod parse;
pub use parse::*;

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
