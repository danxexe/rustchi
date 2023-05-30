use std::fmt;

use crate::registers::Reg;

// Five-bit immediate data or label 0x00–0x1F
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
pub struct S(u8);

impl From<u16> for S {
    fn from(item: u16) -> S {
        S(item.try_into().unwrap())
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

// Four-bit immediate data 0x0–0x0F
pub struct I(u8);

impl I {
    pub fn u8(&self) -> u8 {
        self.0
    }
}

impl From<u16> for I {
    fn from(item: u16) -> I {
        I(item.try_into().unwrap())
    }
}

impl From<I> for u8 {
    fn from(item: I) -> u8 {
        item.0
    }
}

impl fmt::Display for I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:01X}", self.0)
    }
}

// Instruction data source
pub enum Source {
    I(I),
    L(L),
    Reg(Reg),
}

impl From<I> for Source {
    fn from(item: I) -> Source {
        Source::I(item)
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
            Self::I(i) => write!(f, "{}", i),
            Self::L(l) => write!(f, "{}", l),
            Self::Reg(reg) => write!(f, "{}", reg),
        }
    }
}
