use std::usize;

use crate::state::*;
use crate::opcode::*;

pub struct Interpreter {
    pub state: State,
    pub rom: Vec<u8>,
 }

 impl Interpreter {
    pub fn load(bytes: Vec<u8>) -> Self {
        Self {
            state: State::new(),
            rom: bytes,
        }
    }

    pub fn pc(&self) -> usize {
        self.state.pc.try_into().unwrap()
    }

    pub fn words(&self) -> impl Iterator<Item = u16> + '_ {
        self.rom.chunks_exact(2).map(|bytes|
            u16::from_be_bytes([bytes[0], bytes[1]])
        )
    }

    pub fn disassemble(&self, offset: usize) -> impl Iterator<Item = (usize, String)> + '_ {
        self.words().skip(offset).enumerate().map(move |(i, word)| {
            let address = offset + i;
            (address, format!("0x{address:04X} {word:04X} {}", Opcode::decode(word)))
        })
    }

    pub fn step(&mut self) {
        let opcode = Opcode::decode(self.words().skip(self.pc()).take(1).last().unwrap());
        self.state = self.exec(opcode);
    }

    fn exec(&self, opcode: Opcode) -> State {
        match opcode {
            Opcode::PSET(p) => self.state.next(|mut state| {
                state.np = p.into();
            }),
            Opcode::JP(s) => self.state.next(|mut state| {
                state.pc = u16::from(s) | (u16::from(self.state.np) << 8);
            }),
            Opcode::LD(reg, i) => self.state.next(|mut state| {
                state.registers = state.registers.load(reg, i);
            }),
            Opcode::RST(i) => self.state.next(|mut state| {
                state.flags = Flags::from_bits(i.into()).unwrap();
            }),
            _ => panic!("Interpreter::exec {}", opcode),
        }
    }
}
