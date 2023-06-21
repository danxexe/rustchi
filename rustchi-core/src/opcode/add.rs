use crate::prelude::*;
use std::fmt;

def_opcode! {
    pub enum ADD {
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
        let (r, a, b) = match *self {
            ADD::RI(r, i) => (r, state.fetch(r), i),
            ADD::RQ(r, q) => (r, state.fetch(r), state.fetch(q)),
        };

        let sum = u8![a] + u8![b];
        let (sum, carry) = if state.flags.contains(Flags::D) {
            // assuming BCD digits <= 9
            let carry = sum >= 10;
            (if carry {u4![sum - 10]} else {u4![sum]}, carry)
        } else {
            (u4![sum & 0xF], sum > 0xF)
        };

        state
        .set(r, u4![sum])
        .set_flag(Flags::C, carry)
        .set_flag(Flags::Z, sum == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}
