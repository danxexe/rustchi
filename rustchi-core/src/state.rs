use crate::change::*;
use crate::flags::Flags;
use crate::registers::*;
use crate::memory::Memory;

#[derive(Clone)]
pub struct State {
    pub tick: u32,
    pub flags: Flags,
    pub registers: Registers,
    pub memory: Memory,
}

impl State {
    pub fn new() -> Self {
        Self {
            tick: 0,
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

        state
    }
}
