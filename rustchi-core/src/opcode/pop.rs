use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub enum POP {
        R(RQ),
        XP,
        XH,
        XL,
        YP,
        YH,
        YL,
        F,
    }
}

impl From<POP> for IdentU4 {
    fn from(value: POP) -> IdentU4 {
        match value {
            POP::R(rq) => IdentU4::from(rq),
            POP::XP => IdentU4::XP,
            POP::XH => IdentU4::XH,
            POP::XL => IdentU4::XL,
            POP::YP => IdentU4::YP,
            POP::YH => IdentU4::YH,
            POP::YL => IdentU4::YL,
            POP::F => IdentU4::F,
        }
    }
}

impl fmt::Display for POP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            POP::R(r) => write!(f, "{NAME} {}", r),
            p => write!(f, "{NAME} {:?}", p)
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let value = state.fetch(IdentU4::MSP);
        let sp = state.fetch(IdentU8::SP) + 1;

        state
            .set(IdentU4::from(*self), value)
            .set(IdentU8::SP, sp);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
