use std::fs;
use rustchi_core::{
    interpreter::Interpreter,
    input::Button,
};
use rustchi_terminal::{FFI, Terminal};

use crossterm::{
    event,
    event::Event,
    event::KeyCode,
    event::KeyEvent,
    event::KeyEventKind,
    QueueableCommand,
    cursor,
    style,
    terminal,
    ExecutableCommand
};

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

    let mut paused = false;

    stdout
        .queue(cursor::Hide)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::Print("[q]uit [p]ause/resume"))?;

    loop {
        stdout.queue(cursor::MoveTo(0, 1))?;

        if !paused {
            gui.run_frame();
        }

        stdout.flush()?;

        if let Ok(true) = event::poll(std::time::Duration::from_secs(0)) {
            let event = event::read()?;

            match event {
                Event::Key(KeyEvent {code: KeyCode::Char('q'), kind: KeyEventKind::Press, ..}) =>
                    break,
                Event::Key(KeyEvent {code: KeyCode::Char('p'), kind: KeyEventKind::Press, ..}) =>
                    paused = !paused,
                Event::Key(KeyEvent {code: KeyCode::Char('a'), kind: KeyEventKind::Press, ..}) =>
                    gui.press_button(Button::A),
                Event::Key(KeyEvent {code: KeyCode::Char('a'), kind: KeyEventKind::Release, ..}) =>
                    gui.release_button(Button::A),
                Event::Key(KeyEvent {code: KeyCode::Char('s'), kind: KeyEventKind::Press, ..}) =>
                    gui.press_button(Button::B),
                Event::Key(KeyEvent {code: KeyCode::Char('s'), kind: KeyEventKind::Release, ..}) =>
                    gui.release_button(Button::B),
                Event::Key(KeyEvent {code: KeyCode::Char('d'), kind: KeyEventKind::Press, ..}) =>
                    gui.press_button(Button::C),
                Event::Key(KeyEvent {code: KeyCode::Char('d'), kind: KeyEventKind::Release, ..}) =>
                    gui.release_button(Button::C),
                _ => (),
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
