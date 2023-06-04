use std::slice::Iter;

use crate::{primitive::*, flags::Flags, registers::Reg};

#[derive(Clone)]
pub enum Change {
    Register(Register),
    Memory(Memory),
    Flags(Flags),
}

#[derive(Clone)]
pub enum Register {
    PCS(u8),
    PCP(u4),
    PCB(u1),
    NPP(u4),
    NBP(u1),
    SP(u8),
    X(u12),
    Y(u12),
    RP(u4),
    A(u4),
    B(u4),
}

impl From<(Reg, u4)> for Register {
    fn from((reg, val): (Reg, u4)) -> Self {
        match reg {
            Reg::A => Self::A(val.into()),
            _ => panic!()
        }
    }
}

#[derive(Clone)]
pub struct Memory {
    pub address: u12,
    pub value: u4,
}

impl Memory {
    pub fn at(address: u12, value: u4) -> Self {
        Self { address, value }
    }
}

pub struct Changes(Vec<Change>);


impl Changes {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn register(&mut self, register: Register) -> &mut Self {
        self.0.push(Change::Register(register));
        self
    }

    pub fn memory(&mut self, memory: Memory) -> &mut Self {
        self.0.push(Change::Memory(memory));
        self
    }


    pub fn flags(&mut self, flags: Flags) -> &mut Self {
        self.0.push(Change::Flags(flags));
        self
    }
}

impl Changes {
    pub fn iter(&self) -> Iter<Change> {
        self.0.iter()
    }
}
