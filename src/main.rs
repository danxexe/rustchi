use std::{fs, usize};
use std::path::Path;

mod opcode;
mod immediate;
mod registers;
mod state;

use opcode::*;
use state::*;

 struct Interpreter {
    pub state: State,
    pub rom: Vec<u8>,
 }

 impl Interpreter {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        Self {
            state: State::new(),
            rom: fs::read(path).unwrap(),
        }
    }

    pub fn pc(&self) -> usize {
        self.state.pc.try_into().unwrap()
    }

    pub fn words(&self) -> impl Iterator<Item = u16> + '_ {
        self.rom.chunks_exact(2).map(|bytes|
            u16::from_be_bytes([bytes[0], bytes[1]])
        )
    }

    pub fn disassemble(&self, offset: usize) -> impl Iterator<Item = (usize, String)> + '_ {
        self.words().skip(offset).enumerate().map(move |(i, word)| {
            let address = offset + i;
            (address, format!("0x{address:04X} {word:04X} {}", Opcode::decode(word)))
        })
    }

    pub fn step(&mut self) -> () {
        let opcode = Opcode::decode(self.words().skip(self.pc()).take(1).last().unwrap());
        println!("Executing: {}", opcode);
        let _new_state = self.exec(opcode);
    }

    fn exec(&mut self, opcode: Opcode) -> () {
        self.state = match opcode {
            Opcode::PSET(p) => State {
                np: p.into(),
                pc: self.state.pc + 1,
                ..self.state
            },
            Opcode::JP(s) => State {
                pc: u16::from(s) | (u16::from(self.state.np) << 8),
                ..self.state
            },
            Opcode::LD(reg, i) => State {
                pc: self.state.pc + 1,
                registers: self.state.registers.load(reg, i),
                ..self.state
            },
            Opcode::RST(i) => State {
                pc: self.state.pc + 1,
                flags: Flags::from_bits(i.into()).unwrap(),
                ..self.state
            },
            _ => State {..self.state},
        };
    }
 }

fn print_disassembler(interpreter: &Interpreter) -> () {
    let style = Colour::Black.on(Colour::White);

    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");

    let pos = interpreter.pc() - 4;
    // let pos = 0;
    for (address, line) in interpreter.disassemble(pos).take(24) {
        if address == interpreter.pc() {
            println!("┃{:40}┃", style.paint(format!("{:40}", line)));
        } else {
            println!("┃{:40}┃", line);
        }
    }

    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}

use ansi_term::Colour;

fn main() {
    println!("Loading rom...");

    let mut interpreter = Interpreter::load("rom.bin");

    println!("Loaded {} bytes.\n", interpreter.rom.len());

    print_disassembler(&interpreter);
    for _ in 0..8 {
        interpreter.step();
        print_disassembler(&interpreter);    
    }
}
