use std::fmt;

use crate::primitive::*;
use crate::registers::Reg;

// Five-bit immediate data or label 0x00–0x1F
#[derive(Clone, Copy)]
pub struct P(u8);

impl From<u16> for P {
    fn from(item: u16) -> P {
        P(item.try_into().unwrap())
    }
}

impl From<P> for u8 {
    fn from(item: P) -> u8 {
        item.0
    }
}

impl fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

// Eight-bit immediate data or label 0x00–0xFF
#[derive(Clone, Copy)]
pub struct S(u8);

impl From<u16> for S {
    fn from(item: u16) -> S {
        S(item.try_into().unwrap())
    }
}

impl From<S> for u8 {
    fn from(item: S) -> u8 {
        item.0
    }
}

impl From<S> for u16 {
    fn from(item: S) -> u16 {
        item.0.try_into().unwrap()
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

// Eight-bit immediate data 0x00–0xFF
#[derive(Clone, Copy)]
pub struct L(u8);

impl L {
    pub fn u8(&self) -> u8 {
        self.0
    }
}

impl From<u16> for L {
    fn from(item: u16) -> L {
        L(item.try_into().unwrap())
    }
}

impl fmt::Display for L {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

// Instruction data source
#[derive(Clone, Copy)]
pub enum Source {
    U4(u4),
    L(L),
    Reg(Reg),
}

impl From<u4> for Source {
    fn from(item: u4) -> Source {
        Source::U4(item)
    }
}

impl From<L> for Source {
    fn from(item: L) -> Source {
        Source::L(item)
    }
}

impl From<Reg> for Source {
    fn from(item: Reg) -> Source {
        Source::Reg(item)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U4(i) => write!(f, "{}", i),
            Self::L(l) => write!(f, "{}", l),
            Self::Reg(reg) => write!(f, "{}", reg),
        }
    }
}
