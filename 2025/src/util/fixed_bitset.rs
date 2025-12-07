use num::traits::*;

use std::ops;

pub type Bitset128 = FixedBitset<u128>;
pub type Bitset64 = FixedBitset<u64>;
pub type Bitset32 = FixedBitset<u32>;
pub type Bitset16 = FixedBitset<u16>;
pub type Bitset8 = FixedBitset<u8>;

// No longer built-in types
pub type Bitset256 = FixedBitset<U256>;

/// (╯°□°)╯︵ ┻━┻
pub trait Backing:
    num::PrimInt
    + ops::BitOrAssign
    + ops::BitAndAssign
    + ConstOne
    + ConstZero
    + Default
    + core::fmt::Display
{
}

impl Backing for u8 {}
impl Backing for u16 {}
impl Backing for u32 {}
impl Backing for u64 {}
impl Backing for u128 {}

#[derive(Copy, Clone, Default)]
pub struct FixedBitset<N> {
    bits: N,
}

impl<N: Backing> FixedBitset<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner(&self) -> N {
        self.bits
    }

    #[track_caller]
    pub fn contains<Elem>(&self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        (self.bits & (N::ONE << idx)) != N::ZERO
    }

    #[track_caller]
    pub fn insert<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        let old = (self.bits & (N::ONE << idx)) != N::ZERO;
        self.bits |= N::ONE << idx;
        old
    }

    #[track_caller]
    pub fn remove<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        let old = (self.bits & (N::ONE << idx)) != N::ZERO;
        self.bits &= !(N::ONE << idx);
        old
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        (0..Self::bit_width()).filter(|&i| self.contains(i))
    }

    fn bit_width() -> u32 {
        N::ZERO.count_zeros()
    }

    #[track_caller]
    fn idx_of<T: num::PrimInt + core::fmt::Display>(t: T) -> usize {
        if let Some(i) = t.to_u32() {
            if i < Self::bit_width() {
                i as usize
            } else {
                panic!(
                    "Failed to convert \"{t}\" to an 8-bit int because it's out of bounds (valid is 0..<{bits}).",
                    bits = Self::bit_width()
                );
            }
        } else {
            panic!("Failed to convert \"{t}\" to an 8-bit int");
        }
    }
}

use std::ops::{BitAnd, BitOr, Not, Shl};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct U256(pub [u128; 2]);

impl U256 {
    pub const ONE: Self = Self([0, 1]);
    pub const ZERO: Self = Self([0, 0]);

    pub fn count_ones(self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones()
    }

    pub fn bit_width() -> u32 {
        256
    }
}

impl Shl<usize> for U256 {
    type Output = Self;

    fn shl(self, n: usize) -> Self::Output {
        use std::num::Wrapping;

        let Self([a, b]) = self;
        let mut a = Wrapping(a);
        let mut b = Wrapping(b);

        a <<= n;
        a |= b >> (256 - n);

        b <<= n;

        Self([a.0, b.0])
    }
}

impl BitAnd for U256 {
    type Output = Self;

    fn bitand(self, U256(rhs): U256) -> Self::Output {
        let U256(this) = self;
        U256([this[0] & rhs[0], this[1] & rhs[1]])
    }
}

impl BitOr for U256 {
    type Output = Self;

    fn bitor(self, U256(rhs): U256) -> Self::Output {
        let U256(this) = self;
        U256([this[0] | rhs[0], this[1] | rhs[1]])
    }
}

impl Not for U256 {
    type Output = Self;

    fn not(self) -> Self {
        let U256(this) = self;
        U256([!this[0], !this[1]])
    }
}

impl FixedBitset<U256> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner(&self) -> U256 {
        self.bits
    }

    #[track_caller]
    pub fn contains<Elem>(&self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        (self.bits & (U256::ONE << idx)) != U256::ZERO
    }

    #[track_caller]
    pub fn insert<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        let old = (self.bits & (U256::ONE << idx)) != U256::ZERO;
        self.bits = self.bits | (U256::ONE << idx);
        old
    }

    #[track_caller]
    pub fn remove<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: num::PrimInt + core::fmt::Display,
    {
        let idx: usize = Self::idx_of(item);

        let old = (self.bits & (U256::ONE << idx)) != U256::ZERO;
        self.bits = self.bits & !(U256::ONE << idx);
        old
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        (0..Self::bit_width()).filter(|&i| self.contains(i))
    }

    fn bit_width() -> u32 {
        U256::bit_width()
    }

    #[track_caller]
    fn idx_of<T: num::PrimInt + core::fmt::Display>(t: T) -> usize {
        if let Some(i) = t.to_u32() {
            if i < Self::bit_width() {
                i as usize
            } else {
                panic!(
                    "Failed to convert \"{t}\" to an 8-bit int because it's out of bounds (valid is 0..<{bits}).",
                    bits = Self::bit_width()
                );
            }
        } else {
            panic!("Failed to convert \"{t}\" to an 8-bit int");
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FixedBitsetIter {
    // All bitset sizes route to this because IDGAF
    bits: u128,
    i: u32,
}

impl Iterator for FixedBitsetIter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let shifted = self.bits >> self.i;
        if shifted.count_ones() == 0 {
            return None;
        }

        self.i += shifted.trailing_zeros();
        let res = self.i;
        assert_eq!((self.bits >> self.i) & 1, 1);
        self.i += 1;

        Some(res)
    }
}

impl<N: Backing> IntoIterator for FixedBitset<N> {
    type Item = u32;
    type IntoIter = FixedBitsetIter;

    fn into_iter(self) -> Self::IntoIter {
        // (╯°□°)╯︵ ┻━┻
        let bits = N::from(self.bits).unwrap().to_u128().unwrap();
        FixedBitsetIter { bits, i: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[allow(clippy::bool_assert_comparison)]
    #[generic_tests::define]
    mod bitset_generic_tests {

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn check_empty<N: Backing>() {
            let bitset = FixedBitset::<N>::new();

            assert_eq!(bitset.len(), 0);
            assert!(bitset.is_empty());
            for i in 0..FixedBitset::<N>::bit_width() {
                assert!(!bitset.contains(i));
            }
        }

        #[test]
        fn check_basic_usage<N: Backing>() {
            let mut bitset = FixedBitset::<N>::new();
            assert!(bitset.is_empty());

            // Insert first 1
            assert_eq!(bitset.insert(1), false);
            assert_eq!(bitset.len(), 1);
            assert!(bitset.contains(1));

            // Insert second 1 (changes nothing)
            assert_eq!(bitset.insert(1), true);
            assert_eq!(bitset.len(), 1);
            assert!(bitset.contains(1));

            // Insert a different number
            assert_eq!(bitset.insert(FixedBitset::<N>::bit_width() - 1), false);
            assert_eq!(bitset.len(), 2);
            assert!(bitset.contains(FixedBitset::<N>::bit_width() - 1));
        }

        #[test]
        fn check_all_inserts<N: Backing>() {
            let mut bitset = FixedBitset::<N>::new();
            assert!(bitset.is_empty());

            for i in 0..FixedBitset::<N>::bit_width() {
                let i = i as usize;
                assert!(!bitset.contains(i), "bitset already contains {i} somehow");
                assert_eq!(
                    bitset.insert(i),
                    false,
                    "inserting {i} and it already contains it somehow"
                );
                assert_eq!(bitset.len(), 1 + i, "bitset has the wrong length");
            }
        }

        #[test]
        #[should_panic(expected = "out of bounds")]
        fn check_bad_small_index<N: Backing>() {
            let mut bitset = FixedBitset::<N>::new();
            bitset.insert(FixedBitset::<N>::bit_width());
        }

        #[test]
        #[should_panic(expected = "to convert")]
        fn check_bad_big_index<N: Backing>() {
            let mut bitset = FixedBitset::<N>::new();
            bitset.insert(10_000_u16);
        }

        #[test]
        fn check_forward_iter<N: Backing>() {
            let mut bitset = FixedBitset::<N>::new();

            let expected = (0..FixedBitset::<N>::bit_width()).step_by(2).collect_vec();

            for &i in &expected {
                bitset.insert(i);
            }
            let actual = bitset.into_iter().take(1_000).collect_vec();

            assert_eq!(expected, actual);
        }

        // #[test]
        // fn check_backard_iter<N: Backing>() {
        //     let mut bitset = FixedBitset::<N>::new();

        //     let expected = (0..FixedBitset::<N>::bit_width())
        //         .step_by(2)
        //         .rev()
        //         .collect_vec();

        //     for &i in &expected {
        //         bitset.insert(i);
        //     }
        //     let actual = bitset.into_iter().rev().take(1_000).collect_vec();

        //     assert_eq!(expected, actual);
        // }

        #[instantiate_tests(<u8>)]
        mod bitset8 {}

        #[instantiate_tests(<u16>)]
        mod bitset16 {}

        #[instantiate_tests(<u32>)]
        mod bitset32 {}

        #[instantiate_tests(<u64>)]
        mod bitset64 {}

        #[instantiate_tests(<u128>)]
        mod bitset128 {}
    }
}
