use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

use g3a::File;
use wasm_bindgen::prelude::*;

use crate::{
    target::{self, WasmDebugger},
    Emulator, Exception, Key, Modifier,
};

mod executor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct EmulatorWrapper {
    emulator: Emulator<target::WasmInput, target::WasmDisplay>,
    current_fut: Option<Pin<Box<dyn Future<Output = Result<(), Exception>>>>>,
    input_data: Rc<RefCell<Option<(Key, Modifier)>>>,
    executor: executor::Executor,
}

#[wasm_bindgen]
pub fn create_emulator(
    raw_file: Vec<u8>,
    canvas_id: String,
    debug: bool,
) -> Option<EmulatorWrapper> {
    let file = match File::parse(&raw_file) {
        Ok(f) => f,
        Err(_) => {
            return None;
        }
    };

    let (input, input_data) = target::WasmInput::new();
    let display = target::WasmDisplay::new(&canvas_id);
    let mut emulator = Emulator::new(file, input, display);
    if debug {
        emulator.set_debug(Box::new(WasmDebugger::new()));
    }

    Some(EmulatorWrapper {
        emulator,
        current_fut: None,
        input_data,
        executor: executor::Executor::new(),
    })
}

#[wasm_bindgen]
pub fn enable_panic_hook() {
    console_error_panic_hook::set_once();
}

unsafe fn to_static<'a, T>(val: &'a mut T) -> &'static mut T {
    std::mem::transmute(val)
}

#[wasm_bindgen]
impl EmulatorWrapper {
    pub fn tick(&mut self) -> bool {
        let mut fut = match self.current_fut.as_mut() {
            Some(fut) => fut,
            None => {
                let st_self = unsafe { to_static(self) };
                let tmp = Box::pin(st_self.emulator.emulate_single());
                self.current_fut.replace(tmp);

                self.current_fut.as_mut().unwrap()
            }
        };

        match self.executor.poll(&mut fut) {
            Some(res) => {
                self.current_fut = None;

                res.is_ok()
            }
            None => true,
        }
    }

    pub fn get_pc(&self) -> u32 {
        self.emulator.pc()
    }

    pub fn display(&mut self) {
        self.emulator.force_display();
    }

    pub fn press_key(&mut self, id: u8, shift: bool, alpha: bool) {
        let n_key = match id {
            10 => Key::ArrowUp,
            11 => Key::ArrowRight,

            _ => panic!("Unknown Key-ID"),
        };
        let n_modifier = if shift {
            Modifier::Shift
        } else if alpha {
            Modifier::Alpha
        } else {
            Modifier::None
        };

        self.input_data.replace(Some((n_key, n_modifier)));
    }
}
