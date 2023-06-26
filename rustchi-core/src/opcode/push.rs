use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub enum PUSH {
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

impl From<PUSH> for IdentU4 {
    fn from(value: PUSH) -> IdentU4 {
        match value {
            PUSH::R(rq) => IdentU4::from(rq),
            PUSH::XP => IdentU4::XP,
            PUSH::XH => IdentU4::XH,
            PUSH::XL => IdentU4::XL,
            PUSH::YP => IdentU4::YP,
            PUSH::YH => IdentU4::YH,
            PUSH::YL => IdentU4::YL,
            PUSH::F => IdentU4::F,
        }
    }
}

impl fmt::Display for PUSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PUSH::R(r) => write!(f, "{NAME} {}", r),
            p => write!(f, "{NAME} {:?}", p)
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let value = state.fetch(IdentU4::from(*self));
        let sp = state.fetch(IdentU8::SP) - 1;

        state
            .set(IdentU8::SP, sp)
            .set(IdentU4::MSP, value);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
