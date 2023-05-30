use std::fs;
use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{Printer, Terminal};

struct StdoutPrinter;

impl StdoutPrinter {
    fn new() -> Self {
        Self
    }
}

impl Printer for StdoutPrinter {
    fn print(&self, val: String) {
        print!("{}", val)
    }
}

fn main() {
    println!("Loading rom...");

    let bytes = fs::read("www/rom.bin").unwrap();
    let mut interpreter = Interpreter::load(bytes);

    println!("Loaded {} bytes.\n", interpreter.rom.len());

    Terminal::new(StdoutPrinter::new()).run(&mut interpreter)
}
