#![allow(non_camel_case_types)]

use crate::prelude::*;
use crate::{
    immediate::*,
    registers::*,
};

automod::dir!("src/opcode");

pub use {
    adc::*,
    add::*,
    and::*,
    or::*,
    cp::*,
    dec_mn::*,
    fan::*,
    inc::*,
    inc_mn::*,
    jp::*,
    ld::*,
    ldpx::*,
    ldpy::*,
    push::*,
    pop::*,
    pset::*,
    rlc::*,
    rrc::*,
    xor::*,
    acp::*,
};

use bitmatch::bitmatch;
use std::{fmt, rc::Rc};

#[derive(Clone)]
pub enum Opcode {
    CALL(S),
    CALZ(S),
    RET,
    RETS,
    RETD(u8),
    NOP5,
    NOP7,
    HALT,
    LD(Reg, Source),
    LBPX(u8),
    SET(u4),
    RST(u4),
    Op(Rc<dyn Op>),
    TODO(String),
    UNKNOWN,
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::*;
        match self {
            CALL(s) => write!(f, "CALL {}", s),
            CALZ(s) => write!(f, "CALZ {}", s),
            RET => write!(f, "RET"),
            RETS => write!(f, "RETS"),
            RETD(l) => write!(f, "RETD {:#04X}", l),
            NOP5 => write!(f, "NOP5"),
            NOP7 => write!(f, "NOP7"),
            HALT => write!(f, "HALT"),
            LD(r, l) => write!(f, "LD {} {}", r, l),
            LBPX(l) => write!(f, "LBPX {:#04X}", l),
            SET(i) => write!(f, "SET F {:#X}", i),
            RST(i) => write!(f, "RST F {:#X}", i),
            Op(op) => write!(f, "{}", op),
            TODO(s) => write!(f, "{} #TODO", s),
            UNKNOWN => write!(f, "??"),
        }
    }
}

impl Opcode {
    #[bitmatch]
    pub fn decode(instruction: u16) -> Opcode {
        #[bitmatch]
        match instruction {
            "0000_1110_010p_qqqq" => op!(PSET(u1![p], u4![q])),
            "0000_0000_ssss_ssss" => op!(JP::S(u8![s])),
            "0000_0010_ssss_ssss" => op!(JP::C(u8![s])),
            "0000_0011_ssss_ssss" => op!(JP::NC(u8![s])),
            "0000_0110_ssss_ssss" => op!(JP::Z(u8![s])),
            "0000_0111_ssss_ssss" => op!(JP::NZ(u8![s])),
            "0000_1111_1110_1000" => op!(JP::BA),
            "0000_0100_ssss_ssss" => Opcode::CALL(s.into()),
            "0000_0101_ssss_ssss" => Opcode::CALZ(s.into()),
            "0000_1111_1101_1111" => Opcode::RET,
            "0000_1111_1101_1110" => Opcode::RETS,
            "0000_0001_llll_llll" => Opcode::RETD(u8![l]),
            "0000_1111_1111_1011" => Opcode::NOP5,
            "0000_1111_1111_1111" => Opcode::NOP7,
            "0000_1111_1111_1000" => Opcode::HALT,
            "0000_1110_1110_0000" => op!(INC::X),
            "0000_1110_1111_0000" => op!(INC::Y),
            "0000_1011_xxxx_xxxx" => op!(LD::XHL(u8![x])),
            "0000_1000_yyyy_yyyy" => op!(LD::YHL(u8![y])),
            "0000_1110_1000_00rr" => op!(LD::XP_r(rq![r])),
            "0000_1110_1000_01rr" => op!(LD::XH_r(rq![r])),
            "0000_1110_1000_10rr" => op!(LD::XL_r(rq![r])),
            "0000_1110_1001_00rr" => op!(LD::YP_r(rq![r])),
            "0000_1110_1001_01rr" => op!(LD::YH_r(rq![r])),
            "0000_1110_1001_10rr" => op!(LD::YL_r(rq![r])),
            "0000_1110_1010_00rr" => op!(LD::r_XP(rq![r])),
            "0000_1110_1010_01rr" => op!(LD::r_XH(rq![r])),
            "0000_1110_1010_10rr" => op!(LD::r_XL(rq![r])),
            "0000_1110_1011_00rr" => op!(LD::r_YP(rq![r])),
            "0000_1110_1011_01rr" => op!(LD::r_YH(rq![r])),
            "0000_1110_1011_10rr" => op!(LD::r_YL(rq![r])),
            "0000_1010_0000_iiii" => op!(ADC::XHi(u4![i])),
            "0000_1010_0001_iiii" => op!(ADC::XLi(u4![i])),
            "0000_1010_0010_iiii" => op!(ADC::YHi(u4![i])),
            "0000_1010_0011_iiii" => op!(ADC::YLi(u4![i])),
            "0000_1010_0100_iiii" => op!(CP::XHi(u4![i])),
            "0000_1010_0101_iiii" => op!(CP::XLi(u4![i])),
            "0000_1010_0110_iiii" => op!(CP::YHi(u4![i])),
            "0000_1010_0111_iiii" => op!(CP::YLi(u4![i])),
            "0000_1110_00rr_iiii" => op!(LD::r_i(rq![r], u4![i])),
            "0000_1110_1100_rrqq" => op!(LD::r_q(rq![r], rq![q])),
            "0000_1111_1010_nnnn" => op!(LD::A_Mn(u4![n])),
            "0000_1111_1011_nnnn" => op!(LD::B_Mn(u4![n])),
            "0000_1111_1000_nnnn" => op!(LD::Mn_A(u4![n])),
            "0000_1111_1001_nnnn" => op!(LD::Mn_B(u4![n])),
            "0000_1110_0110_iiii" => op!(LDPX::MX(u4![i])),
            "0000_1110_1110_rrqq" => op!(LDPX::RQ(rq![r], rq![q])),
            "0000_1110_0111_iiii" => op!(LDPY::MY(u4![i])),
            "0000_1110_1111_rrqq" => op!(LDPY::RQ(rq![r], rq![q])),
            "0000_1001_llll_llll" => Opcode::LBPX(u8![l]),
            "0000_1111_0100_iiii" => Opcode::SET(u4![i]),
            "0000_1111_0101_iiii" => Opcode::RST(u4![i]),
            "0000_1111_0100_0001" => Opcode::TODO(format!("SCF")),
            "0000_1111_0101_1110" => Opcode::TODO(format!("RCF")),
            "0000_1111_0100_0010" => Opcode::TODO(format!("SZF")),
            "0000_1111_0101_1101" => Opcode::TODO(format!("RZF")),
            "0000_1111_0100_0100" => Opcode::TODO(format!("SDF")),
            "0000_1111_0101_1011" => Opcode::TODO(format!("RDF")),
            "0000_1111_0100_1000" => Opcode::TODO(format!("EI")),
            "0000_1111_0101_0111" => Opcode::TODO(format!("DI")),
            "0000_1111_1101_1011" => Opcode::TODO(format!("INC SP")),
            "0000_1111_1100_1011" => Opcode::TODO(format!("DEC SP")),
            "0000_1111_1100_00rr" => op!(PUSH::R(RQ::from(u4![r]))),
            "0000_1111_1100_0100" => op!(PUSH::XP),
            "0000_1111_1100_0101" => op!(PUSH::XH),
            "0000_1111_1100_0110" => op!(PUSH::XL),
            "0000_1111_1100_0111" => op!(PUSH::YP),
            "0000_1111_1100_1000" => op!(PUSH::YH),
            "0000_1111_1100_1001" => op!(PUSH::YL),
            "0000_1111_1100_1010" => op!(PUSH::F),
            "0000_1111_1101_00rr" => op!(POP::R(RQ::from(u4![r]))),
            "0000_1111_1101_0100" => op!(POP::XP),
            "0000_1111_1101_0101" => op!(POP::XH),
            "0000_1111_1101_0110" => op!(POP::XL),
            "0000_1111_1101_0111" => op!(POP::YP),
            "0000_1111_1101_1000" => op!(POP::YH),
            "0000_1111_1101_1001" => op!(POP::YL),
            "0000_1111_1101_1010" => op!(POP::F),
            "0000_1111_1110_00rr" => Opcode::LD(Reg::SPH, Source::Reg(r.into())),
            "0000_1111_1111_00rr" => Opcode::LD(Reg::SPL, Source::Reg(r.into())),
            "0000_1111_1110_01rr" => Opcode::TODO(format!("LD {} SPH", rq(r))),
            "0000_1111_1111_01rr" => Opcode::TODO(format!("LD {} SPL", rq(r))),
            "0000_1100_00rr_iiii" => op!(ADD::RI(rq![r], u4![i])),
            "0000_1010_1000_rrqq" => op!(ADD::RQ(rq![r], rq![q])),
            "0000_1100_01rr_iiii" => op!(ADC::RI(rq![r], u4![i])),
            "0000_1010_1001_rrqq" => op!(ADC::RQ(rq![r], rq![q])),
            "0000_1010_1010_rrqq" => Opcode::TODO(format!("SUB {} {}", rq(r), rq(q))),
            "0000_1011_01rr_iiii" => Opcode::TODO(format!("SBC {} 0x{:02X}", rq(r), i)),
            "0000_1010_1011_rrqq" => Opcode::TODO(format!("SBC {} {}", rq(r), rq(q))),
            "0000_1100_10rr_iiii" => op!(AND::RI(rq![r], u4![i])),
            "0000_1010_1100_rrqq" => op!(AND::RQ(rq![r], rq![q])),
            "0000_1100_11rr_iiii" => op!(OR::RI(rq![r], u4![i])),
            "0000_1010_1101_rrqq" => op!(OR::RQ(rq![r], rq![q])),
            "0000_1101_00rr_iiii" => op!(XOR::RI(rq![r], u4![i])),
            "0000_1010_1110_rrqq" => op!(XOR::RQ(rq![r], rq![q])),
            "0000_1101_11rr_iiii" => op!(CP::RI(rq![r], u4![i])),
            "0000_1111_0000_rrqq" => op!(CP::RQ(rq![r], rq![q])),
            "0000_1101_10rr_iiii" => op!(FAN::RI(rq![r], u4![i])),
            "0000_1111_0001_rrqq" => op!(FAN::RQ(rq![r], rq![q])),
            "0000_1010_1111_rr??" => op!(RLC(rq![r])),
            "0000_1110_1000_11rr" => op!(RRC(rq![r])),
            "0000_1111_0110_nnnn" => op!(INC_Mn(u4![n])),
            "0000_1111_0111_nnnn" => op!(DEC_Mn(u4![n])),
            "0000_1111_0010_10rr" => op!(ACP::X(rq![r])),
            "0000_1111_0010_11rr" => op!(ACP::Y(rq![r])),
            "0000_1111_0011_10rr" => Opcode::TODO(format!("SCPX MX {}", rq(r))),
            "0000_1111_0011_11rr" => Opcode::TODO(format!("SCPY MY {}", rq(r))),
            "0000_1101_00rr_1111" => Opcode::TODO(format!("NOT {}", rq(r))),
            _ => Opcode::UNKNOWN,
        }
    }

    pub fn cycles(&self) -> u32 {
        match self {
              Self::RETS
            | Self::RETD(_)
                => 12,
              Self::CALL(_)
            | Self::CALZ(_)
            | Self::RET
            | Self::NOP7
            // | Self::ADC(_)
            | Self::SET(_)
            | Self::RST(_)
            // | Self::SCF
            // | Self::RCF
            // | Self::SZF
            // | Self::RZF
            // | Self::SDF
            // | Self::RDF
            // | Self::EI
            // | Self::DI
            // | Self::SUB
            // | Self::SBC
            // | Self::XOR
            // | Self::RLC
            // | Self::DEC
            // | Self::ACPX
            // | Self::ACPY
            // | Self::SCPX
            // | Self::SCPY
            // | Self::NOT
                => 7,
            Self::Op(op) => op.cycles(),
            _ => 5,
        }
    }
}
