use core::fmt;
use core::ops;

use crate::util::fixed_bitset::Backing;

#[derive(Copy, Clone, PartialEq, Eq, bytemuck::NoUninit)]
#[repr(transparent)]
pub struct U256(pub [u128; 2]);

impl Default for U256 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("U256")
            .field("hi", &format!("0b_{:0b}", self.0[0]))
            .field("lo", &format!("0b_{:0b}", self.0[1]))
            .finish()
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("U256")
            .field("hi", &format!("0b_{:0b}", self.0[0]))
            .field("lo", &format!("0b_{:0b}", self.0[1]))
            .finish()
    }
}

impl ops::Shl<usize> for U256 {
    type Output = Self;

    fn shl(self, mut n: usize) -> Self::Output {
        let Self([mut hi, mut lo]) = self;

        if n >= 128 {
            // shift by 128
            hi = lo;
            lo = 0;
            n -= 128;
        }

        if n > 0 {
            hi <<= n;
            hi |= lo >> (128 - n);
            lo <<= n;
        }

        Self([hi, lo])
    }
}

impl ops::BitAnd for U256 {
    type Output = Self;

    fn bitand(self, U256(rhs): U256) -> Self::Output {
        let U256(this) = self;
        U256([this[0] & rhs[0], this[1] & rhs[1]])
    }
}

impl ops::BitOr for U256 {
    type Output = Self;

    fn bitor(self, U256(rhs): U256) -> Self::Output {
        let U256(this) = self;
        U256([this[0] | rhs[0], this[1] | rhs[1]])
    }
}

impl ops::Not for U256 {
    type Output = Self;

    fn not(self) -> Self {
        let U256(this) = self;
        U256([!this[0], !this[1]])
    }
}

impl Backing for U256 {
    const ONE: Self = Self([0, 1]);
    const ZERO: Self = Self([0, 0]);

    fn count_ones(&self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones()
    }

    fn count_zeros(&self) -> u32 {
        self.0[0].count_zeros() + self.0[1].count_zeros()
    }

    fn bit_width() -> u32 {
        256
    }

    fn as_be_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_u256_shifts() {
        for i in 0..256 {
            dbg!(i);
            let bits = U256::ONE << i;
            dbg!(bits);

            assert_eq!(bits.count_ones(), 1);
            assert_eq!(bits.count_zeros(), 255);

            let U256([hi, lo]) = bits;
            if i > 127 {
                assert_eq!(hi.count_ones(), 1);
                assert_eq!(hi.count_zeros(), 127);
                assert_eq!(lo.count_ones(), 0);
                assert_eq!(lo.count_zeros(), 128);
            } else {
                assert_eq!(hi.count_ones(), 0);
                assert_eq!(hi.count_zeros(), 128);
                assert_eq!(lo.count_ones(), 1);
                assert_eq!(lo.count_zeros(), 127);
            }
            dbg!();
        }
    }
}
