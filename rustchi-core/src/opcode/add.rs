use crate::{
    primitive::u4,
    opcode::rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ADD {
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for ADD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ADD::RI(r, i) => write!(f, "ADD {} {:#03X}", r, i),
            ADD::RQ(r, q) => write!(f, "ADD {} {}", r, q),
        }
    }
}
