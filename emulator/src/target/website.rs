use crate::{traits::Debugger, wasm::log, Input, Key, Modifier};

use sh::asm::Instruction;

mod display;
pub use display::WasmDisplay;

mod input;
pub use input::WasmInput;

pub struct WasmDebugger {}

impl WasmDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for WasmDebugger {
    fn print(&self, content: &str) {
        unsafe {
            log(content);
        }
    }
}
