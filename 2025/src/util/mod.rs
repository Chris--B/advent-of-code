use crate::prelude::*;

use std::str::FromStr;

pub mod cardinal;
pub use cardinal::*;

pub mod parse;
pub use parse::*;

#[track_caller]
pub fn just_str(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

/// Returns the first index of the maximum value
///
/// Iterator::max() returns the LAST maximum value in the iterator, which is sometimes not what's desired.
pub fn first_max<T, I>(iter: I) -> Option<usize>
where
    T: Ord,
    I: IntoIterator<Item = T>,
    I::IntoIter: Clone,
{
    let mut iter = iter.into_iter();
    let max = iter.clone().max()?;
    iter.position(|b| b == max)
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
