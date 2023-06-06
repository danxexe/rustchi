use std::fmt;

use super::RQ;

#[derive(Debug, Clone, Copy)]
pub enum LD {
    // XHL(u8),
    RYL(RQ),
}

impl fmt::Display for LD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Self::XHL(x) => write!(f, "LD XHL {}", x),
            Self::RYL(r) => write!(f, "LD {} YL", r),
        }
    }
}
