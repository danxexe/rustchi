#![allow(non_camel_case_types)]

use bitmatch::bitmatch;
use std::fmt;

use crate::immediate::*;
use crate::primitive::*;
use crate::registers::Reg;

#[derive(Clone)]
pub enum Opcode {
    PSET(u1, u4),
    JP(S),
    JP_C(S),
    JP_NC(S),
    JP_Z(S),
    JP_NZ(S),
    JP_BA,
    CALL(S),
    CALZ(S),
    RET,
    RETS,
    RETD(L),
    NOP5,
    NOP7,
    HALT,
    INC(Reg),
    LD(Reg, Source),
    RST(u4),
    AND(Reg, Source),
    ADD(Reg, Source),
    TODO(String),
    UNKNOWN,
}

#[bitmatch]
fn rq(r: u16) -> String {
    #[bitmatch]
    match r {
        "00" => format!("A"),
        "01" => format!("B"),
        "10" => format!("MX"),
        "11" => format!("MY"),
    }
}

impl Opcode {    
    #[bitmatch]
    pub fn decode(instruction: u16) -> Opcode {
        #[bitmatch]
        match instruction {
            "0000_1110_010p_qqqq" => Opcode::PSET(p.into(), q.into()),
            "0000_0000_ssss_ssss" => Opcode::JP(s.into()),
            "0000_0010_ssss_ssss" => Opcode::JP_C(s.into()),
            "0000_0011_ssss_ssss" => Opcode::JP_NC(s.into()),
            "0000_0110_ssss_ssss" => Opcode::JP_Z(s.into()),
            "0000_0111_ssss_ssss" => Opcode::JP_NZ(s.into()),
            "0000_1111_1110_1000" => Opcode::JP_BA,
            "0000_0100_ssss_ssss" => Opcode::CALL(s.into()),
            "0000_0101_ssss_ssss" => Opcode::CALZ(s.into()),
            "0000_1111_1101_1111" => Opcode::RET,
            "0000_1111_1101_1110" => Opcode::RETS,
            "0000_0001_llll_llll" => Opcode::RETD(l.into()),
            "0000_1111_1111_1011" => Opcode::NOP5,
            "0000_1111_1111_1111" => Opcode::NOP7,
            "0000_1111_1111_1000" => Opcode::HALT,
            "0000_1110_1110_0000" => Opcode::INC(Reg::X),
            "0000_1110_1111_0000" => Opcode::INC(Reg::Y),
            "0000_1011_xxxx_xxxx" => Opcode::LD(Reg::X, Source::L(x.into())),
            "0000_1000_yyyy_yyyy" => Opcode::LD(Reg::Y, Source::L(y.into())),
            "0000_1110_1000_00rr" => Opcode::LD(Reg::XP, Source::Reg(r.into())),
            "0000_1110_1000_01rr" => Opcode::TODO(format!("LD XH {}", rq(r))),
            "0000_1110_1000_10rr" => Opcode::TODO(format!("LD XL {}", rq(r))),
            "0000_1110_1001_00rr" => Opcode::TODO(format!("LD YP {}", rq(r))),
            "0000_1110_1001_01rr" => Opcode::TODO(format!("LD YH {}", rq(r))),
            "0000_1110_1001_10rr" => Opcode::TODO(format!("LD YL {}", rq(r))),
            "0000_1110_1010_00rr" => Opcode::TODO(format!("LD {} XP", rq(r))),
            "0000_1110_1010_01rr" => Opcode::TODO(format!("LD {} XH", rq(r))),
            "0000_1110_1010_10rr" => Opcode::TODO(format!("LD {} XL", rq(r))),
            "0000_1110_1011_00rr" => Opcode::TODO(format!("LD {} YP", rq(r))),
            "0000_1110_1011_01rr" => Opcode::TODO(format!("LD {} YH", rq(r))),
            "0000_1110_1011_10rr" => Opcode::TODO(format!("LD {} YL", rq(r))),
            "0000_1010_0000_iiii" => Opcode::TODO(format!("ADC XH 0x{:01X}", i)),
            "0000_1010_0001_iiii" => Opcode::TODO(format!("ADC XL 0x{:01X}", i)),
            "0000_1010_0010_iiii" => Opcode::TODO(format!("ADC YH 0x{:01X}", i)),
            "0000_1010_0011_iiii" => Opcode::TODO(format!("ADC YL 0x{:01X}", i)),
            "0000_1010_0100_iiii" => Opcode::TODO(format!("CP XH 0x{:01X}", i)),
            "0000_1010_0101_iiii" => Opcode::TODO(format!("CP XL 0x{:01X}", i)),
            "0000_1010_0110_iiii" => Opcode::TODO(format!("CP YH 0x{:01X}", i)),
            "0000_1010_0111_iiii" => Opcode::TODO(format!("CP YL 0x{:01X}", i)),
            "0000_1110_00rr_iiii" => Opcode::LD(Reg::from(r), Source::U4(i.into())),
            "0000_1110_1100_rrqq" => Opcode::LD(r.into(), Source::Reg(q.into())),
            "0000_1111_1010_nnnn" => Opcode::TODO(format!("LD A MN 0x{:01X}", n)),
            "0000_1111_1011_nnnn" => Opcode::TODO(format!("LD B MN 0x{:01X}", n)),
            "0000_1111_1000_nnnn" => Opcode::TODO(format!("LD MN A 0x{:01X}", n)),
            "0000_1111_1001_nnnn" => Opcode::TODO(format!("LD MN B 0x{:01X}", n)),
            "0000_1110_0110_iiii" => Opcode::TODO(format!("LDPX MX 0x{:01X}", i)),
            "0000_1110_1110_rrqq" => Opcode::TODO(format!("LDPX {} {}", rq(r), rq(q))),
            "0000_1110_0111_iiii" => Opcode::TODO(format!("LDPY MY 0x{:01X}", i)),
            "0000_1110_1111_rrqq" => Opcode::TODO(format!("LDPY {} {}", rq(r), rq(q))),
            "0000_1001_llll_llll" => Opcode::TODO(format!("LBPX 0x{:02X}", l)),
            "0000_1111_0100_iiii" => Opcode::TODO(format!("SET 0x{:01X}", i)),
            "0000_1111_0101_iiii" => Opcode::RST(i.into()),
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
            "0000_1111_1100_00rr" => Opcode::TODO(format!("PUSH {}", rq(r))),
            "0000_1111_1100_0100" => Opcode::TODO(format!("PUSH XP")),
            "0000_1111_1100_0101" => Opcode::TODO(format!("PUSH XH")),
            "0000_1111_1100_0110" => Opcode::TODO(format!("PUSH XL")),
            "0000_1111_1100_0111" => Opcode::TODO(format!("PUSH YP")),
            "0000_1111_1100_1000" => Opcode::TODO(format!("PUSH YH")),
            "0000_1111_1100_1001" => Opcode::TODO(format!("PUSH YL")),
            "0000_1111_1100_1010" => Opcode::TODO(format!("PUSH F")),
            "0000_1111_1101_00rr" => Opcode::TODO(format!("POP {}", rq(r))),
            "0000_1111_1101_0100" => Opcode::TODO(format!("POP XP")),
            "0000_1111_1101_0101" => Opcode::TODO(format!("POP XH")),
            "0000_1111_1101_0110" => Opcode::TODO(format!("POP XL")),
            "0000_1111_1101_0111" => Opcode::TODO(format!("POP YP")),
            "0000_1111_1101_1000" => Opcode::TODO(format!("POP YH")),
            "0000_1111_1101_1001" => Opcode::TODO(format!("POP YL")),
            "0000_1111_1101_1010" => Opcode::TODO(format!("POP F")),
            "0000_1111_1110_00rr" => Opcode::LD(Reg::SPH, Source::Reg(r.into())),
            "0000_1111_1111_00rr" => Opcode::LD(Reg::SPL, Source::Reg(r.into())),
            "0000_1111_1110_01rr" => Opcode::TODO(format!("LD {} SPH", rq(r))),
            "0000_1111_1111_01rr" => Opcode::TODO(format!("LD {} SPL", rq(r))),
            "0000_1100_00rr_iiii" => Opcode::ADD(r.into(), Source::U4(i.into())),
            "0000_1010_1000_rrqq" => Opcode::TODO(format!("ADD {} {}", rq(r), rq(q))),
            "0000_1100_01rr_iiii" => Opcode::TODO(format!("ADC {} 0x{:02X}", rq(r), i)),
            "0000_1010_1001_rrqq" => Opcode::TODO(format!("ADC {} {}", rq(r), rq(q))),
            "0000_1010_1010_rrqq" => Opcode::TODO(format!("SUB {} {}", rq(r), rq(q))),
            "0000_1011_01rr_iiii" => Opcode::TODO(format!("SBC {} 0x{:02X}", rq(r), i)),
            "0000_1010_1011_rrqq" => Opcode::TODO(format!("SBC {} {}", rq(r), rq(q))),
            "0000_1100_10rr_iiii" => Opcode::AND(r.into(), Source::U4(i.into())),
            "0000_1010_1100_rrqq" => Opcode::TODO(format!("AND {} {}", rq(r), rq(q))),
            "0000_1100_11rr_iiii" => Opcode::TODO(format!("OR {} 0x{:02X}", rq(r), i)),
            "0000_1010_1101_rrqq" => Opcode::TODO(format!("OR {} {}", rq(r), rq(q))),
            "0000_1101_00rr_iiii" => Opcode::TODO(format!("XOR {} 0x{:02X}", rq(r), i)),
            "0000_1010_1110_rrqq" => Opcode::TODO(format!("XOR {} {}", rq(r), rq(q))),
            "0000_1101_11rr_iiii" => Opcode::TODO(format!("CP {} 0x{:02X}", rq(r), i)),
            "0000_1111_0000_rrqq" => Opcode::TODO(format!("CP {} {}", rq(r), rq(q))),
            "0000_1101_10rr_iiii" => Opcode::TODO(format!("FAN {} 0x{:02X}", rq(r), i)),
            "0000_1111_0001_rrqq" => Opcode::TODO(format!("FAN {} {}", rq(r), rq(q))),
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
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PSET(p, q) => write!(f, "PSET {} {:#X}", p, q),
            Self::JP(s) => write!(f, "JP {}", s),
            Self::JP_C(s) => write!(f, "JP C {}", s),
            Self::JP_NC(s) => write!(f, "JP NC {}", s),
            Self::JP_Z(s) => write!(f, "JP Z {}", s),
            Self::JP_NZ(s) => write!(f, "JP NZ {}", s),
            Self::JP_BA => write!(f, "JP BA"),
            Self::CALL(s) => write!(f, "CALL {}", s),
            Self::CALZ(s) => write!(f, "CALZ {}", s),
            Self::RET => write!(f, "RET"),
            Self::RETS => write!(f, "RETS"),
            Self::RETD(l) => write!(f, "RETD {}", l),
            Self::NOP5 => write!(f, "NOP5"),
            Self::NOP7 => write!(f, "NOP7"),
            Self::HALT => write!(f, "HALT"),
            Self::INC(r) => write!(f, "INC {}", r),
            Self::LD(r, l) => write!(f, "LD {} {}", r, l),
            Self::RST(i) => write!(f, "RST {}", i),
            Self::AND(r, i) => write!(f, "AND {} {}", r, i),
            Self::ADD(r, i) => write!(f, "ADD {} {}", r, i),
            Self::TODO(s) => write!(f, "{} #TODO", s),
            Self::UNKNOWN => write!(f, "??"),
        }
    }
}
