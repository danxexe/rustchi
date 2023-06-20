use crate::{
    primitive::u4,
    rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CP {
    XHi(u4),
    XLi(u4),
    YHi(u4),
    YLi(u4),
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for CP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CP::XHi(i) => write!(f, "CP XH {:#03X}", i),
            CP::XLi(i) => write!(f, "CP XL {:#03X}", i),
            CP::YHi(i) => write!(f, "CP YH {:#03X}", i),
            CP::YLi(i) => write!(f, "CP YL {:#03X}", i),
            CP::RI(r, i) => write!(f, "CP {} {}", r, i),
            CP::RQ(r, q) => write!(f, "CP {} {}", r, q),
        }
    }
}
