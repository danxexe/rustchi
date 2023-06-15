use crate::{
    flags::Flags,
    primitive::u4,
    opcode::rq::*,
    opcode::exec::*,
    state::State,
};

use std::fmt;

def_opcode! {
    pub enum FAN {
        RI(RQ, u4),
        RQ(RQ, RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RI(r, i) => write!(f, "{NAME} {} {:#03X}", r, i),
            Self::RQ(r, q) => write!(f, "{NAME} {} {}", r, q),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let (a, b) = match *self {
            Self::RI(r, i) => (state.fetch_u4(r.into()), i),
            Self::RQ(r, q) => (state.fetch_u4(r.into()), state.fetch_u4(q.into())),
        };

        let value = a & b;

        state.set_flag(Flags::Z, value == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}
