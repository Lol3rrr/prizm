use std::future::Future;

use sh::asm::Instruction;

use crate::{Key, Memory, Modifier};

/// A generic Trait that allows for different
/// Input-Methods to be used with the emulator
pub trait Input {
    type Fut: Future<Output = (Key, Modifier)>;

    fn get_key(&mut self) -> Self::Fut;
}

/// A generic Trait that allows for different "Frontend"
/// to be used with the emulator
pub trait Display {
    /// Actually draws the current VRAM to screen
    fn display_vram(&mut self, memory: &mut Memory);
}

/// A simple Debugger trait that abstracts the debug information
/// to allow for more general handling of debug related stuff
pub trait Debugger {
    /// Prints the given String to the given Debug
    fn print(&self, content: &str);

    /// Prints the given Instruction to the Debug output
    fn print_instr(&self, address: u32, instr: &Instruction) {
        let content = format!("[{:02X}] {:?}", address, instr);
        self.print(&content);
    }
}
