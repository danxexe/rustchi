use std::fmt;

use super::{RQ, IdentU4};

#[derive(Debug, Clone, Copy)]
pub enum LD {
    // XHL(u8),
    RXP(RQ),
    RXH(RQ),
    RXL(RQ),
    RYP(RQ),
    RYH(RQ),
    RYL(RQ),
}

impl fmt::Display for LD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Self::XHL(x) => write!(f, "LD XHL {}", x),
            Self::RXP(r) => write!(f, "LD {} XP", r),
            Self::RXH(r) => write!(f, "LD {} XH", r),
            Self::RXL(r) => write!(f, "LD {} XL", r),
            Self::RYP(r) => write!(f, "LD {} YP", r),
            Self::RYH(r) => write!(f, "LD {} YH", r),
            Self::RYL(r) => write!(f, "LD {} YL", r),
        }
    }
}

impl LD {
    pub fn source(&self) -> IdentU4 {
        match self {
            LD::RXP(r) => IdentU4::from(*r),
            LD::RXH(r) => IdentU4::from(*r),
            LD::RXL(r) => IdentU4::from(*r),
            LD::RYP(r) => IdentU4::from(*r),
            LD::RYH(r) => IdentU4::from(*r),
            LD::RYL(r) => IdentU4::from(*r),
        }
    }

    pub fn dest(&self) -> IdentU4 {
        match self {
            LD::RXP(_) => IdentU4::XP,
            LD::RXH(_) => IdentU4::XH,
            LD::RXL(_) => IdentU4::XL,
            LD::RYP(_) => IdentU4::YP,
            LD::RYH(_) => IdentU4::YH,
            LD::RYL(_) => IdentU4::YL,
        }
    }
}
