use crate::primitive::*;
use crate::registers::*;
use crate::memory::Memory;

#[derive(Clone, Copy)]
pub struct State {
    pub flags: Flags,
    pub registers: Registers,
    pub memory: Memory,
}

impl State {
    pub fn new() -> Self {
        Self {
            flags: Flags::empty(),
            registers: Registers::zero(),
            memory: Memory::new(),
        }
    }

    pub fn next<F>(&self, mut f: F)  -> Self where F: FnMut(&mut Self) {
        let state = &mut self.clone();
        state.registers.PCS += 1;
        f(state);
        *state
    }

    pub fn push(&mut self, val: u4) {
        self.registers.SP -= 1;
        self.memory.set(self.registers.SP.into(), val)
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
