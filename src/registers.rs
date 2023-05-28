use std::fmt;

use crate::immediate::Source;

#[derive(Debug)]
pub enum Reg {
    SPH,
    SPL,
    X,
    Y,
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
    pub sp: u8,
    pub a: u8,
}

impl Registers {
    pub fn zero() -> Self {
        Self {
            sp: 0,
            a: 0,
        }
    }

    pub fn get(&self, reg: Reg) -> u8 {
        match reg {
            Reg::SPH => self.sp & 0xF0,
            Reg::SPL => self.sp & 0x0F,
            Reg::A => self.a,
            _ => panic!("Registers::get {}", reg),
        }
    }

    pub fn load(&self, reg: Reg, i: Source) -> Self {
        let data = match i {
            Source::I(i) => i.u8(),
            Source::L(l) => l.u8(),
            Source::Reg(reg) => self.get(reg),
        };

        match reg {
            Reg::A => Self { a: data, ..*self },
            Reg::SPH => Self { a: (data << 4) | self.get(Reg::SPL), ..*self },
            Reg::SPL => Self { a: self.get(Reg::SPH) | data, ..*self },
            _ => panic!("Registers::load {}", reg),
        }
    }
}
