use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub enum ACP {
        X(RQ),
        Y(RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X(r) => write!(f, "{NAME}X MX {}", r),
            Self::Y(r) => write!(f, "{NAME}Y MY {}", r),
        }
    }
}

impl T {
    pub fn dest(&self) -> IdentU4 {
        match *self {
            Self::X(r) => r,
            Self::Y(r) => r,
        }.into()
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let x: u12 = state.fetch(IdentU12::X);
        let mx: u8 = state.fetch(IdentU4::MX).into();
        let r: u8 = state.fetch(self.dest()).into();
        let c: u8 = state.fetch(Flags::C);
        let d: bool = state.fetch(Flags::D) != 0;
        let val = mx + r + c;

        let (new_val, carry) = if d {
            if val >= 10 {
                (u4![(val - 10) & 0xF], true)
            } else {
                (u4![val], false)
            } 
        } else {
            (u4![val & 0xF], (val >> 4) != 0)
        };

        state
            .set(IdentU4::MX, new_val)
            .set(IdentU12::X, u12![x + u12![1]])
            .set_flag(Flags::C, carry)
            .set_flag(Flags::Z, new_val == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}
