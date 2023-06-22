#![allow(non_camel_case_types)]

use std::{fmt, ops::Add, ops::BitAnd, ops::BitOr, ops::BitXor, ops::Not, ops::Shl, ops::Shr, ops::Sub};

#[derive(Debug)]
pub struct TryFromIntError;

macro_rules! from_lower_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl From<$source> for $target {
            #[inline]
            fn from(u: $source) -> Self {
                u.0.into()
            }
        }
    )*}
}

macro_rules! try_from_upper_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                if u > Self::MAX.0.into() {
                    Err(TryFromIntError)
                } else {
                    Ok(Self(u.try_into().unwrap()))
                }
            }
        }
    )*}
}

macro_rules! try_from_both_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                let min = Self::MIN.0.into();
                let max = Self::MAX.0.into();
                if u < min || u > max {
                    Err(TryFromIntError)
                } else {
                    Ok(Self(u.try_into().unwrap()))
                }
            }
        }
    )*}
}

macro_rules! bit_or {
    ($target:ty) => {
        impl BitOr for $target {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
    }
}

macro_rules! bit_xor {
    ($target:ty) => {
        impl BitXor for $target {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
    }
}

macro_rules! bit_and {
    ($target:ty) => {
        impl BitAnd for $target {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
    }
}

macro_rules! not {
    ($target:ty) => {
        impl Not for $target {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0 & Self::MAX.0)
            }
        }
    }
}

macro_rules! shl {
    ($target:ty) => {
        impl Shl for $target {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self::Output {
                Self(self.0 << rhs.0) & Self::MAX
            }
        }
    }
}

macro_rules! shr {
    ($target:ty) => {
        impl Shr for $target {
            type Output = Self;

            fn shr(self, rhs: Self) -> Self::Output {
                Self(self.0 >> rhs.0)
            }
        }
    }
}

macro_rules! sub {
    ($target:ty) => {
        impl Sub for $target {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0) & Self::MAX
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct u1(u8);
impl u1 {
    pub const MIN: u1 = Self(0x0);
    pub const MAX: u1 = Self(0x1);
    pub const OFF: u1 = Self::MIN;
    pub const ON: u1 = Self::MAX;
}
impl fmt::Display for u1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:01X}", self.0)
    }
}
impl TryFrom<u4> for u1 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(u: u4) -> Result<Self, Self::Error> {
        if u > u4![1] {
            Err(TryFromIntError)
        } else {
            Ok(Self(u.try_into().unwrap()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct u4(u8);
impl u4 {
    pub const MIN: Self = Self(0x0);
    pub const MAX: Self = Self(0xF);

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn is_set(self, bits: u4) -> bool {
        (self & bits) == bits
    }
}
impl Add for u4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) & 0xF)
    }
}
impl fmt::Display for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:01X}", self.0)
    }
}
impl fmt::UpperHex for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#01X}", self.0)
    }
}

impl fmt::Binary for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04b}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u12(u16);
impl u12 {
    pub const MIN: Self = Self(0x000);
    pub const MAX: Self = Self(0xFFF);

    pub fn low_mid_u8(&self) -> u8 {
        (self.0 & 0xFF).try_into().unwrap()
    }
    pub fn with_hl(self, hl: u8) -> Self {
        self.with_nibble(0, hl.nibble(0)).with_nibble(1, hl.nibble(1))
    }
    pub fn upper_u12(&self) -> Self {
        (self.0 & 0xF00).try_into().unwrap()
    }
    pub fn upper_u4(&self) -> u4 {
        self.nibble(2)
    }
    pub fn mid_u4(&self) -> u4 {
        self.nibble(1)
    }
    pub fn low_u4(&self) -> u4 {
        self.nibble(0)
    }
}
impl From<u8> for u12 {
    fn from(item: u8) -> Self {
        Self(item.into())
    }
}
impl Add for u12 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) & 0xFFF)
    }
}
impl fmt::Display for u12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:03X}", self.0)
    }
}

pub trait GetNibble {
    fn nibble(&self, i: usize) -> u4;
    fn with_nibble(&self, i: usize, data: u4) -> Self;
    fn from_be_nibbles(nibbles: Vec<u4>) -> Self;
}
impl GetNibble for u8 {
    fn nibble(&self, i: usize) -> u4 {
        ((self >> (4 * i)) & 0x0F).try_into().unwrap()
    }

    fn with_nibble(&self, i: usize, data: u4) -> Self {
        let nibble = u8::from(data) << (i * 4);
        let mask = !(0x0Fu8 << (i * 4));
        (self & mask) | nibble
    }

    fn from_be_nibbles(nibbles: Vec<u4>) -> Self {
        0u8.with_nibble(0, nibbles[1]).with_nibble(1, nibbles[0])
    }
}
impl GetNibble for u12 {
    fn nibble(&self, i: usize) -> u4 {
        ((self.0 >> (4 * i)) & 0x0F).try_into().unwrap()
    }

    fn with_nibble(&self, i: usize, data: u4) -> Self {
        let nibble = u16::from(data) << (i * 4);
        let mask = !(0x0Fu16 << (i * 4));
        ((self.0 & mask) | nibble).try_into().unwrap()
    }

    fn from_be_nibbles(nibbles: Vec<u4>) -> Self {
        u12![0].with_nibble(0, nibbles[2]).with_nibble(1, nibbles[1]).with_nibble(2, nibbles[0])
    }
}

not!(u4);

bit_or!(u4);
bit_or!(u12);

bit_xor!(u4);
bit_xor!(u12);

bit_and!(u4);
bit_and!(u12);

shl!(u4);
shl!(u12);

shr!(u4);
shr!(u12);

sub!(u4);
sub!(u12);

from_lower_bounded!(u1, u8, usize);
from_lower_bounded!(u4, u8, u12, u16, usize);
from_lower_bounded!(u12, u16, usize);
try_from_upper_bounded!(u8, u1, u4);
try_from_upper_bounded!(u16, u1, u4, u12);
try_from_upper_bounded!(usize, u4, u12);
try_from_both_bounded!(i32, u4, u12);

#[cfg(test)]
mod test {
    #[test]
    fn nibbles() {
        assert_eq!(u12![0x100], u12![0x123].upper_u12());
        assert_eq!(0x23u8, u12![0x123].low_mid_u8());
        assert_eq!(u4![0x1], u12![0x123].upper_u4());
        assert_eq!(u4![0x2], u12![0x123].mid_u4());
        assert_eq!(u4![0x3], u12![0x123].low_u4());
    }
}
