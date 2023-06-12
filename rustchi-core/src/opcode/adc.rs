use crate::{
    primitive::u4,
    opcode::rq::*,
};

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ADC {
    XHi(u4),
    XLi(u4),
    YHi(u4),
    YLi(u4),
    RI(RQ, u4),
    RQ(RQ, RQ),
}

impl fmt::Display for ADC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ADC::XHi(i) => write!(f, "ADC XH {:#03X}", i),
            ADC::XLi(i) => write!(f, "ADC XL {:#03X}", i),
            ADC::YHi(i) => write!(f, "ADC YH {:#03X}", i),
            ADC::YLi(i) => write!(f, "ADC YL {:#03X}", i),
            ADC::RI(r, i) => write!(f, "ADC {} {:#03X}", r, i),
            ADC::RQ(r, q) => write!(f, "ADC {} {}", r, q),
        }
    }
}
