use std::fmt::Display;

/// A set with < 128 items, indexed with 0..128
#[derive(Copy, Clone, Default)]
pub struct Bitset128 {
    bits: u128,
}

impl Bitset128 {
    pub fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    pub fn contains<Elem>(&self, item: Elem) -> bool
    where
        Elem: TryInto<u8> + Display + Copy,
    {
        let Ok(idx) = item.try_into() else {
            panic!("Unable to convert {item} into 8-bit index");
        };
        assert!(
            idx < 128,
            "Index is out of bounds. Must be < 128, but idx={idx}"
        );

        (self.bits & (1 << idx)) != 0
    }

    #[track_caller]
    pub fn insert<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: TryInto<u8> + Display + Copy,
    {
        let Ok(idx) = item.try_into() else {
            panic!("Unable to convert {item} into 8-bit index");
        };
        assert!(
            idx < 128,
            "Index is out of bounds. Must be < 128, but idx={idx}"
        );

        let old = (self.bits & (1 << idx)) != 0;
        self.bits |= 1 << idx;
        old
    }

    #[track_caller]
    pub fn remove<Elem>(&mut self, item: Elem) -> bool
    where
        Elem: TryInto<u8> + Display + Copy,
    {
        let Ok(idx) = item.try_into() else {
            panic!("Unable to convert {item} into 8-bit index");
        };
        assert!(
            idx < 128,
            "Index is out of bounds. Must be < 128, but idx={idx}"
        );

        let old = (self.bits & (1 << idx)) != 0;
        self.bits &= !(1 << idx);
        old
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod test {
    use super::Bitset128;

    #[test]
    fn check_empty() {
        let bitset = Bitset128::new();

        assert_eq!(bitset.len(), 0);
        assert!(bitset.is_empty());
        for i in 0..128 {
            assert!(!bitset.contains(i));
        }
    }

    #[test]
    fn check_basic_usage() {
        let mut bitset = Bitset128::new();
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
        assert_eq!(bitset.insert(127), false);
        assert_eq!(bitset.len(), 2);
        assert!(bitset.contains(127));
    }

    #[test]
    fn check_all_inserts() {
        let mut bitset = Bitset128::new();
        assert!(bitset.is_empty());

        for i in 0_usize..128 {
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
    fn check_bad_small_index() {
        let mut bitset = Bitset128::new();
        bitset.insert(128_u16);
    }

    #[test]
    #[should_panic(expected = "to convert")]
    fn check_bad_big_index() {
        let mut bitset = Bitset128::new();
        bitset.insert(10_000_u16);
    }
}
