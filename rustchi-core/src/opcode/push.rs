use crate::prelude::*;
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
            PUSH::R(r) => write!(f, "PUSH {}", r),
            p => write!(f, "PUSH {:?}", p)
        }
    }
}
