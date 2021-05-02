use sh::asm::Instruction;

use crate::traits::Debugger;

pub struct EmptyDebugger {}

impl EmptyDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for EmptyDebugger {
    fn print_instr(&self, addr: u32, instr: &Instruction) {}
}
