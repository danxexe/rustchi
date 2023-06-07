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
            REG_SVD => self.bytes[addr] & u4![0x7], // Supply voltage detection, bit 3 on means Low, off means Normal. Let's keep it Normal
            _ => panic!("read IO! {:#X}", addr),
        }
    }

    fn set_io(&mut self, addr: usize, _val: u4) {
        match addr {
            REG_SVD => (), // Supply voltage detection, ignore
            _ => panic!("write IO! {:#X}", addr),
        }
    }
}

const REG_SVD: usize = 0xF73;
