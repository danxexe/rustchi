use crate::{
    flags::Flags,
    primitive::u4,
    opcode::rq::*,
    opcode::exec::Exec,
    state::State,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum FAN {
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for FAN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FAN::RI(r, i) => write!(f, "FAN {} {:#03X}", r, i),
            FAN::RQ(r, q) => write!(f, "FAN {} {}", r, q),
        }
    }
}

impl Exec for FAN {
    fn exec(&self, state: &mut State) {
        let (a, b) = match *self {
            FAN::RI(r, i) => (state.fetch_u4(r.into()), i),
            FAN::RQ(r, q) => (state.fetch_u4(r.into()), state.fetch_u4(q.into())),
        };

        let value = a & b;

        state.set_flag(Flags::Z, value == u4![0]);
    }
}
