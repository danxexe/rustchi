use crate::{
    opcode::*, state::State, flags::Flags,
};

use std::fmt;

def_opcode! {
    pub struct RRC(pub RQ);
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{NAME} {}", self.0)
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let ident = IdentU4::from(self.0);
        let value = state.fetch_u4(ident);
        let c = (state.flags & Flags::C).bits();

        let value = (value >> u4![1]) | (u4![c] << u4![3]);
        let carry = (value & u4![0x1]) != u4![0];

        state
        .set_u4(ident, value)
        .set_flag(Flags::C, carry)
        .set_flag(Flags::Z, value == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
