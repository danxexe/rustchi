use std::ops::Range;

use crate::primitive::*;

#[derive(Clone, Copy)]
pub struct Memory {
    bytes: [u4; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self {bytes: [u4::MIN; 4096]}
    }

    pub fn slice(&self, slice: Range<usize>) -> &[u4] {
        &self.bytes[slice]
    }

    pub fn get(self, addr: usize) -> u4 {
        if addr >= 0xF00 {
            return self.get_io(addr)
        }

        self.bytes[addr]
    }

    pub fn set(&mut self, addr: usize, val: u4) {
        self.bytes[addr] = val;

        if addr >= 0xF00 {
            self.set_io(addr, val)
        };
    }

    fn get_io(self, addr: usize) -> u4 {
        let val = self.bytes[addr];
        match addr {
            REG_EIT1_EIT2_EIT8_EIT32 => val,
            REG_EISW1_EISW0 => val,
            REG_EIPT => val,
            REG_EISIO => val,
            REG_EIK03_EIK02_EIK01_EIK00 => val,
            REG_EIK13_EIK12_EIK11_EIK10 => val,
            REG_RD3_RD2_RD1_RD0 => val,
            REG_RD7_RD6_RD5_RD4 => val,
            REG_K03_K02_K01_K00 => val, // TODO: input
            REG_R43_R42_R41_R40 => val,
            REG_CLKCHG_OSCC_VSC1_VSC0 => val,
            REG_ALOFF_ALON_LDUTY_HLMOD => val,
            REG_LC3_LC2_LC1_LC0 => val,
            REG_SVDDT_SVDON_SVC1_SVC0 => val & !u4![0b1000],
            REG_PTCOUT_PTC2_PTC1_PTC0 => val,
            _ => panic!("read IO! {:#X}", addr),
        }
    }

    fn set_io(&mut self, addr: usize, val: u4) {
        match addr {
            REG_EIT1_EIT2_EIT8_EIT32 => assert!(val == u4![0x8], "1Hz interrupt timer expected"),
            REG_EISW1_EISW0 => assert!(val == u4![0x0], "stopwatch interrupt not expected"),
            REG_EIPT => assert!(val == u4![0x1], "programmable timer interrupt expected"),
            REG_EISIO => assert!(val == u4![0x0], "serial interface interrupt not expected"),
            REG_EIK03_EIK02_EIK01_EIK00 => assert!(val == u4![0x0], "K03-K00 interrupt not expected"),
            REG_EIK13_EIK12_EIK11_EIK10 => assert!(val == u4![0x0], "K13-K10 interrupt not expected"),
            REG_RD3_RD2_RD1_RD0 => (), // TODO: timer
            REG_RD7_RD6_RD5_RD4 => (), // TODO: timer
            REG_K03_K02_K01_K00 => (),
            REG_R43_R42_R41_R40 => assert!(val == u4![0xF], "REG_R43_R42_R41_R40 not expected"),
            REG_CLKCHG_OSCC_VSC1_VSC0 => (),
            REG_ALOFF_ALON_LDUTY_HLMOD => (), // TODO: display,
            REG_LC3_LC2_LC1_LC0 => assert!(val == u4![0x8]),
            REG_SVDDT_SVDON_SVC1_SVC0 => (),
            REG_TMRST_WDRST => (),  // TODO: timer
            REG_SWRST_SWRUN => assert!(val == u4![0x2]), // TODO: timer
            REG_PTRST_PTRUN => unexpected("write", REG_PTRST_PTRUN, val, u4![0x2]), // TODO: timer
            REG_PTCOUT_PTC2_PTC1_PTC0 => assert!(val == u4![0x2]), // TODO: timer
            _ => panic!("write IO! {:#X} {:#X}", addr, val),
        }
    }
}

fn unexpected(t: &str, addr: usize, val: u4, expected: u4) {
    assert!(val == expected, "unexpected IO {} {:#X} {:#X}", t, addr, val)
}

// RW | Interrupt mask register (clock timer in Hz)
const REG_EIT1_EIT2_EIT8_EIT32: usize = 0xF10;

// RW | 0b0010 = Interrupt mask register (stopwatch 1 Hz) | 0b0001 = Interrupt mask register (stopwatch 10 Hz)
const REG_EISW1_EISW0: usize = 0xF11;

// RW | 0b0001 = Interrupt mask register (programmable timer)
const REG_EIPT: usize = 0xF12;

// RW | 0b0001 = Interrupt mask register (serial interface)
const REG_EISIO: usize = 0xF13;

// RW | Interrupt mask register K03-K00
const REG_EIK03_EIK02_EIK01_EIK00: usize = 0xF14;

// RW | Interrupt mask register K13-K10
const REG_EIK13_EIK12_EIK11_EIK10: usize = 0xF15;

// RW | Programmable timer reload data (low-order)
const REG_RD3_RD2_RD1_RD0: usize = 0xF26;

// RW | Programmable timer reload data (high-order)
const REG_RD7_RD6_RD5_RD4: usize = 0xF27;

// R | Input port K03-K00
const REG_K03_K02_K01_K00: usize = 0xF40;

// RW | R43 = Output port (R43), Buzzer output (BZ) | R42 = Clock output (FOUT), [Buzzer inverted output (BZ)] | R40 = Clock inverted output (FOUT)
const REG_R43_R42_R41_R40: usize = 0xF54;

// RW | 0b1000 = CPU system clock switch | 0b0100 = OSC3 oscillation On/Off | 0b0011 = CPU operating voltage switch
const REG_CLKCHG_OSCC_VSC1_VSC0: usize = 0xF70;

// RW | All LCD dots fade out control | All LCD dots displayed control | LCD drive duty switch | Heavy load protection mode
const REG_ALOFF_ALON_LDUTY_HLMOD: usize = 0xF71;

// LCD contrast adjustment
const REG_LC3_LC2_LC1_LC0: usize = 0xF72;

// Supply voltage detection
// R | 0b1000 = SVD evaluation data. 1 means Low, 0 means Normal.
// RW | 0b0100 SVD circuit On/Off | 0b0011 = SVD criteria voltage setting
const REG_SVDDT_SVDON_SVC1_SVC0: usize = 0xF73;

// W | 0b0010 = TMRST = Clock timer reset | 0b0001 = WDRST = Watchdog timer reset
const REG_TMRST_WDRST: usize = 0xF76;

// W | 0b0010 = SWRST = Stopwatch timer reset | 0b0001 = SWRUN = Stopwatch timer Run/Stop
const REG_SWRST_SWRUN: usize = 0xF77;

// W | 0b0010 = SWRST = Programmable timer reset | 0b0001 = SWRUN = Programmable timer Run/Stop
const REG_PTRST_PTRUN: usize = 0xF78;

// RW | 0b0010 = Programmable timer clock output | 0b0111 = Programmable timer input clock selection
const REG_PTCOUT_PTC2_PTC1_PTC0: usize = 0xF79;
