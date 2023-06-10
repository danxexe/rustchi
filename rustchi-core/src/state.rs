use crate::{
    opcode::{IdentU4, IdentU12},
    primitive::{u4, u12, GetNibble},
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
}

impl State {
    pub fn new() -> Self {
        Self {
            tick: 0,
            clock_speed: 32_768,
            cycles: 0,
            flags: Flags::empty(),
            registers: Registers::zero(),
            memory: Memory::new(),
        }
    }

    pub fn pc(&self) -> usize {
        let step: usize = self.registers.PCS.into();
        let page: usize = self.registers.PCP.into();
        let bank: usize = self.registers.PCB.into();

        step | (page << 8) | (bank << 12)
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
            IdentU4::YP => Change::Register(Register::Y(self.registers.X.with_nibble(2, value))),
            IdentU4::YH => Change::Register(Register::Y(self.registers.X.with_nibble(1, value))),
            IdentU4::YL => Change::Register(Register::Y(self.registers.X.with_nibble(0, value))),
            IdentU4::F => Change::Flags(Flags::from_bits(value.into()).unwrap()),
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

    pub fn apply(&self, changes: &Changes) -> Self {
        let mut state = self.clone();
        state.tick += 1;
        state.registers.PCS += 1;


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

        if !self.flags.contains(Flags::I) && state.flags.contains(Flags::I) {
            println!("Interrupts enabled!");
        }

        if self.flags.contains(Flags::I) && !state.flags.contains(Flags::I) {
            println!("Interrupts disabled!");
        }

        if state.tick == 751 {
        // if state.tick == 643 {
            panic!("break!");
        }

        state
    }
}
