extern crate wasm_bindgen;
extern crate xterm_js_sys;
extern crate console_error_panic_hook;

use web_sys::console;
use web_sys::{Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{FFI, Terminal};
use rustchi_core::input::Button;

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
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub async fn load(rom_url: &str) -> Self {
        let bytes = fetch_url(rom_url).await;

        let interpreter = Interpreter::load(bytes);

        Self {
            terminal: Terminal::new(BrowserFFI::new(), interpreter),
        }
    }

    #[wasm_bindgen]
    pub fn run_frame(&mut self) {
        self.terminal.run_frame()
    }

    #[wasm_bindgen]
    pub fn press_button(&mut self, button: &str) {

        if let Some(button) = match button {
            "A" => Some(Button::A),
            "B" => Some(Button::B),
            "C" => Some(Button::C),
            _ => None,
        } {
            self.terminal.press_button(button)
        }
    }

    #[wasm_bindgen]
    pub fn release_button(&mut self, button: &str) {

        if let Some(button) = match button {
            "A" => Some(Button::A),
            "B" => Some(Button::B),
            "C" => Some(Button::C),
            _ => None,
        } {
            self.terminal.release_button(button)
        }
    }
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
