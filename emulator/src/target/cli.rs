use std::io::{stdin, stdout, Write};

use sh::asm::Instruction;

use crate::{traits::Debugger, Input, Key, Modifier};

pub struct CLIInput {}

impl CLIInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for CLIInput {
    fn get_key(&mut self) -> (Key, Modifier) {
        let mut entered = String::new();
        stdout().write(&[b'#']).expect("Writing to Stdout");
        stdout().flush().expect("Flushing StdOut");
        stdin()
            .read_line(&mut entered)
            .expect("Could not get string");

        let key = entered.chars().next().unwrap();

        if key.is_digit(10) {
            let digit = key.to_digit(10).unwrap();
            (Key::Number(digit as u8), Modifier::None)
        } else if key.is_ascii() {
            let filtered = entered.replace("\n", "").to_lowercase();

            match filtered.as_str() {
                "exe" => (Key::Exe, Modifier::None),
                "menu" => (Key::Menu, Modifier::None),
                _ => panic!("Unknown Input"),
            }
        } else {
            println!("Unknown: {:?}", key);
            (Key::Exe, Modifier::None)
        }
    }
}

pub struct CLIDebugger {}

impl CLIDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for CLIDebugger {
    fn print_instr(&self, addr: u32, instr: &Instruction) {
        println!("[x{:04X}] {:?}", addr, instr);
    }
}
