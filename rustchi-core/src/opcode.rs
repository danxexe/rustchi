#![allow(non_camel_case_types)]

mod exec;
mod adc;
mod add;
mod and;
mod cp;
mod fan;
mod ident;
mod inc;
mod jp;
mod ld;
mod ldpx;
mod push;
mod pop;
mod rq;

pub use {
    self::
    exec::Exec,
    adc::*,
    add::*,
    and::*,
    cp::*,
    fan::*,
    ident::*,
    inc::*,
    jp::*,
    ld::*,
    ldpx::*,
    push::*,
    pop::*,
    rq::*,
};

use crate::{
    immediate::*,
    primitive::*,
    registers::Reg,
};

use bitmatch::bitmatch;
use std::fmt;

#[derive(Clone)]
pub enum Opcode {
    PSET(u1, u4),
    JP(JP),
    CALL(S),
    CALZ(S),
    RET,
    RETS,
    RETD(u8),
    NOP5,
    NOP7,
    HALT,
    INC(INC),
    PUSH(PUSH),
    POP(POP),
    LD(Reg, Source),
    LDv2(LD),
    LDPX(LDPX),
    LBPX(u8),
    SET(u4),
    RST(u4),
    ADC(ADC),
    AND(AND),
    ADD(ADD),
    CP(CP),
    FAN(FAN),
    TODO(String),
    UNKNOWN,
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::*;
        match self {
            PSET(p, q) => write!(f, "PSET {} {:#X}", p, q),
            JP(s) => write!(f, "{}", s),
            CALL(s) => write!(f, "CALL {}", s),
            CALZ(s) => write!(f, "CALZ {}", s),
            RET => write!(f, "RET"),
            RETS => write!(f, "RETS"),
            RETD(l) => write!(f, "RETD {:#04X}", l),
            NOP5 => write!(f, "NOP5"),
            NOP7 => write!(f, "NOP7"),
            HALT => write!(f, "HALT"),
            INC(op) => write!(f, "{}", op),
            PUSH(p) => write!(f, "{}", p),
            POP(p) => write!(f, "{}", p),
            LD(r, l) => write!(f, "LD {} {}", r, l),
            LDv2(op) => write!(f, "{}", op),
            LDPX(op) => write!(f, "{}", op),
            LBPX(l) => write!(f, "LBPX {:#04X}", l),
            SET(i) => write!(f, "SET F {:#X}", i),
            RST(i) => write!(f, "RST F {:#X}", i),
            ADC(op) => write!(f, "{}", op),
            AND(op) => write!(f, "{}", op),
            ADD(op) => write!(f, "{}", op),
            CP(op) => write!(f, "{}", op),
            FAN(op) => write!(f, "{}", op),
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
            "0000_1110_010p_qqqq" => Opcode::PSET(p.try_into().unwrap(), q.try_into().unwrap()),
            "0000_0000_ssss_ssss" => Opcode::JP(JP::S(u8![s])),
            "0000_0010_ssss_ssss" => Opcode::JP(JP::C(u8![s])),
            "0000_0011_ssss_ssss" => Opcode::JP(JP::NC(u8![s])),
            "0000_0110_ssss_ssss" => Opcode::JP(JP::Z(u8![s])),
            "0000_0111_ssss_ssss" => Opcode::JP(JP::NZ(u8![s])),
            "0000_1111_1110_1000" => Opcode::JP(JP::BA),
            "0000_0100_ssss_ssss" => Opcode::CALL(s.into()),
            "0000_0101_ssss_ssss" => Opcode::CALZ(s.into()),
            "0000_1111_1101_1111" => Opcode::RET,
            "0000_1111_1101_1110" => Opcode::RETS,
            "0000_0001_llll_llll" => Opcode::RETD(u8![l]),
            "0000_1111_1111_1011" => Opcode::NOP5,
            "0000_1111_1111_1111" => Opcode::NOP7,
            "0000_1111_1111_1000" => Opcode::HALT,
            "0000_1110_1110_0000" => Opcode::INC(INC::X),
            "0000_1110_1111_0000" => Opcode::INC(INC::Y),
            "0000_1011_xxxx_xxxx" => Opcode::LD(Reg::X, Source::L(x.into())),
            "0000_1000_yyyy_yyyy" => Opcode::LD(Reg::Y, Source::L(y.into())),
            "0000_1110_1000_00rr" => Opcode::LD(Reg::XP, Source::Reg(r.into())),
            "0000_1110_1000_01rr" => Opcode::TODO(format!("LD XH {}", rq(r))),
            "0000_1110_1000_10rr" => Opcode::TODO(format!("LD XL {}", rq(r))),
            "0000_1110_1001_00rr" => Opcode::LD(Reg::YP, Source::Reg(r.into())),
            "0000_1110_1001_01rr" => Opcode::TODO(format!("LD YH {}", rq(r))),
            "0000_1110_1001_10rr" => Opcode::TODO(format!("LD YL {}", rq(r))),
            "0000_1110_1010_00rr" => Opcode::LDv2(LD::RXP(rq![r])),
            "0000_1110_1010_01rr" => Opcode::LDv2(LD::RXH(rq![r])),
            "0000_1110_1010_10rr" => Opcode::LDv2(LD::RXL(rq![r])),
            "0000_1110_1011_00rr" => Opcode::LDv2(LD::RYP(rq![r])),
            "0000_1110_1011_01rr" => Opcode::LDv2(LD::RYH(rq![r])),
            "0000_1110_1011_10rr" => Opcode::LDv2(LD::RYL(rq![r])),
            "0000_1010_0000_iiii" => Opcode::ADC(ADC::XHi(u4![i])),
            "0000_1010_0001_iiii" => Opcode::ADC(ADC::XLi(u4![i])),
            "0000_1010_0010_iiii" => Opcode::ADC(ADC::YHi(u4![i])),
            "0000_1010_0011_iiii" => Opcode::ADC(ADC::YLi(u4![i])),
            "0000_1010_0100_iiii" => Opcode::CP(CP::XHi(u4![i])),
            "0000_1010_0101_iiii" => Opcode::CP(CP::XLi(u4![i])),
            "0000_1010_0110_iiii" => Opcode::CP(CP::YHi(u4![i])),
            "0000_1010_0111_iiii" => Opcode::CP(CP::YLi(u4![i])),
            "0000_1110_00rr_iiii" => Opcode::LD(Reg::from(r), Source::U4(i.try_into().unwrap())),
            "0000_1110_1100_rrqq" => Opcode::LD(r.into(), Source::Reg(q.into())),
            "0000_1111_1010_nnnn" => Opcode::TODO(format!("LD A MN 0x{:01X}", n)),
            "0000_1111_1011_nnnn" => Opcode::TODO(format!("LD B MN 0x{:01X}", n)),
            "0000_1111_1000_nnnn" => Opcode::TODO(format!("LD MN A 0x{:01X}", n)),
            "0000_1111_1001_nnnn" => Opcode::TODO(format!("LD MN B 0x{:01X}", n)),
            "0000_1110_0110_iiii" => Opcode::LDPX(LDPX::MX(u4![i])),
            "0000_1110_1110_rrqq" => Opcode::LDPX(LDPX::RQ(rq![r], rq![q])),
            "0000_1110_0111_iiii" => Opcode::TODO(format!("LDPY MY 0x{:01X}", i)),
            "0000_1110_1111_rrqq" => Opcode::TODO(format!("LDPY {} {}", rq(r), rq(q))),
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
            "0000_1111_1100_00rr" => Opcode::PUSH(PUSH::R(RQ::from(u4![r]))),
            "0000_1111_1100_0100" => Opcode::PUSH(PUSH::XP),
            "0000_1111_1100_0101" => Opcode::PUSH(PUSH::XH),
            "0000_1111_1100_0110" => Opcode::PUSH(PUSH::XL),
            "0000_1111_1100_0111" => Opcode::PUSH(PUSH::YP),
            "0000_1111_1100_1000" => Opcode::PUSH(PUSH::YH),
            "0000_1111_1100_1001" => Opcode::PUSH(PUSH::YL),
            "0000_1111_1100_1010" => Opcode::PUSH(PUSH::F),
            "0000_1111_1101_00rr" => Opcode::POP(POP::R(RQ::from(u4![r]))),
            "0000_1111_1101_0100" => Opcode::POP(POP::XP),
            "0000_1111_1101_0101" => Opcode::POP(POP::XH),
            "0000_1111_1101_0110" => Opcode::POP(POP::XL),
            "0000_1111_1101_0111" => Opcode::POP(POP::YP),
            "0000_1111_1101_1000" => Opcode::POP(POP::YH),
            "0000_1111_1101_1001" => Opcode::POP(POP::YL),
            "0000_1111_1101_1010" => Opcode::POP(POP::F),
            "0000_1111_1110_00rr" => Opcode::LD(Reg::SPH, Source::Reg(r.into())),
            "0000_1111_1111_00rr" => Opcode::LD(Reg::SPL, Source::Reg(r.into())),
            "0000_1111_1110_01rr" => Opcode::TODO(format!("LD {} SPH", rq(r))),
            "0000_1111_1111_01rr" => Opcode::TODO(format!("LD {} SPL", rq(r))),
            "0000_1100_00rr_iiii" => Opcode::ADD(ADD::RI(rq![r], u4![i])),
            "0000_1010_1000_rrqq" => Opcode::ADD(ADD::RQ(rq![r], rq![q])),
            "0000_1100_01rr_iiii" => Opcode::ADC(ADC::RI(rq![r], u4![i])),
            "0000_1010_1001_rrqq" => Opcode::ADC(ADC::RQ(rq![r], rq![q])),
            "0000_1010_1010_rrqq" => Opcode::TODO(format!("SUB {} {}", rq(r), rq(q))),
            "0000_1011_01rr_iiii" => Opcode::TODO(format!("SBC {} 0x{:02X}", rq(r), i)),
            "0000_1010_1011_rrqq" => Opcode::TODO(format!("SBC {} {}", rq(r), rq(q))),
            "0000_1100_10rr_iiii" => Opcode::AND(AND::RI(rq![r], u4![i])),
            "0000_1010_1100_rrqq" => Opcode::AND(AND::RQ(rq![r], rq![q])),
            "0000_1100_11rr_iiii" => Opcode::TODO(format!("OR {} 0x{:02X}", rq(r), i)),
            "0000_1010_1101_rrqq" => Opcode::TODO(format!("OR {} {}", rq(r), rq(q))),
            "0000_1101_00rr_iiii" => Opcode::TODO(format!("XOR {} 0x{:02X}", rq(r), i)),
            "0000_1010_1110_rrqq" => Opcode::TODO(format!("XOR {} {}", rq(r), rq(q))),
            "0000_1101_11rr_iiii" => Opcode::CP(CP::RI(rq![r], u4![i])),
            "0000_1111_0000_rrqq" => Opcode::CP(CP::RQ(rq![r], rq![q])),
            "0000_1101_10rr_iiii" => Opcode::FAN(FAN::RI(rq![r], u4![i])),
            "0000_1111_0001_rrqq" => Opcode::FAN(FAN::RQ(rq![r], rq![q])),
            "0000_1010_1111_rrbb" => Opcode::TODO(format!("RLC {} {}", rq(r), rq(b))),
            "0000_1110_1000_11rr" => Opcode::TODO(format!("RRC {}", rq(r))),
            "0000_1111_0110_nnnn" => Opcode::TODO(format!("INC MN 0x{:01X}", n)),
            "0000_1111_0111_nnnn" => Opcode::TODO(format!("DEC MN 0x{:01X}", n)),
            "0000_1111_0010_10rr" => Opcode::TODO(format!("ACPX MX {}", rq(r))),
            "0000_1111_0010_11rr" => Opcode::TODO(format!("ACPY MY {}", rq(r))),
            "0000_1111_0011_10rr" => Opcode::TODO(format!("SCPX MX {}", rq(r))),
            "0000_1111_0011_11rr" => Opcode::TODO(format!("SCPY MY {}", rq(r))),
            "0000_1101_00rr_1111" => Opcode::TODO(format!("NOT {}", rq(r))),
            _ => Opcode::UNKNOWN,
        }
    }

    pub fn cycles(&self) -> u32 {
        match self {
            //   self::RETS
            // | self::RETD(_)
            //     => 12,
              Self::CALL(_)
            | Self::CALZ(_)
            | Self::RET
            | Self::NOP7
            // | Self::ADC(_)
            | Self::CP(_)
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
            | Self::ADD(_)
            // | Self::ADC
            // | Self::SUB
            // | Self::SBC
            | Self::AND(_)
            // | Self::OR
            // | Self::XOR
            // | Self::FAN
            // | Self::RLC
            | Self::INC(_)
            // | Self::DEC
            // | Self::ACPX
            // | Self::ACPY
            // | Self::SCPX
            // | Self::SCPY
            // | Self::NOT
                => 7,
            _ => 5,
        }
    }
}
