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

    pub fn step(&mut self) -> () {
        let opcode = Opcode::decode(self.words().skip(self.pc()).take(1).last().unwrap());
        println!("Executing: {}", opcode);
        let _new_state = self.exec(opcode);
    }

    fn exec(&mut self, opcode: Opcode) -> () {
        self.state = match opcode {
            Opcode::PSET(p) => State {
                np: p.into(),
                pc: self.state.pc + 1,
                ..self.state
            },
            Opcode::JP(s) => State {
                pc: u16::from(s) | (u16::from(self.state.np) << 8),
                ..self.state
            },
            Opcode::LD(reg, i) => State {
                pc: self.state.pc + 1,
                registers: self.state.registers.load(reg, i),
                ..self.state
            },
            Opcode::RST(i) => State {
                pc: self.state.pc + 1,
                flags: Flags::from_bits(i.into()).unwrap(),
                ..self.state
            },
            _ => State {..self.state},
        };
    }
}
