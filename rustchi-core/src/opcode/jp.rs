use crate::prelude::*;
use std::fmt;

def_opcode! {
    pub enum JP {
        S(u8),
        C(u8),
        NC(u8),
        Z(u8),
        NZ(u8),
        BA,
    }
}

impl fmt::Display for JP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JP::S(s) => write!(f, "{NAME} {:#04X}", s),
            JP::C(s) => write!(f, "{NAME} C {:#04X}", s),
            JP::NC(s) => write!(f, "{NAME} NC {:#04X}", s),
            JP::Z(s) => write!(f, "{NAME} Z {:#04X}", s),
            JP::NZ(s) => write!(f, "{NAME} NZ {:#04X}", s),
            JP::BA => write!(f, "{NAME} BA"),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let op = match *self {
            JP::S(s) => Option::Some(s),
            JP::C(s) => if state.flags.contains(Flags::C) {
                Option::Some(s)
            } else {
                Option::None
            },
            JP::NC(s) => if state.flags.contains(Flags::C) {
                Option::None
            } else {
                Option::Some(s)
            },
            JP::Z(s) => if state.flags.contains(Flags::Z) {
                Option::Some(s)
            } else {
                Option::None
            },
            JP::NZ(s) => if state.flags.contains(Flags::Z) {
                Option::None
            } else {
                Option::Some(s)
            },
            JP::BA => {
                let b = state.fetch(IdentU4::B);
                let a = state.fetch(IdentU4::A);
                Option::Some(u8::from_be_nibbles(vec![b, a]))
            }
        };

        match op {
            Option::None => (),
            Option::Some(s) => {
                let nbp = state.fetch_u1(IdentU1::NBP);
                let npp = state.fetch(IdentU4::NPP);

                state
                .set_u1(IdentU1::PCB, nbp)
                .set(IdentU4::PCP, npp)
                .set(IdentU8::PCS, s);
            }
        }
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
