use crate::{
    opcode::ident::*,
    opcode::rq::*,
};

use std::fmt;

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

impl From<PUSH> for Ident {
    fn from(value: PUSH) -> Ident {
        match value {
            PUSH::R(rq) => Ident::from(rq),
            PUSH::XP => Ident::XP,
            PUSH::XH => Ident::XH,
            PUSH::XL => Ident::XL,
            PUSH::YP => Ident::YP,
            PUSH::YH => Ident::YH,
            PUSH::YL => Ident::YL,
            PUSH::F => Ident::F,
        }
    }
}

impl fmt::Display for PUSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PUSH::R(r) => write!(f, "PUSH {}", r),
            p => write!(f, "PUSH {:?}", p)
        }
    }
}
