use std::fmt;

use crate::primitive::u4;

use super::{RQ, IdentU4};

#[derive(Debug, Clone, Copy)]
pub enum LDPX {
    MX(u4),
    RQ(RQ, RQ),
}

impl fmt::Display for LDPX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LDPX::MX(i) => write!(f, "LDPX MX {}", i),
            LDPX::RQ(r, q) => write!(f, "LDPX {} {}", r, q),
        }
    }
}

impl LDPX {
    pub fn dest(&self) -> IdentU4 {
        match self {
            LDPX::MX(_) => IdentU4::MX,
            LDPX::RQ(r, _) => IdentU4::from(*r),
        }
    }
}
