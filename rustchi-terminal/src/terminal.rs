use ansi_term::Colour;
use itertools::Itertools;
use rustchi_core::interpreter::Interpreter;

const STEPS: usize = 13;

pub trait Printer {
    fn print(&self, val: &str);
    fn println(&self, val: &str) {
        self.print(val);
        self.print("\n");
    }
}

pub struct Panel {
    rows: Vec<String>,
}
impl<'a> Panel {
    pub fn new() -> Self {
        Self {rows: vec![]}
    }

    pub fn push_top(&mut self, width: usize) {
        self.push(&format!("{}{}{}", BOX_TL, BOX_H.repeat(width), BOX_TR));
    }

    pub fn push_bottom(&mut self, width: usize) {
        self.push(&format!("{}{}{}", BOX_BL, BOX_H.repeat(width), BOX_BR));
    }

    pub fn push_with_border(&mut self, width: usize, value: &str) {
        self.push(&format!("{}{:0w$}{}", BOX_V, value, BOX_V, w = width));
    }

    pub fn push(&mut self, value: &str) {
        self.rows.push(value.to_owned());
    }

    pub fn print(&self, printer: &impl Printer) {
        for row in &self.rows {
            printer.println(row);
        }
    }

    pub fn zip(&self, b: Panel) -> Panel {
        let mut panel = Panel::new();
        let empty = &"".to_string();
        for ab in self.rows.iter().zip_longest(b.rows.iter()) {
            let ab = ab.or(empty, empty);
            let (a, b) = ab;
            panel.push(&format!("{}{}", a, b));
        }
        panel
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

const BOX_TL: &str = "┏";
const BOX_TR: &str = "┓";
const BOX_H: &str = "━";
const BOX_V:  &str = "┃";

const BOX_BL: &str = "┗";
const BOX_BR: &str = "┛";

impl<T> Terminal<T> where T: Printer {
    pub fn run(&self, interpreter: &mut Interpreter) {
        self.print_panels(&interpreter).print(&self.printer);
        for _ in 0..STEPS {
            interpreter.step();
            self.print_panels(&interpreter).print(&self.printer);
        }
    }

    fn print_panels(&self, interpreter: &Interpreter) -> Panel {
        let disassembler = self.print_disassembler(&interpreter);
        let registers = self.print_registers(&interpreter);
        // let memory = self.print_memory(&interpreter);
        disassembler.zip(registers)
    }

    fn print_registers(&self, interpreter: &Interpreter) -> Panel {
        let reg = interpreter.state.registers;
        let width = 10;
        let mut panel = Panel::new();

        panel.push_top(width);
        panel.push_with_border(width, &format!("{:0w$}", format!(" PCS 0x{:02X}", reg.PCS), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" PCP  {:#X}", reg.PCP), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" PCB  {}", reg.PCB), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" NPP  {:#X}", reg.NPP), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" NBP  {}", reg.NBP), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" SP  0x{:02X}", reg.SP), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" X  {}", reg.X), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" Y  {}", reg.Y), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" RP   {:#X}", reg.RP), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" A    {:#X}", reg.A), w = width));
        panel.push_with_border(width, &format!("{:0w$}", format!(" B    {:#X}", reg.B), w = width));
        panel.push_bottom(width);

        for _ in 0..14 {
            panel.push(&" ".repeat(width));
        }

        panel
    }
    
    fn print_disassembler(&self, interpreter: &Interpreter) -> Panel {
        let width = 40;
        let mut panel = Panel::new();
        let style = Colour::Black.on(Colour::White);

        panel.push_top(width);
    
        let pos = interpreter.pc() - 4;
        for (address, line) in interpreter.disassemble(pos).take(24) {
            let line = if address == interpreter.pc() {
                style.paint(format!("{:40}", line)).to_string()
            } else {
                line
            };

            panel.push_with_border(width, &line);
        }
    
        panel.push_bottom(width);

        panel
    }

    // fn print_memory(&self, interpreter: &Interpreter) -> Panel {
    //     let width = 64;
    //     let mut panel = Panel::new();
    //     panel.push_top(width);
    //     interpreter.state.memory.slice(0..4096).chunks(64).for_each(|chunk| {
    //         panel.push_with_border(width, &chunk.iter().join(""));
    //     });
    //     panel.push_bottom(width);
    //     panel
    // }
}
