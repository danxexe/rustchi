use bitmatch::bitmatch;
use std::fmt;
use crate::{
    primitive::u4,
    opcode::ident::Ident,
};

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

impl From<RQ> for Ident {
    fn from(value: RQ) -> Self {
        match value {
            RQ::A => Ident::A,
            RQ::B => Ident::B,
            RQ::MX => Ident::MX,
            RQ::MY => Ident::MY,
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
