#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Clone, Copy)]
pub struct u1(u8);
impl From<u8> for u1 {
    fn from(item: u8) -> Self {
        Self(item)
    }
}
impl From<u16> for u1 {
    fn from(item: u16) -> Self {
        let val: u8 = item.try_into().unwrap();
        Self(val & 0x1)
    }
}
impl From<u1> for usize {
    fn from(item: u1) -> Self {
        item.0.into()
    }
}
impl fmt::Display for u1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:01X}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct u4(u8);
impl From<u8> for u4 {
    fn from(item: u8) -> Self {
        Self(item)
    }
}
impl From<u16> for u4 {
    fn from(item: u16) -> Self {
        let val: u8 = item.try_into().unwrap();
        assert!(val <= 0xF);
        Self(val)
    }
}
impl From<u4> for u8 {
    fn from(item: u4) -> Self {
        item.0
    }
}
impl From<u4> for u16 {
    fn from(item: u4) -> Self {
        item.0.into()
    }
}
impl From<u4> for usize {
    fn from(item: u4) -> Self {
        item.0.into()
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

#[derive(Clone, Copy)]
pub struct u12(u16);
impl From<u8> for u12 {
    fn from(item: u8) -> Self {
        Self(item.into())
    }
}
impl From<u16> for u12 {
    fn from(item: u16) -> Self {
        Self(item)
    }
}
impl From<u12> for usize {
    fn from(item: u12) -> Self {
        item.0.into()
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
        ((self >> (4 * i)) & 0x0F).into()
    }

    fn with_nibble(&self, i: usize, data: u4) -> Self {
        let nibble = u8::from(data) << (i * 4);
        let mask = !(0x0Fu8 << (i * 4));
        (self & mask) | nibble
    }
}
impl GetNibble for u12 {
    fn nibble(&self, i: usize) -> u4 {
        ((self.0 >> (4 * i)) & 0x0F).into()
    }

    fn with_nibble(&self, i: usize, data: u4) -> Self {
        let nibble = u16::from(data) << (i * 4);
        let mask = !(0x0Fu16 << (i * 4));
        ((self.0 & mask) | nibble).into()
    }
}
