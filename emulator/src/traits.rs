use sh::asm::Instruction;

use crate::{DisplayBits, Key, Modifier};

/// A generic Trait that allows for different
/// Input-Methods to be used with the emulator
pub trait Input {
    fn get_key(&mut self) -> (Key, Modifier);
}

/// A generic Trait that allows for different "Frontend"
/// to be used with the emulator
pub trait Display {
    /// Resets all the Pixels in the VRAM to white
    fn clear_vram(&mut self);
    /// Sets the given Pixel (x, y) to the given Color
    fn write_vram(&mut self, x: u32, y: u32, color: u16);
    /// Writes only half of a pixel to the VRAM, the `part` describes
    /// which half is written to
    fn write_vram_u8(&mut self, x: u32, y: u32, part: DisplayBits, color: u8);
    /// Actually draws the current VRAM to screen
    fn display_vram(&mut self);
}

/// A simple Debugger trait that abstracts the debug information
/// to allow for more general handling of debug related stuff
pub trait Debugger {
    /// Prints the given Instruction as debug information
    fn print_instr(&self, address: u32, instr: &Instruction);
}
