use crate::{
    opcode::{IdentU1, IdentU4, IdentU8, IdentU12},
    primitive::{u1, u4, u12, GetNibble},
    change,
    change::{Change, Changes, Register},
    flags::Flags,
    registers::*,
    memory::Memory,
};

#[derive(Clone)]
pub struct State {
    pub tick: u32,
    pub clock_speed: u32,
    pub cycles: u32,
    pub flags: Flags,
    pub registers: Registers,
    pub memory: Memory,
    pub changes: Changes,
}

impl State {
    pub fn new() -> Self {
        Self {
            tick: 1,
            clock_speed: 32_768,
            cycles: 0,
            flags: Flags::empty(),
            registers: Registers::zero(),
            memory: Memory::new(),
            changes: Changes::new(),
        }
    }

    pub fn clone_without_changes(&self) -> Self {
        Self {
            tick: self.tick.clone(),
            clock_speed: self.clock_speed.clone(),
            cycles: self.cycles.clone(),
            flags: self.flags.clone(),
            registers: self.registers.clone(),
            memory: self.memory.clone(),
            changes: Changes::new(),
        }
    }

    pub fn pc(&self) -> usize {
        let step: usize = self.registers.PCS.into();
        let page: usize = self.registers.PCP.into();
        let bank: usize = self.registers.PCB.into();

        step | (page << 8) | (bank << 12)
    }

    pub fn fetch_u1(&self, ident: IdentU1) -> u1 {
        match ident {
            IdentU1::PCB => self.registers.PCB,
            IdentU1::NBP => self.registers.NBP,
        }
    }

    pub fn fetch_u4(&self, ident: IdentU4) -> u4 {
        match ident {
            IdentU4::A => self.registers.A,
            IdentU4::B => self.registers.B,
            IdentU4::MX => self.memory.get(self.registers.X.into()),
            IdentU4::MY => self.memory.get(self.registers.Y.into()),
            IdentU4::MSP => self.memory.get(self.registers.SP.into()),
            IdentU4::XP => self.registers.X.upper_u4(),
            IdentU4::XH => self.registers.X.mid_u4(),
            IdentU4::XL => self.registers.X.low_u4(),
            IdentU4::YP => self.registers.Y.upper_u4(),
            IdentU4::YH => self.registers.Y.mid_u4(),
            IdentU4::YL => self.registers.Y.low_u4(),
            IdentU4::F => u4![self.flags.bits()],
            IdentU4::PCP => self.registers.PCP,
            IdentU4::NPP => self.registers.NPP,
            IdentU4::Mn(n) => self.memory.get(n.into()),
            IdentU4::Imm(i) => i,
        }
    }

    pub fn change_u4(&self, ident: IdentU4, value: u4) -> Change {
        match ident {
            IdentU4::A => Change::Register(Register::A(value)),
            IdentU4::B => Change::Register(Register::B(value)),
            IdentU4::MX => Change::Memory(change::Memory{address: self.registers.X, value}),
            IdentU4::MY => Change::Memory(change::Memory{address: self.registers.Y, value}),
            IdentU4::MSP => Change::Memory(change::Memory{address: self.registers.SP.into(), value}),
            IdentU4::XP => Change::Register(Register::X(self.registers.X.with_nibble(2, value))),
            IdentU4::XH => Change::Register(Register::X(self.registers.X.with_nibble(1, value))),
            IdentU4::XL => Change::Register(Register::X(self.registers.X.with_nibble(0, value))),
            IdentU4::YP => Change::Register(Register::Y(self.registers.Y.with_nibble(2, value))),
            IdentU4::YH => Change::Register(Register::Y(self.registers.Y.with_nibble(1, value))),
            IdentU4::YL => Change::Register(Register::Y(self.registers.Y.with_nibble(0, value))),
            IdentU4::F => Change::Flags(Flags::from_bits(value.into()).unwrap()),
            IdentU4::PCP => Change::Register(Register::PCP(value)),
            IdentU4::NPP => Change::Register(Register::NPP(value)),
            IdentU4::Mn(n) => Change::Memory(change::Memory { address: n.into(), value }),
            IdentU4::Imm(i) => panic!("can't change immediate value {}", i),
        }
    }

    pub fn fetch_u12(&self, ident: IdentU12) -> u12 {
        match ident {
            IdentU12::X => self.registers.X,
            IdentU12::Y => self.registers.Y,
        }
    }

    pub fn change_u12(&self, ident: IdentU12, value: u12) -> Change {
        match ident {
            IdentU12::X => Change::Register(Register::X(value)),
            IdentU12::Y => Change::Register(Register::Y(value)),
        }
    }

    pub fn set_u1(&mut self, ident: IdentU1, value: u1) -> &mut Self {
        match ident {
            IdentU1::PCB => {
                self.registers.PCB = value;
                self.changes.register(Register::PCB(value));
            }
            IdentU1::NBP => {
                self.registers.NBP = value;
                self.changes.register(Register::NBP(value));
            }
        }

        self
    }

    pub fn set_u4(&mut self, ident: IdentU4, value: u4) -> &mut Self {
        match ident {
            IdentU4::A => {
                self.registers.A = value;
                self.changes.register(Register::A(value));
            }
            IdentU4::B => {
                self.registers.B = value;
                self.changes.register(Register::B(value));
            }
            IdentU4::MX => {
                self.memory.set(self.registers.X.into(), value);
                self.changes.memory(change::Memory{address: self.registers.X, value});
            }
            IdentU4::MY => {
                self.memory.set(self.registers.Y.into(), value);
                self.changes.memory(change::Memory{address: self.registers.Y, value});
            }
            IdentU4::MSP => {
                self.memory.set(self.registers.SP.into(), value);
                self.changes.memory(change::Memory{address: self.registers.SP.into(), value});
            }
            IdentU4::XP => {
                self.registers.X = self.registers.X.with_nibble(2, value);
                self.changes.register(Register::X(self.registers.X));
            }
            IdentU4::XH => {
                self.registers.X = self.registers.X.with_nibble(1, value);
                self.changes.register(Register::X(self.registers.X));
            }
            IdentU4::XL => {
                self.registers.X = self.registers.X.with_nibble(0, value);
                self.changes.register(Register::X(self.registers.X));
            }
            IdentU4::YP => {
                self.registers.Y = self.registers.Y.with_nibble(2, value);
                self.changes.register(Register::Y(self.registers.Y));
            }
            IdentU4::YH => {
                self.registers.Y = self.registers.Y.with_nibble(1, value);
                self.changes.register(Register::Y(self.registers.Y));
            }
            IdentU4::YL => {
                self.registers.Y = self.registers.Y.with_nibble(0, value);
                self.changes.register(Register::Y(self.registers.Y));
            }
            IdentU4::F => {
                self.flags = Flags::from_bits(value.into()).unwrap();
                self.changes.flags(self.flags);
            }
            IdentU4::PCP => {
                self.registers.PCP = value;
                self.changes.register(Register::PCP(value));
            }
            IdentU4::NPP => {
                self.registers.PCP = value;
                self.changes.register(Register::PCP(value));
            }
            IdentU4::Mn(n) => {
                self.memory.set(n.into(), value);
                Change::Memory(change::Memory { address: n.into(), value });
            }
            IdentU4::Imm(i) => {
                panic!("can't change immediate value {}", i);
            }
        }

        self
    }

    pub fn set_u8(&mut self, ident: IdentU8, value: u8) -> &mut Self {
        match ident {
            IdentU8::PCS => {
                self.registers.PCS = value;
                self.changes.register(Register::PCS(value));
            }
        }

        self
    }

    pub fn set_u12(&mut self, ident: IdentU12, value: u12) -> &mut Self {
        match ident {
            IdentU12::X => {
                self.registers.X = value;
                self.changes.register(Register::X(value));
            }
            IdentU12::Y => {
                self.registers.Y = value;
                self.changes.register(Register::Y(value));
            }
        }

        self
    }

    pub fn set_flag(&mut self, flags: Flags, value: bool) -> &mut Self {
        self.flags.set(flags, value);
        self.changes.flags(self.flags);
        self
    }

    pub fn apply(&self, changes: &Changes) -> Self {
        let mut state = self.clone();
        state.tick += 1;

        for change in changes.iter() {
            match change {
                Change::Register(register) => match register {
                    Register::PCS(val) => state.registers.PCS = *val,
                    Register::PCP(val) => state.registers.PCP = *val,
                    Register::PCB(val) => state.registers.PCB = *val,
                    Register::NPP(val) => state.registers.NPP = *val,
                    Register::NBP(val) => state.registers.NBP = *val,
                    Register::SP(val) => state.registers.SP = *val,
                    Register::X(val) => state.registers.X = *val,
                    Register::Y(val) => state.registers.Y = *val,
                    Register::RP(val) => state.registers.RP = *val,
                    Register::A(val) => state.registers.A = *val,
                    Register::B(val) => state.registers.B = *val,
                }
                Change::Memory(memory) => {
                    state.memory.set(memory.address.into(), memory.value)
                }
                Change::Flags(flags) => {
                    state.flags = *flags
                }
            }
        }

        state
    }
}
