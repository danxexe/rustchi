use crate::{
    change::*,
    flags::*,
    immediate::Source,
    opcode::ident::Ident,
    primitive::{GetNibble},
    state::*,
    opcode::*,
    registers::*,
};

use std::ops::Add;
use std::usize;

use tap::Tap;

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

    fn read_source(&self, source: Source) -> u8 {
        let registers = self.state.registers;
        let memory = self.state.memory;
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

    fn exec(&self, opcode: Opcode) -> (Option<State>, State, Changes) {
        let state = &self.state;
        let registers = &self.state.registers;
        let memory = &self.state.memory;
        let flags = &self.state.flags;
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
            Opcode::JP_NZ(s) => {
                if flags.contains(Flags::Z) {
                    &mut changes
                } else {
                    changes
                    .register(Register::PCB(registers.NBP))
                    .register(Register::PCP(registers.NPP))
                    .register(Register::PCS(s.into()))
                }
            }
            Opcode::LD(reg, i) => {
                let data = self.read_source(i);

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
            Opcode::LDPX(reg, i) => {
                let data = self.read_source(i);
                let low_mid = registers.X.low_mid_u8() + 1;
                let upper = registers.X.upper_u12();
                let new_x = Register::X((upper | u12![low_mid]).try_into().unwrap());

                match reg {
                    Reg::MX => {
                        changes
                        .memory(Memory { address: registers.X, value: data.try_into().unwrap() })
                        .register(new_x)
                    }
                    Reg::A => {
                        changes
                        .register(Register::A(data.try_into().unwrap()))
                        .register(new_x)
                    }
                    _ => panic!("{}", opcode)
                }
            }
            Opcode::LBPX(i, j) => {
                let i = self.read_source(i);
                let j = self.read_source(j);
                let low_mid = registers.X.low_mid_u8() + 2;
                let upper = registers.X.upper_u12();

                changes
                .memory(Memory { address: registers.X, value: i.try_into().unwrap() })
                .memory(Memory { address: registers.X + u12![1], value: j.try_into().unwrap() })
                .register(Register::X((upper | u12![low_mid]).try_into().unwrap()))
            }
            Opcode::SET_F(i) => {
                let f = self.state.fetch_u4(Ident::F);
                let f = f | i;
                changes
                .flags(Flags::from_bits(f.into()).unwrap())
            }
            Opcode::RST_F(i) => {
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
            Opcode::CALZ(s) => {
                changes
                .memory(Memory::at((registers.SP - 1).into(), registers.PCP))
                .memory(Memory::at((registers.SP - 2).into(), registers.PCS.nibble(1)))
                .memory(Memory::at((registers.SP - 3).into(), registers.PCS.nibble(0)))
                .register(Register::SP(registers.SP - 3))
                .register(Register::PCP(u4![0]))
                .register(Register::PCS(s.into()))
            }
            Opcode::RET => {
                changes
                .register(Register::PCS(
                    0u8
                    .with_nibble(0, memory.get(registers.SP.into()))
                    .with_nibble(1, memory.get(registers.SP.add(1).into()))
                    .add(1)
                ))
                .register(Register::PCP(memory.get(registers.SP.add(2).into())))
                .register(Register::SP(registers.SP.add(3)))
            }
            Opcode::NOP5 => &mut changes,
            Opcode::NOP7 => &mut changes,
            Opcode::AND(a, b) => {
                let data_a = self.read_source(a);
                let data_b = self.read_source(b);
                let value = data_a & data_b;
                let flags = flags.clone().tap_mut(|flags|
                    flags.set(Flags::Z, value == 0)
                );

                match a {
                    Source::Reg(Reg::MX) | Source::Reg(Reg::MY) => {
                        changes.memory(Memory { address: registers.X, value: value.try_into().unwrap() })
                    }
                    Source::Reg(reg) => {
                        changes
                        .register(Register::from((reg, value.try_into().unwrap())))
                        .flags(flags)
                    }
                    _ => panic!("{}", opcode),
                }
            }
            Opcode::ADD(r, i) => {
                let data = self.read_source(i);
                let sum = registers.get(r) + data;
                let (sum, carry) = if flags.contains(Flags::D) {
                    (sum - 10, sum > 10)
                } else {
                    (sum & 0xF, sum > 0xF)
                };

                changes
                .register(Register::from((r, u4![sum])))
                .flags(flags.clone().tap_mut(|flags| {
                    flags.set(Flags::C, carry);
                    flags.set(Flags::Z, sum == 0);
                }))
            }
            Opcode::PUSH(push) => {
                let ident = Ident::from(push);
                let data = self.state.fetch_u4(ident);
                let sp = registers.SP - 1;

                changes
                .register(Register::SP(sp))
                .memory(Memory { address: u12![sp], value: u4![data] })
            }
            Opcode::POP(pop) => {
                changes
                .register(Register::SP(registers.SP + 1))
                .push(state.change_u4(pop.into(), state.fetch_u4(Ident::MSP)))
            }
            Opcode::CP(cp) => {
                let (a, b) = match cp {
                    CP::RI(r, i) => (state.fetch_u4(r.into()), i),
                    CP::RQ(r, q) => (state.fetch_u4(r.into()), state.fetch_u4(q.into())),
                };

                changes.flags(flags.clone().tap_mut(|flags| {
                    flags.set(Flags::C, a < b);
                    flags.set(Flags::Z, a == b);
                }))
            },
            Opcode::JP_C(_) => todo!("{}", opcode),
            Opcode::JP_NC(_) => todo!("{}", opcode),
            Opcode::JP_Z(_) => todo!("{}", opcode),
            Opcode::JP_BA => todo!("{}", opcode),
            Opcode::RETS => todo!("{}", opcode),
            Opcode::RETD(_) => todo!("{}", opcode),
            Opcode::HALT => todo!("{}", opcode),
            Opcode::INC(_) => todo!("{}", opcode),
            Opcode::TODO(_) => todo!("{}", opcode),
            Opcode::UNKNOWN => todo!("{}", opcode),
        };

        let prev = Option::Some(self.state.to_owned());
        let state = self.state.apply(&changes).tap_mut(|state|
            match opcode {
                Opcode::PSET(_, _) => (),
                // Reset next page pointer. 🤔 This seems like a hack.
                _ => state.registers.NPP = state.registers.PCP,
            }
        );

        (prev, state, changes)
    }
}
