use crate::registers::*;

pub struct State {
    pub np: u8,
    pub pc: u16,
    pub flags: Flags,
    pub registers: Registers,
}

impl State {
    pub fn new() -> Self {
        Self {
            np: 0x01,
            pc: 0x0100,
            flags: Flags::empty(),
            registers: Registers::zero(),
        }
    }
}

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags: u8 {
        const C = 0x1 << 0;
        const Z = 0x1 << 1;
        const D = 0x1 << 2;
        const I = 0x1 << 3;
    }
}
