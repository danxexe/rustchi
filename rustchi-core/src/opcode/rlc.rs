use crate::prelude::*;
use std::fmt;

def_opcode! {
    pub struct RLC(pub RQ);
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{NAME} {}", self.0)
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let ident = IdentU4::from(self.0);
        let value = state.fetch(ident);
        let c = (state.flags & Flags::C).bits();

        let value = (value << u4![1]) | u4![c];
        let carry = (value & u4![0x8]) != u4![0];

        state
        .set(ident, value)
        .set_flag(Flags::C, carry)
        .set_flag(Flags::Z, value == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}
