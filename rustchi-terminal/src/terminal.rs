use rustchi_core::{interpreter::Interpreter, change::{Change, Register, Memory}};
use rustchi_core::primitive::{u1, u4};

use ansi_term::{Colour, Style};
use clap::Parser;
use itertools::Itertools;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    breakpoint: Option<u32>,

    #[arg(short, long)]
    short: bool,

    #[arg(short, long)]
    debugger: bool,

    #[arg(short, long)]
    lcd: bool,
}

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
    args: Cli,
    pub printer: T,
}

impl<T> Terminal<T> {
    pub fn new(printer: T) -> Self {
        Self {
            args: Cli::parse(),
            printer,
        }
    }
}

macro_rules! style {
    ($changes:expr, $pattern:pat, $on:expr, $off:expr) => {
        if $changes.iter().any(|c| match c { $pattern => true, _ => false } ) {$on} else {$off}
    }
}

impl<T> Terminal<T> where T: Printer {
    fn print_panels(&self, interpreter: &Interpreter) {

        if self.args.short {
            let opcode = interpreter.next_opcode();
            println!("{:#06X} {}", interpreter.state.pc(), opcode);
            return;
        }

        if self.args.lcd {
            self.print_screen (&interpreter).print(&self.printer);
            return;
        }

        if !self.args.debugger {
            return;
        }

        let screen = self.print_screen (&interpreter);
        let disassembler = self.print_disassembler(&interpreter);
        let registers = self.print_registers(&interpreter);
        let memory = self.print_memory(&interpreter);
        screen.zip(disassembler).zip(registers).zip(memory).print(&self.printer);
    }

    fn print_screen(&self, interpreter: &Interpreter) -> Panel {
        let lcd = interpreter.state.memory.lcd.borrow();

        let mut panel = Panel::new(34);
        let on = Colour::Fixed(255);
        let off = Colour::Fixed(239);
        let both_off = off.on(off);
        let top_on = on.on(off);
        let bottom_on = off.on(on);
        let both_on = on.on(on);

        panel.push_top();
        panel.push(&(off.paint("     󰩰      󰛨      󰡓           ").to_string()));
        panel.push("");
        for y in (0..16).step_by(2)  {
            let top = lcd[y].iter().take(32);
            let bottom = lcd[y+1].iter().take(32);
            let row = top.zip(bottom).map(|(a, b)|
                match (*a, *b) {
                    (u1::OFF, u1::OFF) => both_off.paint("▀"),
                    (u1::ON, u1::OFF) => top_on.paint("▀"),
                    (u1::OFF, u1::ON) => bottom_on.paint("▀"),
                    (u1::ON, u1::ON) => both_on.paint("▀"),
                    _ => panic!(),
                }.to_string()
            ).join("");

            panel.push(row.as_str());
        }
        panel.push("");
        panel.push(&(off.paint("     󰇥      󰓅      󰮯           ").to_string()));
        panel.push_bottom();
        panel
    }

    fn print_registers(&self, interpreter: &Interpreter) -> Panel {
        let reg = interpreter.state.registers;
        let changes = &interpreter.changes;
        let mut panel = Panel::new(12);

        let on = Colour::Fixed(255).on(Colour::Fixed(242));
        let off = Style::new();

        panel.push_top();
        panel.push(&format!("{:9}", interpreter.state.tick));
        panel.push(&format!("{:9}", interpreter.state.cycles));
        panel.push(&format!("{}{:─<w$}{}", "╶", "", "╴", w = panel.width - 4));
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
        panel.push_with_style(&format!(" F    {:#X}", interpreter.state.flags), style!(changes, Change::Flags(_), on, off));
        panel.push_bottom();

        panel
    }

    fn print_disassembler(&self, interpreter: &Interpreter) -> Panel {
        let mut panel = Panel::new(32);

        panel.push_top();

        let pos = interpreter.pc().saturating_sub(10);
        for (address, line) in interpreter.disassemble(pos).take(24) {
            match (address, interpreter.pc(), interpreter.prev_pc) {
                (a, b, _) if a == b =>
                    panel.push_with_style(&line, Colour::Fixed(255).on(Colour::Fixed(242))),
                (a, _, Option::Some(c)) if a == c =>
                    panel.push_with_style(&line, Colour::Black.on(Colour::Fixed(255))),
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

        let changes: Vec<usize> = interpreter.changes.iter().cloned().filter_map(|c|
            match c {
                Change::Memory(Memory{address, value: _} ) => Option::Some(usize::from(address)),
                _ => Option::None,
            }
        ).collect();

        let start = match changes.first() {
            Option::Some(address) => address.saturating_sub(64 * 4),
            Option::None => 0,
        } >> 8 << 8;

        panel.push_top();
        interpreter.state.memory.slice(start..4096).iter().enumerate().chunks(width).into_iter().take(24).for_each(|chunk| {
            let bytes: Vec<(usize, u4)> = chunk.map(|(i, v)| (i, *v)).collect();
            let mut values = bytes.iter().cloned().map(|(i, v)| {
                let addr = start + i;
                let is_change = changes.contains(&addr);

                let style = match (addr, is_change) {
                    (_, true) => Colour::Black.on(Colour::Fixed(255)),
                    (addr, _) if addr >= 0xF00 => Colour::Cyan.on(Colour::Black),
                    _ => Style::new(),
                };

                style.paint(format!("{}", v)).to_string()
            });
            let (start_address, _) = bytes.iter().next().unwrap();
            panel.push(&format!("{:#05X} {}", start_address + start, values.join("")))
        });
        panel.push_bottom();
        panel
    }

    pub fn run(&self, interpreter: &mut Interpreter) {
        self.print_panels(&interpreter);
        loop {
            interpreter.step();
            self.print_panels(&interpreter);

            // Limit execution until we have a proper loop
            if interpreter.state.tick == 6500 {
                panic!("stop!");
            }

            if self.args.breakpoint.is_some() && interpreter.state.tick == self.args.breakpoint.unwrap() {
                panic!("stop!");
            }
        }
    }
}
