use ansi_term::Colour;
use rustchi_core::interpreter::Interpreter;

pub trait Printer {
    fn print(&self, val: String);
    fn println(&self, val: String) {
        self.print(format!("{}\n", val))
    }
}

pub struct Terminal<T> {
    pub printer: T,
}

impl<T> Terminal<T> {
    pub fn new(printer: T) -> Self {
        Self { printer }
    }
}

impl<T> Terminal<T> where T: Printer {
    pub fn run(&self, interpreter: &mut Interpreter) {
        self.print_disassembler(&interpreter);
        for _ in 0..8 {
            interpreter.step();
            self.print_disassembler(&interpreter);    
        }
    }
    
    fn print_disassembler(&self, interpreter: &Interpreter) -> () {
        let style = Colour::Black.on(Colour::White);
    
        self.printer.println(format!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"));
    
        let pos = interpreter.pc() - 4;
        // let pos = 0;
        for (address, line) in interpreter.disassemble(pos).take(24) {
            if address == interpreter.pc() {
                self.printer.println(format!("┃{:40}┃", style.paint(format!("{:40}", line))));
            } else {
                self.printer.println(format!("┃{:40}┃", line));
            }
        }
    
        self.printer.println(format!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"));
    }
}
