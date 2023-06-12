#![allow(non_camel_case_types)]

use std::{fmt, ops::Add, ops::BitAnd, ops::BitOr, ops::Not, ops::Shl};

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

#[derive(Debug, Clone, Copy)]
pub struct u1(u8);
impl u1 {
    pub const MIN: u1 = Self(0x0);
    pub const MAX: u1 = Self(0x1);
}
impl fmt::Display for u1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:01X}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct u4(u8);
impl u4 {
    pub const MIN: Self = Self(0x0);
    pub const MAX: Self = Self(0xF);

    pub fn is_set(&self, bits: u4) -> bool {
        (*self & bits) == bits
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u12(u16);
impl u12 {
    pub const MIN: Self = Self(0x000);
    pub const MAX: Self = Self(0xFFF);

    pub fn low_mid_u8(&self) -> u8 {
        (self.0 & 0xFF).try_into().unwrap()
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
impl Shl for u12 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
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
}

not!(u4);

bit_or!(u4);
bit_or!(u12);

bit_and!(u4);
bit_and!(u12);

from_lower_bounded!(u1, usize);
from_lower_bounded!(u4, u8, u12, u16, usize);
from_lower_bounded!(u12, u16, usize);
try_from_upper_bounded!(u8, u1, u4);
try_from_upper_bounded!(u16, u1, u4, u12);
try_from_upper_bounded!(usize, u12);
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
