use ansi_term::{Colour, Style};
use itertools::Itertools;
use rustchi_core::{interpreter::Interpreter, change::{Change, Register, Memory}};
use rustchi_core::primitive::u4;

const STEPS: usize = 26;

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
        self.push_raw(&format!("┏{}┓", "━".repeat(self.width - 2)));
    }

    pub fn push_bottom(&mut self) {
        self.push_raw(&format!("┗{}┛", "━".repeat(self.width - 2)));
    }

    pub fn push(&mut self, value: &str) {
        self.push_raw(&format!("┃{:0w$}┃", value, w = self.width - 2));
    }

    pub fn push_with_style(&mut self, value: &str, style: Style) {
        let value = format!("{:0w$}", value, w = self.width - 2);
        let value = style.paint(value).to_string();
        self.push_raw(&format!("┃{}┃", value));
    }

    pub fn push_raw(&mut self, value: &str) {
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
            panel.push_raw(&format!("{}{}", a, b));
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

macro_rules! style {
    ($changes:expr, $pattern:pat, $on:expr, $off:expr) => {
        if $changes.iter().any(|c| match c { $pattern => true, _ => false } ) {$on} else {$off}
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
        let changes = &interpreter.changes;
        let mut panel = Panel::new(12);

        let on = Colour::Fixed(255).on(Colour::Fixed(242));
        let off = Style::new();

        panel.push_top();
        panel.push_with_style(&format!(" PCS 0x{:02X}", reg.PCS), style!(changes, Change::Register(Register::PCS(_)), on, off));
        panel.push_with_style(&format!(" PCP  {:#X}", reg.PCP), style!(changes, Change::Register(Register::PCP(_)), on, off));
        panel.push_with_style(&format!(" PCB  {}", reg.PCB), style!(changes, Change::Register(Register::PCB(_)), on, off));
        panel.push_with_style(&format!(" NPP  {:#X}", reg.NPP), style!(changes, Change::Register(Register::NPP(_)), on, off));
        panel.push_with_style(&format!(" NBP  {}", reg.NBP), style!(changes, Change::Register(Register::NBP(_)), on, off));
        panel.push_with_style(&format!(" SP  0x{:02X}", reg.SP), style!(changes, Change::Register(Register::SP(_)), on, off));
        panel.push_with_style(&format!(" X  {}", reg.X), style!(changes, Change::Register(Register::X(_)), on, off));
        panel.push_with_style(&format!(" Y  {}", reg.Y), style!(changes, Change::Register(Register::Y(_)), on, off));
        panel.push_with_style(&format!(" RP   {:#X}", reg.RP), style!(changes, Change::Register(Register::RP(_)), on, off));
        panel.push_with_style(&format!(" A    {:#X}", reg.A), style!(changes, Change::Register(Register::A(_)), on, off));
        panel.push_with_style(&format!(" B    {:#X}", reg.B), style!(changes, Change::Register(Register::B(_)), on, off));
        panel.push(&format!(" F    {:#X}", interpreter.state.flags));
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
                    panel.push_with_style(&line, Colour::Black.on(Colour::Fixed(255))),
                (a, _, Option::Some(c)) if a == c =>
                    panel.push_with_style(&line, Colour::Fixed(255).on(Colour::Fixed(242))),
                _ =>
                    panel.push(&line),
            }
        }
    
        panel.push_bottom();

        panel
    }

    fn print_memory(&self, interpreter: &Interpreter) -> Panel {
        let width = 32;
        let mut panel = Panel::new(width + 8);

        let mut changes = interpreter.changes.iter().filter_map(|c|
            match c {
                Change::Memory(Memory{address, value: _} ) => Option::Some(address),
                _ => Option::None,
            }
        );
        let start = match changes.next() {
            Option::Some(address) => usize::from(*address).saturating_sub(64 * 4),
            Option::None => 0,
        } >> 8 << 8;

        panel.push_top();
        interpreter.state.memory.slice(start..4096).iter().enumerate().chunks(width).into_iter().take(24).for_each(|chunk| {
            let bytes: Vec<(usize, u4)> = chunk.map(|(i, v)| (i, *v) ).collect();
            let mut values = bytes.iter().cloned().map(|(_, v)| v);
            let (start_address, _) = bytes.iter().next().unwrap();
            panel.push(&format!("{:#05X} {}", start_address + start, values.join("")))
        });
        panel.push_bottom();
        panel
    }
}
