use crate::{
    primitive::u4,
    ident::IdentU4,
};

use bitmatch::bitmatch;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum RQ {
    A,
    B,
    MX,
    MY,
}

impl From<u4> for RQ {
    #[bitmatch]
    fn from(value: u4) -> Self {
        #[bitmatch]
        match u8::from(value) {
            "00" => Self::A,
            "01" => Self::B,
            "10" => Self::MX,
            "11" => Self::MY,
        }
    }
}

impl From<RQ> for IdentU4 {
    fn from(value: RQ) -> Self {
        match value {
            RQ::A => IdentU4::A,
            RQ::B => IdentU4::B,
            RQ::MX => IdentU4::MX,
            RQ::MY => IdentU4::MY,
        }
    }
}

impl fmt::Display for RQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RQ::A => write!(f, "A"),
            RQ::B => write!(f, "B"),
            RQ::MX => write!(f, "MX"),
            RQ::MY => write!(f, "MY"),
        }
    }
}

#[bitmatch]
pub fn rq(r: u16) -> String {
    #[bitmatch]
    match r {
        "00" => format!("A"),
        "01" => format!("B"),
        "10" => format!("MX"),
        "11" => format!("MY"),
    }
}
