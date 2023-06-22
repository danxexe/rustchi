use crate::prelude::*;
use std::fmt;


#[derive(Debug, Clone, Copy)]
pub struct DEC_Mn(pub u4);

impl Op for DEC_Mn {}

impl fmt::Display for DEC_Mn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC M({})", self.0)
    }
}

impl Exec for DEC_Mn {
    fn exec(&self, state: &mut State) {
        let ident = IdentU4::Mn(self.0);
        let value = state.fetch(ident);
        let carry = value == u4![0x0];
        let value = value - u4![1];
        let zero = value == u4![0x0];

        state
            .set(ident, value)
            .set_flag(Flags::C, carry)
            .set_flag(Flags::Z, zero);
    }
}

impl Cycles for DEC_Mn {
    fn cycles(&self) -> u32 { 7 }
}
