use std::ops::Range;

use crate::primitive::*;

#[derive(Clone, Copy)]
pub struct Memory {
    bytes: [u4; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self {bytes: [u4::MIN; 4096]}
    }

    pub fn slice(&self, slice: Range<usize>) -> &[u4] {
        &self.bytes[slice]
    }

    pub fn get(self, addr: usize) -> u4 {
        if addr >= 0xF00 {
            return self.get_io(addr)
        }

        self.bytes[addr]
    }

    pub fn set(&mut self, addr: usize, val: u4) {
        self.bytes[addr] = val;

        if addr >= 0xF00 {
            self.set_io(addr, val)
        };
    }

    fn get_io(self, addr: usize) -> u4 {
        match addr {
            REG_CLKCHG_OSCC_VSC1_VSC0 => self.bytes[addr],
            REG_SVDDT_SVDON_SVC1_SVC0 => self.bytes[addr] & !u4![0b1000],
            _ => panic!("read IO! {:#X}", addr),
        }
    }

    fn set_io(&mut self, addr: usize, _val: u4) {
        match addr {
            REG_CLKCHG_OSCC_VSC1_VSC0 => (),
            REG_SVDDT_SVDON_SVC1_SVC0 => (),
            _ => panic!("write IO! {:#X}", addr),
        }
    }
}

// 0b1000 = CPU system clock switch | 0b0100 = OSC3 oscillation On/Off | 0b0011 = CPU operating voltage switch
const REG_CLKCHG_OSCC_VSC1_VSC0: usize = 0xF70;

// Supply voltage detection
// 0b1000 = SVD evaluation data | 0b0100 SVD circuit On/Off | 0b0011 = SVD criteria voltage setting
// 0b1000: 1 means Low, 0 means Normal. Let's keep it Normal.
const REG_SVDDT_SVDON_SVC1_SVC0: usize = 0xF73;
