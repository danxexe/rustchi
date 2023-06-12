use crate::{
    change::*,
    flags::*,
    immediate::Source,
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
            Opcode::JP(jp) => {
                let op = match jp {
                    JP::S(s) => Option::Some(s),
                    JP::C(s) => if flags.contains(Flags::C) {
                        Option::Some(s)
                    } else {
                        Option::None
                    },
                    JP::NC(s) => if flags.contains(Flags::C) {
                        Option::None
                    } else {
                        Option::Some(s)
                    },
                    JP::Z(s) => if flags.contains(Flags::Z) {
                        Option::Some(s)
                    } else {
                        Option::None
                    },
                    JP::NZ(s) => if flags.contains(Flags::Z) {
                        Option::None
                    } else {
                        Option::Some(s)
                    },
                    JP::BA => {
                        let b = state.fetch_u4(IdentU4::B);
                        let a = state.fetch_u4(IdentU4::A);
                        Option::Some(u8::from_be_nibbles(vec![b, a]))
                    }
                };

                match op {
                    Option::None => changes.none(),
                    Option::Some(s) => {
                        changes
                        .register(Register::PCB(registers.NBP))
                        .register(Register::PCP(registers.NPP))
                        .register(Register::PCS(s))
                    }
                }
            }
            Opcode::INC(op) => {
                let ident = IdentU12::from(op);
                changes.push(state.change_u12(ident, state.fetch_u12(ident) + u12![1]))
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
            Opcode::LDv2(ld) => {
                changes.push(state.change_u4(ld.dest(), state.fetch_u4(ld.source())))
            }
            Opcode::LDPX(op) => {
                let data = match op {
                    LDPX::MX(i) => i,
                    LDPX::RQ(_, q) => state.fetch_u4(IdentU4::from(q)),
                };

                changes
                .push(state.change_u4(op.dest(), data))
                .push(state.change_u12(IdentU12::X, state.fetch_u12(IdentU12::X) + u12![1]))
            }
            Opcode::LBPX(l) => {
                changes
                .memory(Memory { address: registers.X, value: l.nibble(0) })
                .memory(Memory { address: registers.X + u12![1], value: l.nibble(1) })
                .register(Register::X(registers.X + u12![2]))
            }
            Opcode::SET(i) => {
                let f = self.state.fetch_u4(IdentU4::F) | i;
                changes
                .flags(Flags::from_bits(f.into()).unwrap())
            }
            Opcode::RST(i) => {
                let f = self.state.fetch_u4(IdentU4::F) & i;
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
            Opcode::AND(op) => {
                let (r, a, b) = match op {
                    AND::RI(r, i) => (r, state.fetch_u4(r.into()), i),
                    AND::RQ(r, q) => (r, state.fetch_u4(r.into()), state.fetch_u4(q.into())),
                };

                let value = a & b;

                changes
                    .push(state.change_u4(r.into(), value))
                    .flags(flags.clone().tap_mut(|flags| {
                        flags.set(Flags::Z, value == u4![0]);
                    }))
            }
            Opcode::ADD(op) => {
                let (r, a, b) = match op {
                    ADD::RI(r, i) => (r, state.fetch_u4(r.into()), i),
                    ADD::RQ(r, q) => (r, state.fetch_u4(r.into()), state.fetch_u4(q.into())),
                };

                let sum = u8![a] + u8![b];
                let (sum, carry) = if flags.contains(Flags::D) {
                    // assuming BCD digits <= 9
                    let carry = sum >= 10;
                    (if carry {u4![sum - 10]} else {u4![sum]}, carry)
                } else {
                    (u4![sum & 0xF], sum > 0xF)
                };

                changes
                .push(state.change_u4(r.into(), u4![sum]))
                .flags(flags.clone().tap_mut(|flags| {
                    flags.set(Flags::C, carry);
                    flags.set(Flags::Z, sum == u4![0]);
                }))
            }
            Opcode::ADC(op) => {
                let (r, a, b, bcd_supported) = match op {
                    ADC::XHi(i) => { let r = IdentU4::XH; (r, state.fetch_u4(r.into()), i, false) },
                    ADC::XLi(i) => { let r = IdentU4::XL; (r, state.fetch_u4(r.into()), i, false) },
                    ADC::YHi(i) => { let r = IdentU4::YH; (r, state.fetch_u4(r.into()), i, false) },
                    ADC::YLi(i) => { let r = IdentU4::YL; (r, state.fetch_u4(r.into()), i, false) },
                    ADC::RI(r, i) => (r.into(), state.fetch_u4(r.into()), i, true),
                    ADC::RQ(r, q) => (r.into(), state.fetch_u4(r.into()), state.fetch_u4(q.into()), true),
                };

                let carry = flags.intersection(Flags::C).bits();
                let sum = u8![a] + u8![b] + carry;

                let (sum, carry) = if bcd_supported && flags.contains(Flags::D) {
                    // assuming BCD digits <= 9
                    let carry = sum >= 10;
                    (if carry {u4![sum - 10]} else {u4![sum]}, carry)
                } else {
                    (u4![sum & 0xF], sum > 0xF)
                };

                changes
                .push(state.change_u4(r.into(), u4![sum]))
                .flags(flags.clone().tap_mut(|flags| {
                    flags.set(Flags::C, carry);
                    flags.set(Flags::Z, sum == u4![0]);
                }))
            }
            Opcode::PUSH(push) => {
                let ident = IdentU4::from(push);
                let data = self.state.fetch_u4(ident);
                let sp = registers.SP - 1;

                changes
                .register(Register::SP(sp))
                .memory(Memory { address: u12![sp], value: u4![data] })
            }
            Opcode::POP(pop) => {
                changes
                .register(Register::SP(registers.SP + 1))
                .push(state.change_u4(pop.into(), state.fetch_u4(IdentU4::MSP)))
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
            Opcode::RETS => todo!("{}", opcode),
            Opcode::RETD(_) => todo!("{}", opcode),
            Opcode::HALT => todo!("{}", opcode),
            Opcode::TODO(_) => todo!("{}", opcode),
            Opcode::UNKNOWN => todo!("{}", opcode),
        };

        let prev = Option::Some(self.state.to_owned());
        let state = self.state.apply(&changes).tap_mut(|state| {
            let delta_cycles = opcode.cycles();
            state.cycles += delta_cycles;
            state.memory.update_timers(delta_cycles);
            match opcode {
                Opcode::PSET(_, _) => (),
                // Reset next page pointer. 🤔 This seems like a hack.
                _ => state.registers.NPP = state.registers.PCP,
            }

            // TODO: This is hardcoded for prog timer interrupt for now
            if state.memory.prog_timer_data() == 0 && state.flags.contains(Flags::I) {
                state.flags.set(Flags::I, false);
                state.memory.set((state.registers.SP - 1).into(), state.registers.PCP);
                state.memory.set((state.registers.SP - 2).into(), state.registers.PCS.nibble(1));
                state.memory.set((state.registers.SP - 3).into(), state.registers.PCS.nibble(2));
                state.registers.SP -= 3;
                state.registers.NPP = u4![0x1];
                state.registers.PCP = u4![0x1];
                state.registers.PCS = u8![0x0C];
            }
        });

        (prev, state, changes)
    }
}
