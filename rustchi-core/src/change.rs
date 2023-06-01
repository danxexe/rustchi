use std::ops::Deref;

use crate::{primitive::*, flags::Flags};

pub enum Change {
    Register(Register),
    Memory(Memory),
    Flags(Flags),
}

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

impl Deref for Changes {
    type Target = Vec<Change>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
