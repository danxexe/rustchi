extern crate wasm_bindgen;
extern crate xterm_js_sys;
extern crate console_error_panic_hook;

use std::panic;
use web_sys::console;
use web_sys::{Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
// use xterm_js_sys::crossterm_support::XtermJsCrosstermBackend;

use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{FFI, Terminal};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn ansi_up(s: &str);
}

#[allow(dead_code)]
fn console_log<T>(data: T) where T: Into<JsValue> {
    console::log_1(&data.into())
}

struct BrowserFFI;
impl BrowserFFI {
    fn new() -> Self {
        Self
    }
}
impl FFI for BrowserFFI {
    fn print(&self, val: &str) {
        ansi_up(val)
    }
}

#[wasm_bindgen]
pub struct Emulator {
    terminal: Terminal<BrowserFFI>,
    // backend: XtermJsCrosstermBackend<'static>,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub async fn load(rom_url: &str) -> Self {
        let bytes = fetch_url(rom_url).await;

        let interpreter = Interpreter::load(bytes);

        Self {
            terminal: Terminal::new(BrowserFFI::new(), interpreter),
            // backend: xterm.dyn_into().unwrap(),
        }
    }

    #[wasm_bindgen]
    pub fn run_frame(&mut self) {
        self.terminal.run_frame()
    }
}

#[wasm_bindgen]
pub async fn run(rom_url: &str) -> () {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let bytes = fetch_url(rom_url).await;

    let interpreter = Interpreter::load(bytes);
    Terminal::new(BrowserFFI::new(), interpreter).run();
}

async fn fetch_url(url: &str) -> Vec<u8> {
    let request = Request::new_with_str(url).unwrap();
    let window = web_sys::window().unwrap();
    let response_future = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let response: Response = response_future.dyn_into().unwrap();
    let buffer = JsFuture::from(response.array_buffer().unwrap()).await.unwrap();
    let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&buffer);
    let mut body = vec![0; typebuf.length() as usize];
    typebuf.copy_to(&mut body[..]);

    body
}
