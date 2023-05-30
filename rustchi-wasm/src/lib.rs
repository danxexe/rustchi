extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use std::panic;
use web_sys::console;
use web_sys::{Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use rustchi_core::interpreter::Interpreter;
use rustchi_terminal::{Printer, Terminal};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn ansi_up(s: &str);
}

#[allow(dead_code)]
fn console_log<T>(data: T) where T: Into<JsValue> {
    console::log_1(&data.into())
}

struct AnsiUpPrinter;
impl AnsiUpPrinter {
    fn new() -> Self {
        Self
    }
}
impl Printer for AnsiUpPrinter {
    fn print(&self, val: String) {
        ansi_up(&val)
    }
}

struct ConsolePrinter;
#[allow(dead_code)]
impl ConsolePrinter {
    fn new() -> Self {
        Self
    }
}
impl Printer for ConsolePrinter {
    fn print(&self, val: String) {
        console_log(val)
    }
}

#[wasm_bindgen]
pub async fn run() -> () {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let bytes = fetch_url("http://localhost:8000/rom.bin").await;

    let mut interpreter = Interpreter::load(bytes);
    Terminal::new(AnsiUpPrinter::new()).run(&mut interpreter)
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
