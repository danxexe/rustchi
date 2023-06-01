use std::usize;

use crate::immediate::Source;
use crate::primitive::GetNibble;
use crate::state::*;
use crate::opcode::*;
use crate::registers::*;

pub struct Interpreter {
    pub state: State,
    pub prev_state: Option<State>,
    pub rom: Vec<u8>,
 }

 impl Interpreter {
    pub fn load(bytes: Vec<u8>) -> Self {
        Self {
            state: State::new(),
            prev_state: Option::None,
            rom: bytes,
        }
    }

    pub fn pc(&self) -> usize {
        self.state.pc()
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
        self.prev_state = Option::Some(self.state.clone());
        (self.prev_state, self.state) = self.exec(opcode);
    }

    fn exec(&self, opcode: Opcode) -> (Option<State>, State) {
        let next_state = match opcode {
            Opcode::PSET(nbp, npp) => self.state.next(|mut state| {
                state.registers.NBP = nbp;
                state.registers.NPP = npp;
            }),
            Opcode::JP(s) => self.state.next(|mut state| {
                state.registers.PCB = state.registers.NBP;
                state.registers.PCP = state.registers.NPP;
                state.registers.PCS = s.into();
            }),
            Opcode::LD(reg, i) => self.state.next(|mut state| {
                let data = match i {
                    Source::I(i) => i.u8(),
                    Source::L(l) => l.u8(),
                    Source::Reg(reg) => self.state.registers.get(reg),
                };

                match reg {
                    Reg::A => state.registers.A = data.into(),
                    Reg::SPH => state.registers.SP = state.registers.SP.with_nibble(1, data.into()),
                    Reg::SPL => state.registers.SP = state.registers.SP.with_nibble(0, data.into()),
                    Reg::XP => state.registers.X = state.registers.X.with_nibble(2, data.into()),
                    Reg::X => state.registers.X = state.registers.X.with_nibble(1, data.nibble(1)).with_nibble(0, data.nibble(0)),
                    Reg::MX => state.memory.set(state.registers.X.into(), data.into()),
                    _ => panic!("{}", opcode),
                }
            }),
            Opcode::RST(i) => self.state.next(|mut state| {
                state.flags = Flags::from_bits(i.into()).unwrap();
            }),
            Opcode::CALL(s) => self.state.next(|mut state| {
                state.push(state.registers.PCP);
                state.push(state.registers.PCS.nibble(1));
                state.push(state.registers.PCS.nibble(0));
                state.registers.PCP = state.registers.NPP;
                state.registers.PCS = s.into();
            }),
            _ => panic!("Interpreter::exec {}", opcode),
        };

        (Option::Some(self.state.clone()), next_state)
    }
}
