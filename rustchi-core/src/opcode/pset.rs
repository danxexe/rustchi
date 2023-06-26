use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub struct PSET(pub u1, pub u4);
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{NAME} {} {:#X}", self.0, self.1)
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        state
            .set_u1(IdentU1::NBP, self.0)
            .set(IdentU4::NPP, self.1);
    }

    fn interruptible(&self) -> bool {
        false
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
