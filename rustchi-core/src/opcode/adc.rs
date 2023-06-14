use crate::{
    primitive::u4,
    opcode::rq::*, state::State, flags::Flags,
};

use std::fmt;

use super::{Exec, IdentU4};

def_opcode! {
    pub enum ADC {
        XHi(u4),
        XLi(u4),
        YHi(u4),
        YLi(u4),
        RI(RQ, u4),
        RQ(RQ, RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::XHi(i) => write!(f, "{NAME} XH {:#03X}", i),
            Self::XLi(i) => write!(f, "{NAME} XL {:#03X}", i),
            Self::YHi(i) => write!(f, "{NAME} YH {:#03X}", i),
            Self::YLi(i) => write!(f, "{NAME} YL {:#03X}", i),
            Self::RI(r, i) => write!(f, "{NAME} {} {:#03X}", r, i),
            Self::RQ(r, q) => write!(f, "{NAME} {} {}", r, q),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let (r, a, b, bcd_supported) = match *self {
            ADC::XHi(i) => { let r = IdentU4::XH; (r, state.fetch_u4(r.into()), i, false) },
            ADC::XLi(i) => { let r = IdentU4::XL; (r, state.fetch_u4(r.into()), i, false) },
            ADC::YHi(i) => { let r = IdentU4::YH; (r, state.fetch_u4(r.into()), i, false) },
            ADC::YLi(i) => { let r = IdentU4::YL; (r, state.fetch_u4(r.into()), i, false) },
            ADC::RI(r, i) => (r.into(), state.fetch_u4(r.into()), i, true),
            ADC::RQ(r, q) => (r.into(), state.fetch_u4(r.into()), state.fetch_u4(q.into()), true),
        };

        let carry = state.flags.intersection(Flags::C).bits();
        let sum = u8![a] + u8![b] + carry;

        let (sum, carry) = if bcd_supported && state.flags.contains(Flags::D) {
            // assuming BCD digits <= 9
            let carry = sum >= 10;
            (if carry {u4![sum - 10]} else {u4![sum]}, carry)
        } else {
            (u4![sum & 0xF], sum > 0xF)
        };

        state
        .set_u4(r.into(), u4![sum])
        .set_flag(Flags::C, carry)
        .set_flag(Flags::Z, sum == u4![0]);
    }
}
