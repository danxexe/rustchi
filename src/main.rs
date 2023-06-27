use std::fs;
use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{FFI, Terminal};

use crossterm::{event, QueueableCommand, cursor, style, terminal, ExecutableCommand};

use std::io::{Write, stdout};

struct ConsoleFFI;

impl ConsoleFFI {
    fn new() -> Self {
        Self
    }
}

impl FFI for ConsoleFFI {
    fn print(&self, val: &str) {
        print!("{}", val);
    }

    fn println(&self, val: &str) {
        print!("{}\r", val);
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_panic(info);
    }));

    println!("Loading rom...");

    let bytes = fs::read("www/rom.bin").unwrap();
    let interpreter = Interpreter::load(bytes);

    println!("Loaded {} bytes.\n", interpreter.rom.len());

    let mut gui = Terminal::new(ConsoleFFI::new(), interpreter);

    let mut stdout = stdout();

    stdout
        .queue(cursor::Hide)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::Print("[q]uit"))?;

    loop {
        stdout.queue(cursor::MoveTo(0, 1))?;

        gui.run_frame();

        stdout.flush()?;

        if let Ok(true) = event::poll(std::time::Duration::from_secs(0)) {
            let event = event::read()?;

            if event == event::Event::Key(event::KeyCode::Char('q').into()) {
                break;
            }
        }

        if gui.time_since_frame_start() < gui.target_fps() {
            let diff = gui.target_fps() - gui.time_since_frame_start();
            if !diff.is_negative() {
                std::thread::sleep(diff.to_std().unwrap());
            }
        }
    }

    restore_terminal();
    Ok(())
}

fn restore_terminal() {
    _ = terminal::disable_raw_mode();
    _ = stdout().execute(cursor::Show);
    println!();
}
