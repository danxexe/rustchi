use crate::prelude::*;

use crate::{
    change::{self, Change, Changes, Register},
    registers::*,
    memory::{self, Memory},
    input::Input,
};

const TIMER_1HZ_CYCLES: u32 = 32768;
const TIMER_256HZ_CYCLES: u32 = 128;

#[derive(Clone)]
pub struct State {
    pub tick: u32,
    pub clock_speed: u32,
    pub cycles: u32,
    pub flags: Flags,
    pub registers: Registers,
    pub memory: Memory,
    pub changes: Changes,
    pub input: Input,
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
            input: Input::all_high(),
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

    pub fn apply(&mut self, changes: &Changes) -> &mut Self {
        let state = self;
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

    fn timer_data(&self) -> u8 {
        let bytes = self.memory.bytes.borrow();
        u8![0]
            .with_nibble(0, bytes[memory::REG_PROG_TIMER_DATA_LO])
            .with_nibble(1, bytes[memory::REG_PROG_TIMER_DATA_HI])
    }

    pub fn update_timers(&mut self, delta_cycles: u32) {
        self.memory.clock_timer_ticks += delta_cycles;
        let timer_data = self.timer_data();

        {
            let mut bytes = self.memory.bytes.borrow_mut();

            if bytes[memory::REG_PROG_TIMER_RESET_ENABLE].is_set(u4![0b0001]) {
                if self.memory.prog_timer_ticks >= TIMER_256HZ_CYCLES {
                    self.memory.prog_timer_ticks -= TIMER_256HZ_CYCLES;
                    let timer_data = timer_data - 1;
                    bytes[memory::REG_PROG_TIMER_DATA_LO] = timer_data.nibble(0);
                    bytes[memory::REG_PROG_TIMER_DATA_HI] = timer_data.nibble(1);
                }

                self.memory.prog_timer_ticks += delta_cycles;
            }
        }

        // println!("prog_timer_data {}", self.timer_data());
        // println!("prog_timer_ticks {}", self.memory.prog_timer_ticks);
    }

    pub fn check_interrupts(&mut self) -> Option<u8> {
        let timer_data = self.timer_data();
        let mut bytes = self.memory.bytes.borrow_mut();

        // Interrupt vector (PCP and PCS), low to high priority
        // 0x102 Clock timer
        // 0x104 Stopwatch timer
        // 0x106 Input (K00–K03)
        // 0x108 Input (K10–K13)
        // 0x10A Serial interface
        // 0x10C Programmable timer

        if bytes[memory::REG_PROG_TIMER_RESET_ENABLE].is_set(u4![0b0001]) && timer_data == 0 {
            self.memory.prog_timer_ticks += 12;
            bytes[memory::REG_PROG_TIMER_DATA_LO] = bytes[memory::REG_PROG_TIMER_RELOAD_DATA_LO];
            bytes[memory::REG_PROG_TIMER_DATA_HI] = bytes[memory::REG_PROG_TIMER_RELOAD_DATA_HI];

            bytes[memory::REG_PROGRAMMABLE_TIMER_INTERRUPT_FACTOR_FLAGS] = bytes[memory::REG_PROGRAMMABLE_TIMER_INTERRUPT_FACTOR_FLAGS] | u4![0b0001];

            if bytes[memory::REG_EIPT].is_set(u4![0b0001]) {
                return Some(u8![0x0C]);
            } else {
                return None;
            }
        }
        
        // 1hz clock timer. It's the only one used by the P1.
        if bytes[memory::REG_EIT1_EIT2_EIT8_EIT32].is_set(u4![0b1000]) && self.memory.clock_timer_ticks >= TIMER_1HZ_CYCLES {
            self.memory.clock_timer_ticks = 12;
            bytes[memory::REG_CLOCK_INTERRUPT_FACTOR_FLAGS] = bytes[memory::REG_CLOCK_INTERRUPT_FACTOR_FLAGS] | u4![0b1000];

            return Some(u8![0x02]);
        }

        return None;
    }

    pub fn process_interrupts(&mut self, pcs: u8) -> u64 {
        if !self.flags.contains(Flags::I) {
            return 0;
        }

        let mut bytes = self.memory.bytes.borrow_mut();

        self.flags.set(Flags::I, false);
        bytes[usize::from(self.registers.SP - 1)] = self.registers.PCP;
        bytes[usize::from(self.registers.SP - 2)] = self.registers.PCS.nibble(1);
        bytes[usize::from(self.registers.SP - 3)] = self.registers.PCS.nibble(2);
        self.registers.SP -= 3;
        self.registers.NPP = u4![0x1];
        self.registers.PCP = u4![0x1];
        self.registers.PCS = pcs;
        return 12;
    }
}

impl FetchIdent<RQ, u4> for State {
    fn fetch(&self, rq: RQ) -> u4 {
        self.fetch(IdentU4::from(rq))
    }
}

impl FetchIdent<IdentU4, u4> for State {
    fn fetch(&self, ident: IdentU4) -> u4 {
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
}

impl FetchIdent<IdentU8, u8> for State {
    fn fetch(&self, ident: IdentU8) -> u8 {
        match ident {
            IdentU8::PCS => self.registers.PCS,
            IdentU8::SP => self.registers.SP,
            IdentU8::XHL => self.registers.X.low_mid_u8(),
            IdentU8::YHL => self.registers.Y.low_mid_u8(),
            IdentU8::Imm(value) => value,
        }
    }
}

impl FetchIdent<IdentU12, u12> for State {
    fn fetch(&self, ident: IdentU12) -> u12 {
        self.fetch_u12(ident)
    }
}

impl FetchIdent<Flags, u8> for State {
    fn fetch(&self, flag: Flags) -> u8 {
        (self.flags & flag).bits()
    }
}

impl SetIdent<RQ, u4> for State {
    fn set(&mut self, rq: RQ, value: u4) -> &mut Self {
        self.set(IdentU4::from(rq), value)
    }
}

impl SetIdent<IdentU12, u12> for State {
    fn set(&mut self, ident: IdentU12, value: u12) -> &mut Self {
        self.set_u12(ident, value)
    }
}

impl SetIdent<IdentU4, u4> for State {
    fn set(&mut self, ident: IdentU4, value: u4) -> &mut Self {
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
                self.registers.NPP = value;
                self.changes.register(Register::NPP(value));
            }
            IdentU4::Mn(n) => {
                self.memory.set(n.into(), value);
                self.changes.memory(change::Memory { address: n.into(), value });
            }
            IdentU4::Imm(i) => {
                panic!("can't change immediate value {}", i);
            }
        }

        self
    }
}

impl SetIdent<IdentU8, u8> for State {
    fn set(&mut self, ident: IdentU8, value: u8) -> &mut Self {
        match ident {
            IdentU8::PCS => {
                self.registers.PCS = value;
                self.changes.register(Register::PCS(value));
            }
            IdentU8::SP => {
                self.registers.SP = value;
                self.changes.register(Register::SP(value));
            }
            IdentU8::XHL => {
                self.registers.X = self.registers.X.with_hl(value);
                self.changes.register(Register::X(self.registers.X));
            }
            IdentU8::YHL => {
                self.registers.Y = self.registers.Y.with_hl(value);
                self.changes.register(Register::Y(self.registers.Y));
            }
            IdentU8::Imm(value) => {
                panic!("can't change immediate value {}", value);
            }
        }

        self
    }
}
