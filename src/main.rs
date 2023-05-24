use std::fs;
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

    pub fn words<'a>(&'a self) -> impl Iterator<Item = u16> + 'a {
        self.rom.chunks_exact(2).map(|bytes|
            u16::from_be_bytes([bytes[0], bytes[1]])
        )
    }

    pub fn program<'a>(&'a self) -> impl Iterator<Item = u16> + 'a {
        let current_word: usize = self.cpu.pc.try_into().unwrap();
        self.words().skip(current_word)
    }

    // pub fn disassemble() -> () {
    // }
 }

 use bitmatch::bitmatch;
#[bitmatch]
fn decode(instruction: u16) -> String {
    // let instruction = u16::from_be_bytes([i1, i2]);
    #[bitmatch]
    match instruction {
        "0000_0000_ssss_ssss" => format!("JP {:#04X}", s),
        "0000_1111_1010_mmmm" => format!("LD A {:#03X}", m),
        "0000_1110_00rr_iiii" => format!("LD R {:#03X} {:#03X}", r, i),
        "0000_1111_0101_iiii" => format!("RST F {:#03X}", i),
        // "0000_1111_1010_0010" => format!("First u16!"),
        // "0100_aaii" => format!("Val {}, {}", a, i),
        // "01??_????" => format!("Invalid"),
        // "10ii_aabb" => format!("Ld {}, {}, {}", a, b, i),
        // "11ii_aabb" => format!("St {}, {}, {}", a, b, i),
        _ => "unknown".into(),
    }
}


fn main() {
    println!("Loading rom...");

    let interpreter = Interpreter::load("rom.bin");

    println!("Loaded {} bytes.", interpreter.rom.len());

    for (i, word) in interpreter.program().enumerate().take(32) {
        let address: u16 = interpreter.cpu.pc + u16::try_from(i).unwrap();
        println!("{:#04X} {:04X} {}", address, word, decode(word));
    }
}
