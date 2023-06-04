#![allow(non_snake_case)]

use std::fmt;

use crate::primitive::*;

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    SPH,
    SPL,
    X,
    Y,
    XP,
    YP,
    A,
    B,
    MX,
    MY,
}

impl From<u16> for Reg {
    fn from(item: u16) -> Reg {
        match item {
            0x0 => Reg::A,
            0x1 => Reg::B,
            0x2 => Reg::MX,
            0x3 => Reg::MY,
            _ => panic!("invalid register 0x{:01X}", item),
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy)]
pub struct Registers {
    pub PCS: u8,
    pub PCP: u4,
    pub PCB: u1,
    pub NPP: u4,
    pub NBP: u1,
    pub SP: u8,
    pub X: u12,
    pub Y: u12,
    pub RP: u4,
    pub A: u4,
    pub B: u4,
}

impl Registers {
    pub fn zero() -> Self {
        Self {
            PCS: 0x00,
            PCP: u4![0x1],
            PCB: u1::MIN,
            NPP: u4![0x1],
            NBP: u1::MIN,
            SP: 0x00,
            X: u12::MIN,
            Y: u12::MIN,
            RP: u4::MIN,
            A: u4::MIN,
            B: u4::MIN,
        }
    }

    pub fn get(&self, reg: Reg) -> u8 {
        match reg {
            Reg::SPH => self.SP.nibble(1).into(),
            Reg::SPL => self.SP.nibble(0).into(),
            Reg::A => self.A.into(),
            _ => panic!("Registers::get {}", reg),
        }
    }
}
