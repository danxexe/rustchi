use crate::{
    primitive::u4,
    opcode::rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum FAN {
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for FAN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FAN::RI(r, i) => write!(f, "FAN {} {:#03X}", r, i),
            FAN::RQ(r, q) => write!(f, "FAN {} {}", r, q),
        }
    }
}
