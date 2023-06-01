use ansi_term::{Colour, Style};
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
    width: usize,
    rows: Vec<String>,
}
impl<'a> Panel {
    pub fn new(width: usize) -> Self {
        Self {width, rows: vec![]}
    }

    pub fn push_top(&mut self) {
        self.push(&format!("┏{}┓", "━".repeat(self.width - 2)));
    }

    pub fn push_bottom(&mut self) {
        self.push(&format!("┗{}┛", "━".repeat(self.width - 2)));
    }

    pub fn push_with_border(&mut self, value: &str) {
        self.push(&format!("┃{:0w$}┃", value, w = self.width - 2));
    }

    pub fn push_with_border_and_highlight(&mut self, style: Style, value: &str) {
        let value = style.paint(format!("{:0w$}", value, w = self.width - 2)).to_string();
        self.push(&format!("┃{}┃", value));
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
        let mut panel = Panel::new(self.width + b.width);
        let empty_a = " ".repeat(self.width);
        let empty_b = " ".repeat(b.width);
        for ab in self.rows.iter().zip_longest(b.rows.iter()) {
            let ab = ab.or(&empty_a, &empty_b);
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
        let memory = self.print_memory(&interpreter);
        disassembler.zip(registers).zip(memory)
    }

    fn print_registers(&self, interpreter: &Interpreter) -> Panel {
        let reg = interpreter.state.registers;
        let mut panel = Panel::new(12);

        panel.push_top();
        panel.push_with_border(&format!(" PCS 0x{:02X}", reg.PCS));
        panel.push_with_border(&format!(" PCP  {:#X}", reg.PCP));
        panel.push_with_border(&format!(" PCB  {}", reg.PCB));
        panel.push_with_border(&format!(" NPP  {:#X}", reg.NPP));
        panel.push_with_border(&format!(" NBP  {}", reg.NBP));
        panel.push_with_border(&format!(" SP  0x{:02X}", reg.SP));
        panel.push_with_border(&format!(" X  {}", reg.X));
        panel.push_with_border(&format!(" Y  {}", reg.Y));
        panel.push_with_border(&format!(" RP   {:#X}", reg.RP));
        panel.push_with_border(&format!(" A    {:#X}", reg.A));
        panel.push_with_border(&format!(" B    {:#X}", reg.B));
        panel.push_bottom();

        panel
    }
    
    fn print_disassembler(&self, interpreter: &Interpreter) -> Panel {
        let mut panel = Panel::new(32);

        panel.push_top();
    
        let pos = interpreter.pc() - 10;
        for (address, line) in interpreter.disassemble(pos).take(24) {
            match (address, interpreter.pc(), interpreter.prev_state.as_ref().map(|s| s.pc())) {
                (a, b, _) if a == b =>
                    panel.push_with_border_and_highlight(Colour::Fixed(255).on(Colour::Fixed(242)), &line),
                (a, _, Option::Some(c)) if a == c =>
                    panel.push_with_border_and_highlight(Colour::Black.on(Colour::Fixed(255)), &line),
                _ =>
                    panel.push_with_border(&line),
            }
        }
    
        panel.push_bottom();

        panel
    }

    fn print_memory(&self, interpreter: &Interpreter) -> Panel {
        let width = 32;
        let mut panel = Panel::new(width + 8);

        let prev_opcode = interpreter.prev_state.as_ref().map(|s|
            interpreter.rom[s.pc()]
        );

        panel.push_top();
        interpreter.state.memory.slice(0..4096).chunks(width).take(24).enumerate().for_each(|(i, chunk)| {
            match prev_opcode {
                _ =>
                    panel.push_with_border(&format!("{:#05X} {}", i * width, &chunk.iter().join(""))),
            }
        });
        panel.push_bottom();
        panel
    }
}
