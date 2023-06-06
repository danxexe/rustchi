use crate::{
    opcode::ident::*,
    opcode::rq::*,
};

use std::fmt;

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

impl From<POP> for Ident {
    fn from(value: POP) -> Ident {
        match value {
            POP::R(rq) => Ident::from(rq),
            POP::XP => Ident::XP,
            POP::XH => Ident::XH,
            POP::XL => Ident::XL,
            POP::YP => Ident::YP,
            POP::YH => Ident::YH,
            POP::YL => Ident::YL,
            POP::F => Ident::F,
        }
    }
}

impl fmt::Display for POP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            POP::R(r) => write!(f, "POP {}", r),
            p => write!(f, "POP {:?}", p)
        }
    }
}
