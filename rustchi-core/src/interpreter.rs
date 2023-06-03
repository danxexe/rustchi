use std::usize;

use crate::change::*;
use crate::flags::*;
use crate::immediate::Source;
use crate::primitive::GetNibble;
use crate::state::*;
use crate::opcode::*;
use crate::registers::*;

pub struct Interpreter {
    pub state: State,
    pub prev_state: Option<State>,
    pub changes: Changes,
    pub rom: Vec<u8>,
 }

 impl Interpreter {
    pub fn load(bytes: Vec<u8>) -> Self {
        Self {
            state: State::new(),
            prev_state: Option::None,
            changes: Changes::new(),
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

        match self.exec(opcode) {
            (prev_state, state, changes) => {
                self.prev_state = prev_state;
                self.state = state;
                self.changes = changes;
            }
        };
    }

    fn exec(&self, opcode: Opcode) -> (Option<State>, State, Changes) {
        let registers = &self.state.registers;
        let memory = &self.state.memory;
        let mut changes = Changes::new();

        match opcode {
            Opcode::PSET(nbp, npp) => {
                changes
                .register(Register::NBP(nbp))
                .register(Register::NPP(npp))
            },
            Opcode::JP(s) => {
                changes
                .register(Register::PCB(registers.NBP))
                .register(Register::PCP(registers.NPP))
                .register(Register::PCS(s.into()))
            }
            Opcode::LD(reg, i) => {
                let data = match i {
                    Source::U4(i) => i.into(),
                    Source::L(l) => l.u8(),
                    Source::Reg(reg) => match reg {
                        Reg::MX => memory.get(registers.X.into()).into(),
                        _ => registers.get(reg),
                    },
                };

                match reg {
                    Reg::A => changes.register(Register::A(data.into())),
                    Reg::SPH => changes.register(Register::SP(registers.SP.with_nibble(1, data.into()))),
                    Reg::SPL => changes.register(Register::SP(registers.SP.with_nibble(0, data.into()))),
                    Reg::XP => changes.register(Register::X(registers.X.with_nibble(2, data.into()))),
                    Reg::X => changes.register(Register::X(registers.X.with_nibble(1, data.nibble(1)).with_nibble(0, data.nibble(0)))),
                    Reg::MX => changes.memory(Memory { address: registers.X, value: data.into() }),
                    _ => panic!("{}", opcode),
                }
            }
            Opcode::RST(i) => {
                changes
                .flags(Flags::from_bits(i.into()).unwrap())
            }
            Opcode::CALL(s) => {
                changes
                .memory(Memory::at((registers.SP - 1).into(), registers.PCP))
                .memory(Memory::at((registers.SP - 2).into(), registers.PCS.nibble(1)))
                .memory(Memory::at((registers.SP - 3).into(), registers.PCS.nibble(0)))
                .register(Register::SP(registers.SP - 3))
                .register(Register::PCP(registers.NPP))
                .register(Register::PCS(s.into()))
            }
            Opcode::NOP5 => &mut changes,
            Opcode::NOP7 => &mut changes,
            Opcode::AND(reg, source) => {
                match source {
                    Source::U4(value) => {
                        let value = registers.get(reg) & u8::from(value);
                        changes.register(Register::from((reg, value.into())))
                    }
                    _ => panic!("{}", opcode),
                }
            }
            _ => panic!("{}", opcode),
        };

        (
            Option::Some(self.state.to_owned()),
            self.state.apply(&changes),
            changes,
        )
    }
}
