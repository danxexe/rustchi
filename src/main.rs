use std::fs;
use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{FFI, Terminal};

struct ConsoleFFI;

impl ConsoleFFI {
    fn new() -> Self {
        Self
    }
}

impl FFI for ConsoleFFI {
    fn print(&self, val: &str) {
        print!("{}", val)
    }
}

fn main() {
    println!("Loading rom...");

    let bytes = fs::read("www/rom.bin").unwrap();
    let interpreter = Interpreter::load(bytes);

    println!("Loaded {} bytes.\n", interpreter.rom.len());

    Terminal::new(ConsoleFFI::new(), interpreter).run()
}
