use crate::prelude::*;
use std::fmt;

def_opcode! {
    pub enum LDPX {
        MX(u4),
        RQ(RQ, RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MX(i) => write!(f, "{NAME} MX {}", i),
            Self::RQ(r, q) => write!(f, "{NAME} {} {}", r, q),
        }
    }
}

impl T {
    pub fn dest(&self) -> IdentU4 {
        match self {
            Self::MX(_) => IdentU4::MX,
            Self::RQ(r, _) => IdentU4::from(*r),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let data = match *self {
            Self::MX(i) => i,
            Self::RQ(_, q) => state.fetch_u4(IdentU4::from(q)),
        };
        let inc = state.fetch_u12(IdentU12::X) + u12![1];

        state
        .set_u4(self.dest(), data)
        .set_u12(IdentU12::X, inc);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
