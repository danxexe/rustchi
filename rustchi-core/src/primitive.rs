#![allow(non_camel_case_types)]

use std::{fmt, ops::Add, ops::BitAnd, ops::BitOr, ops::Shl};

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

#[derive(Debug, Clone, Copy)]
pub struct u4(u8);
impl u4 {
    pub const MIN: Self = Self(0x0);
    pub const MAX: Self = Self(0xF);
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

#[derive(Clone, Copy, PartialEq)]
pub struct u12(u16);
impl u12 {
    pub const MIN: Self = Self(0x000);
    pub const MAX: Self = Self(0xFFF);
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
impl BitAnd for u12 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitOr for u12 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
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

from_lower_bounded!(u1, usize);
from_lower_bounded!(u4, u8, u12, u16, usize);
from_lower_bounded!(u12, u16, usize);
try_from_upper_bounded!(u8, u1, u4);
try_from_upper_bounded!(u16, u1, u4, u12);
try_from_upper_bounded!(usize, u12);
