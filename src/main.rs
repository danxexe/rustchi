use std::{fs, usize};
use std::path::Path;

struct CPU {
    // pub a: u8,
    // pub status: u8,
    pub pc: u16,
 }

 impl CPU {
    pub fn new() -> Self {
        Self {
            // a: 0,
            // status: 0,
            pc: 0x0100,
        }
    }

    pub fn pc_usize(&self) -> usize {
        self.pc.try_into().unwrap()
    }
 }

 struct Interpreter {
    pub cpu: CPU,
    pub rom: Vec<u8>,
 }

 impl Interpreter {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        Self {
            cpu: CPU::new(),
            rom: fs::read(path).unwrap(),
        }
    }

    pub fn words(&self) -> impl Iterator<Item = u16> + '_ {
        self.rom.chunks_exact(2).map(|bytes|
            u16::from_be_bytes([bytes[0], bytes[1]])
        )
    }

    pub fn disassemble(&self, offset: usize) -> impl Iterator<Item = (usize, String)> + '_ {
        self.words().skip(offset).enumerate().map(move |(i, word)| {
            let address = offset + i;
            (address, format!("0x{address:04X} {word:04X} {}", decode(word)))
        })
    }
 }

use bitmatch::bitmatch;

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

#[bitmatch]
fn decode(instruction: u16) -> String {
    #[bitmatch]
    match instruction {
        "0000_1110_010p_pppp" => format!("PSET 0x{:02X}", p),
        "0000_0000_ssss_ssss" => format!("JP 0x{:02X}", s),
        "0000_0010_ssss_ssss" => format!("JP C 0x{:02X}", s),
        "0000_0011_ssss_ssss" => format!("JP NC 0x{:02X}", s),
        "0000_0110_ssss_ssss" => format!("JP Z 0x{:02X}", s),
        "0000_0111_ssss_ssss" => format!("JP NZ 0x{:02X}", s),
        "0000_1111_1110_1000" => format!("JPBA"),
        "0000_0100_ssss_ssss" => format!("CALL 0x{:02X}", s),
        "0000_0101_ssss_ssss" => format!("CALZ 0x{:02X}", s),
        "0000_1111_1101_1111" => format!("RET"),
        "0000_1111_1101_1110" => format!("RETS"),
        "0000_0001_llll_llll" => format!("RETD 0x{:02X}", l),
        "0000_1111_1111_1011" => format!("NOP5"),
        "0000_1111_1111_1111" => format!("NOP7"),
        "0000_1111_1111_1000" => format!("HALT"),
        "0000_1110_1110_0000" => format!("INC X"),
        "0000_1110_1111_0000" => format!("INC Y"),
        "0000_1011_xxxx_xxxx" => format!("LD X 0x{:02X}", x),
        "0000_1000_yyyy_yyyy" => format!("LD Y 0x{:02X}", y),
        "0000_1110_1000_00rr" => format!("LD XP {}", rq(r)),
        "0000_1110_1000_01rr" => format!("LD XH {}", rq(r)),
        "0000_1110_1000_10rr" => format!("LD XL {}", rq(r)),
        "0000_1110_1001_00rr" => format!("LD YP {}", rq(r)),
        "0000_1110_1001_01rr" => format!("LD YH {}", rq(r)),
        "0000_1110_1001_10rr" => format!("LD YL {}", rq(r)),
        "0000_1110_1010_00rr" => format!("LD {} XP", rq(r)),
        "0000_1110_1010_01rr" => format!("LD {} XH", rq(r)),
        "0000_1110_1010_10rr" => format!("LD {} XL", rq(r)),
        "0000_1110_1011_00rr" => format!("LD {} YP", rq(r)),
        "0000_1110_1011_01rr" => format!("LD {} YH", rq(r)),
        "0000_1110_1011_10rr" => format!("LD {} YL", rq(r)),
        "0000_1010_0000_iiii" => format!("ADC XH 0x{:01X}", i),
        "0000_1010_0001_iiii" => format!("ADC XL 0x{:01X}", i),
        "0000_1010_0010_iiii" => format!("ADC YH 0x{:01X}", i),
        "0000_1010_0011_iiii" => format!("ADC YL 0x{:01X}", i),
        "0000_1010_0100_iiii" => format!("CP XH 0x{:01X}", i),
        "0000_1010_0101_iiii" => format!("CP XL 0x{:01X}", i),
        "0000_1010_0110_iiii" => format!("CP YH 0x{:01X}", i),
        "0000_1010_0111_iiii" => format!("CP YL 0x{:01X}", i),
        "0000_1110_00rr_iiii" => format!("LD {} 0x{:01X}", rq(r), i),
        "0000_1110_1100_rrqq" => format!("LD {} {}", rq(r), rq(q)),
        "0000_1111_1010_nnnn" => format!("LD A MN 0x{:01X}", n),
        "0000_1111_1011_nnnn" => format!("LD B MN 0x{:01X}", n),
        "0000_1111_1000_nnnn" => format!("LD MN A 0x{:01X}", n),
        "0000_1111_1001_nnnn" => format!("LD MN B 0x{:01X}", n),
        "0000_1110_0110_iiii" => format!("LDPX MX 0x{:01X}", i),
        "0000_1110_1110_rrqq" => format!("LDPX {} {}", rq(r), rq(q)),
        "0000_1110_0111_iiii" => format!("LDPY MY 0x{:01X}", i),
        "0000_1110_1111_rrqq" => format!("LDPY {} {}", rq(r), rq(q)),
        "0000_1001_llll_llll" => format!("LBPX 0x{:02X}", l),
        "0000_1111_0100_iiii" => format!("SET 0x{:01X}", i),
        "0000_1111_0101_iiii" => format!("RST 0x{:01X}", i),
        "0000_1111_0100_0001" => format!("SCF"),
        "0000_1111_0101_1110" => format!("RCF"),
        "0000_1111_0100_0010" => format!("SZF"),
        "0000_1111_0101_1101" => format!("RZF"),
        "0000_1111_0100_0100" => format!("SDF"),
        "0000_1111_0101_1011" => format!("RDF"),
        "0000_1111_0100_1000" => format!("EI"),
        "0000_1111_0101_0111" => format!("DI"),
        "0000_1111_1101_1011" => format!("INC SP"),
        "0000_1111_1100_1011" => format!("DEC SP"),
        "0000_1111_1100_00rr" => format!("PUSH {}", rq(r)),
        "0000_1111_1100_0100" => format!("PUSH XP"),
        "0000_1111_1100_0101" => format!("PUSH XH"),
        "0000_1111_1100_0110" => format!("PUSH XL"),
        "0000_1111_1100_0111" => format!("PUSH YP"),
        "0000_1111_1100_1000" => format!("PUSH YH"),
        "0000_1111_1100_1001" => format!("PUSH YL"),
        "0000_1111_1100_1010" => format!("PUSH F"),
        "0000_1111_1101_00rr" => format!("POP {}", rq(r)),
        "0000_1111_1101_0100" => format!("POP XP"),
        "0000_1111_1101_0101" => format!("POP XH"),
        "0000_1111_1101_0110" => format!("POP XL"),
        "0000_1111_1101_0111" => format!("POP YP"),
        "0000_1111_1101_1000" => format!("POP YH"),
        "0000_1111_1101_1001" => format!("POP YL"),
        "0000_1111_1101_1010" => format!("POP F"),
        "0000_1111_1110_00rr" => format!("LD SPH {}", rq(r)),
        "0000_1111_1111_00rr" => format!("LD SPL {}", rq(r)),
        "0000_1111_1110_01rr" => format!("LD {} SPH", rq(r)),
        "0000_1111_1111_01rr" => format!("LD {} SPL", rq(r)),
        "0000_1100_00rr_iiii" => format!("ADD {} 0x{:02X}", rq(r), i),
        "0000_1010_1000_rrqq" => format!("ADD {} {}", rq(r), rq(q)),
        "0000_1100_01rr_iiii" => format!("ADC {} 0x{:02X}", rq(r), i),
        "0000_1010_1001_rrqq" => format!("ADC {} {}", rq(r), rq(q)),
        "0000_1010_1010_rrqq" => format!("SUB {} {}", rq(r), rq(q)),
        "0000_1011_01rr_iiii" => format!("SBC {} 0x{:02X}", rq(r), i),
        "0000_1010_1011_rrqq" => format!("SBC {} {}", rq(r), rq(q)),
        "0000_1100_10rr_iiii" => format!("AND {} 0x{:02X}", rq(r), i),
        "0000_1010_1100_rrqq" => format!("AND {} {}", rq(r), rq(q)),
        "0000_1100_11rr_iiii" => format!("OR {} 0x{:02X}", rq(r), i),
        "0000_1010_1101_rrqq" => format!("OR {} {}", rq(r), rq(q)),
        "0000_1101_00rr_iiii" => format!("XOR {} 0x{:02X}", rq(r), i),
        "0000_1010_1110_rrqq" => format!("XOR {} {}", rq(r), rq(q)),
        "0000_1101_11rr_iiii" => format!("CP {} 0x{:02X}", rq(r), i),
        "0000_1111_0000_rrqq" => format!("CP {} {}", rq(r), rq(q)),
        "0000_1101_10rr_iiii" => format!("FAN {} 0x{:02X}", rq(r), i),
        "0000_1111_0001_rrqq" => format!("FAN {} {}", rq(r), rq(q)),
        "0000_1010_1111_rrbb" => format!("RLC {} {}", rq(r), rq(b)),
        "0000_1110_1000_11rr" => format!("RRC {}", rq(r)),
        "0000_1111_0110_nnnn" => format!("INC MN 0x{:01X}", n),
        "0000_1111_0111_nnnn" => format!("DEC MN 0x{:01X}", n),
        "0000_1111_0010_10rr" => format!("ACPX MX {}", rq(r)),
        "0000_1111_0010_11rr" => format!("ACPY MY {}", rq(r)),
        "0000_1111_0011_10rr" => format!("SCPX MX {}", rq(r)),
        "0000_1111_0011_11rr" => format!("SCPY MY {}", rq(r)),
        "0000_1101_00rr_1111" => format!("NOT {}", rq(r)),
        _ => format!("??"),
    }
}

use ansi_term::Colour;

fn main() {
    println!("Loading rom...");

    let interpreter = Interpreter::load("rom.bin");

    println!("Loaded {} bytes.\n", interpreter.rom.len());

    let style = Colour::Black.on(Colour::White);

    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");

    for (address, line) in interpreter.disassemble(interpreter.cpu.pc_usize() - 4).take(16) {
        if address == interpreter.cpu.pc_usize() {
            println!("┃{:40}┃", style.paint(format!("{:40}", line)));
        } else {
            println!("┃{:40}┃", line);
        }
    }

    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}
