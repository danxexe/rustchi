use crate::{
    primitive::u4,
    opcode::rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AND {
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for AND {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AND::RI(r, i) => write!(f, "AND {} {:#03X}", r, i),
            AND::RQ(r, q) => write!(f, "AND {} {}", r, q),
        }
    }
}
