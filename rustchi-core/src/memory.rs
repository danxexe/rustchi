use crate::primitive::u4;

#[derive(Clone, Copy)]
pub struct Memory {
    bytes: [u4; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self {bytes: [0.into(); 4096]}
    }

    pub fn get(&self, i: usize) -> u4 {
        self.bytes[i]
    }

    pub fn set(&mut self, i: usize, val: u4) {
        self.bytes[i] = val;
    }
}
