use crate::{
    primitive::u4,
    opcode::rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CP {
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for CP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CP::RI(r, i) => write!(f, "CP {} {}", r, i),
            CP::RQ(r, q) => write!(f, "CP {} {}", r, q),
        }
    }
}
