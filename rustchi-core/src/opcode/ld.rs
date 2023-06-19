use crate::{
    opcode::*
};

use super::{RQ, IdentU4};

def_opcode! {
    pub enum LD {
        r_XP(RQ),
        r_XH(RQ),
        r_XL(RQ),
        r_YP(RQ),
        r_YH(RQ),
        r_YL(RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::r_XP(r) => write!(f, "{NAME} {} XP", r),
            Self::r_XH(r) => write!(f, "{NAME} {} XH", r),
            Self::r_XL(r) => write!(f, "{NAME} {} XL", r),
            Self::r_YP(r) => write!(f, "{NAME} {} YP", r),
            Self::r_YH(r) => write!(f, "{NAME} {} YH", r),
            Self::r_YL(r) => write!(f, "{NAME} {} YL", r),
        }
    }
}

impl LD {
    pub fn dest(&self) -> IdentU4 {
        match self {
            LD::r_XP(r) => IdentU4::from(*r),
            LD::r_XH(r) => IdentU4::from(*r),
            LD::r_XL(r) => IdentU4::from(*r),
            LD::r_YP(r) => IdentU4::from(*r),
            LD::r_YH(r) => IdentU4::from(*r),
            LD::r_YL(r) => IdentU4::from(*r),
        }
    }

    pub fn source(&self) -> IdentU4 {
        match self {
            LD::r_XP(_) => IdentU4::XP,
            LD::r_XH(_) => IdentU4::XH,
            LD::r_XL(_) => IdentU4::XL,
            LD::r_YP(_) => IdentU4::YP,
            LD::r_YH(_) => IdentU4::YH,
            LD::r_YL(_) => IdentU4::YL,
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut crate::state::State) {
        state.set_u4(self.dest(), state.fetch_u4(self.source()));
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
