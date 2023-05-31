use ansi_term::Colour;
use rustchi_core::interpreter::Interpreter;

pub trait Printer {
    fn print(&self, val: &str);
    fn println(&self, val: &str) {
        self.print(val);
        self.print("\n");
    }
}

pub struct Panel {
    rows: Vec<String>
}
impl<'a> Panel {
    pub fn new() -> Self {
        Self {rows: vec![]}
    }

    pub fn push(&mut self, value: &str) {
        self.rows.push(value.to_owned());
    }

    pub fn print(&self, printer: &impl Printer) {
        for row in &self.rows {
            printer.println(row);
        }
    }

    pub fn zip(a: Panel, b: Panel) -> Panel {
        let mut panel = Panel::new();
        for (a, b) in a.rows.iter().zip(b.rows.iter()) {
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
        for _ in 0..8 {
            interpreter.step();
            self.print_panels(&interpreter).print(&self.printer);
        }
    }

    fn print_panels(&self, interpreter: &Interpreter) -> Panel {
        let disassembler = self.print_disassembler(&interpreter);
        let registers = self.print_registers(&interpreter);
        Panel::zip(disassembler, registers)
    }

    fn print_registers(&self, interpreter: &Interpreter) -> Panel {
        let mut panel = Panel::new();
        let reg = interpreter.state.registers;
        let width = 10;

        panel.push(&format!("{}{}{}", BOX_TL, BOX_H.repeat(width), BOX_TR));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" PCS 0x{:02X}", reg.PCS), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" PCP  {}", reg.PCP), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" PCB  {}", reg.PCB), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" NPP  {}", reg.NPP), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" NBP  {}", reg.NBP), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" SP  0x{:02X}", reg.SP), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" X    {}", reg.X), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" Y    {}", reg.Y), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" RP   {}", reg.RP), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" A    {}", reg.A), BOX_V, w = width));
        panel.push(&format!("{}{:0w$}{}", BOX_V, format!(" B    {}", reg.B), BOX_V, w = width));
        panel.push(&format!("{}{}{}", BOX_BL, BOX_H.repeat(width), BOX_BR));

        for _ in 0..13 {
            panel.push(&" ".repeat(width));
        }

        panel
    }
    
    fn print_disassembler(&self, interpreter: &Interpreter) -> Panel {
        let mut panel = Panel::new();
        let style = Colour::Black.on(Colour::White);

        panel.push(&format!("{}{}{}", BOX_TL, BOX_H.repeat(40), BOX_TR));
    
        let pos = interpreter.pc() - 4;
        // let pos = 0;
        for (address, line) in interpreter.disassemble(pos).take(24) {
            if address == interpreter.pc() {
                panel.push(&format!("{}{:40}{}", BOX_V, style.paint(format!("{:40}", line)), BOX_V));
            } else {
                panel.push(&format!("{}{:40}{}", BOX_V, line, BOX_V));
            }
        }
    
        panel.push(&format!("{}{}{}", BOX_BL, BOX_H.repeat(40), BOX_BR));

        panel
    }
}
