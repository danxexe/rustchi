use crate::prelude::*;
use std::fmt;

def_opcode! {
    pub enum CP {
        XHi(u4),
        XLi(u4),
        YHi(u4),
        YLi(u4),
        RI(RQ, u4),
        RQ(RQ, RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::XHi(i) => write!(f, "{NAME} XH {:#03X}", i),
            Self::XLi(i) => write!(f, "{NAME} XL {:#03X}", i),
            Self::YHi(i) => write!(f, "{NAME} YH {:#03X}", i),
            Self::YLi(i) => write!(f, "{NAME} YL {:#03X}", i),
            Self::RI(r, i) => write!(f, "{NAME} {} {}", r, i),
            Self::RQ(r, q) => write!(f, "{NAME} {} {}", r, q),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let (a, b) = match *self {
            Self::XHi(i) => (state.fetch(IdentU4::XH), i),
            Self::XLi(i) => (state.fetch(IdentU4::XL), i),
            Self::YHi(i) => (state.fetch(IdentU4::YH), i),
            Self::YLi(i) => (state.fetch(IdentU4::YL), i),
            Self::RI(r, i) => (state.fetch(r), i),
            Self::RQ(r, q) => (state.fetch(r), state.fetch(q)),
        };

        state
        .set_flag(Flags::C, a < b)
        .set_flag(Flags::Z, a == b);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}
