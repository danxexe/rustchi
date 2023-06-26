use crate::prelude::*;

use crate::{
    change::*,
    immediate::Source,
    opcode::*,
    registers::*,
};

use std::borrow::Borrow;
use std::ops::Add;
use std::usize;

use tap::Tap;

pub struct Interpreter {
    pub state: State,
    pub prev_pc: Option<usize>,
    pub changes: Changes,
    pub rom: Vec<u8>,
    pub cycle_counter: u64,
 }

 impl Interpreter {
    pub fn load(bytes: Vec<u8>) -> Self {
        Self {
            state: State::new(),
            prev_pc: Option::None,
            changes: Changes::new(),
            rom: bytes,
            cycle_counter: 0,
        }
    }

    pub fn reset_cycle_counter(&mut self) {
        self.cycle_counter = 0;
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

    pub fn next_opcode(&self) -> Opcode {
        Opcode::decode(self.words().skip(self.pc()).take(1).last().unwrap())
    }

    pub fn step(&mut self) {
        let opcode = self.next_opcode();
        self.prev_pc = Option::Some(self.pc());

        self.exec(opcode);
    }

    fn read_source(&self, source: Source) -> u8 {
        let registers = self.state.registers;
        let memory = &self.state.memory;
        match source {
            Source::U4(value) => value.into(),
            Source::L(l) => l.u8(),
            Source::Reg(reg) => match reg {
                Reg::MX => memory.get(registers.X.into()).into(),
                Reg::MY => memory.get(registers.Y.into()).into(),
                _ => registers.get(reg),
            },
        }
    }

    fn exec(&mut self, opcode: Opcode) {
        // TODO: this should overflow to PCP
        self.state.registers.PCS += 1;

        let state = &self.state;
        let registers = &self.state.registers;
        let memory = &self.state.memory;
        let mut changes = Changes::new();

        match opcode.borrow() {
            Opcode::LD(reg, i) => {
                let data = self.read_source(*i);

                match reg {
                    Reg::A => changes.register(Register::A(data.try_into().unwrap())),
                    Reg::B => changes.register(Register::B(data.try_into().unwrap())),
                    Reg::SPH => changes.register(Register::SP(registers.SP.with_nibble(1, data.try_into().unwrap()))),
                    Reg::SPL => changes.register(Register::SP(registers.SP.with_nibble(0, data.try_into().unwrap()))),
                    Reg::XP => changes.register(Register::X(registers.X.with_nibble(2, data.try_into().unwrap()))),
                    Reg::YP => changes.register(Register::Y(registers.Y.with_nibble(2, data.try_into().unwrap()))),
                    Reg::X => {
                        changes.register(Register::X(registers.X.with_nibble(1, data.nibble(1)).with_nibble(0, data.nibble(0))))
                    }
                    Reg::Y => {
                        changes.register(Register::Y(registers.Y.with_nibble(1, data.nibble(1)).with_nibble(0, data.nibble(0))))
                    }
                    Reg::MX => changes.memory(Memory { address: registers.X, value: data.try_into().unwrap() }),
                    Reg::MY => changes.memory(Memory { address: registers.Y, value: data.try_into().unwrap() }),
                }
            }
            Opcode::LBPX(l) => {
                changes
                .memory(Memory { address: registers.X, value: l.nibble(0) })
                .memory(Memory { address: registers.X + u12![1], value: l.nibble(1) })
                .register(Register::X(registers.X + u12![2]))
            }
            Opcode::SET(i) => {
                let f = self.state.fetch(IdentU4::F) | *i;
                changes
                .flags(Flags::from_bits(f.into()).unwrap())
            }
            Opcode::RST(i) => {
                let f = self.state.fetch(IdentU4::F) & *i;
                changes
                .flags(Flags::from_bits(f.into()).unwrap())
            }
            Opcode::CALL(s) => {
                changes
                .memory(Memory::at((registers.SP - 1).into(), registers.PCP))
                .memory(Memory::at((registers.SP - 2).into(), registers.PCS.nibble(1)))
                .memory(Memory::at((registers.SP - 3).into(), registers.PCS.nibble(0)))
                .register(Register::SP(registers.SP - 3))
                .register(Register::PCP(registers.NPP))
                .register(Register::PCS(u8::from(*s)))
            }
            Opcode::CALZ(s) => {
                changes
                .memory(Memory::at((registers.SP - 1).into(), registers.PCP))
                .memory(Memory::at((registers.SP - 2).into(), registers.PCS.nibble(1)))
                .memory(Memory::at((registers.SP - 3).into(), registers.PCS.nibble(0)))
                .register(Register::SP(registers.SP - 3))
                .register(Register::PCP(u4![0]))
                .register(Register::PCS(u8::from(*s)))
            }
            Opcode::RET => {
                changes
                .register(Register::PCS(
                    0u8
                    .with_nibble(0, memory.get(registers.SP.into()))
                    .with_nibble(1, memory.get(registers.SP.add(1).into()))
                ))
                .register(Register::PCP(memory.get(registers.SP.add(2).into())))
                .register(Register::SP(registers.SP.add(3)))
            }
            Opcode::RETD(l) => {
                changes
                .register(Register::PCS(
                    0u8
                    .with_nibble(0, memory.get(registers.SP.into()))
                    .with_nibble(1, memory.get(registers.SP.add(1).into()))
                ))
                .register(Register::PCP(memory.get(registers.SP.add(2).into())))
                .register(Register::SP(registers.SP.add(3)))
                .memory(Memory { address: registers.X, value: l.nibble(0) })
                .memory(Memory { address: registers.X + u12![1], value: l.nibble(1) })
                .push(state.change_u12(IdentU12::X, state.fetch_u12(IdentU12::X) + u12![2]))
            }
            Opcode::NOP5 => &mut changes,
            Opcode::NOP7 => &mut changes,
            Opcode::Op(op) => {
                op.exec(&mut self.state);
                changes.append(&mut self.state.changes)
            }
            Opcode::RETS => todo!("{}", opcode),
            Opcode::HALT => todo!("{}", opcode),
            Opcode::TODO(_) => todo!("{}", opcode),
            Opcode::UNKNOWN => todo!("{}", opcode),
        };

        self.state.apply(&changes).tap_mut(|state| {
            let delta_cycles = opcode.cycles();
            state.cycles += delta_cycles;
            state.update_timers(delta_cycles);

            let process_interrupts = match opcode {
                Opcode::Op(op) => op.interruptible(),
                _ => true,
            };

            if process_interrupts {
                state.check_interrupts();
                state.process_interrupts();
                state.registers.NPP = state.registers.PCP;
            };

            self.cycle_counter += u64::from(delta_cycles);
        });

        self.changes = changes;
    }
}
