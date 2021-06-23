use crate::traits::Debugger;

mod input;
pub use input::CLIInput;

pub struct CLIDebugger {}

impl CLIDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for CLIDebugger {
    fn print(&self, content: &str) {
        println!("{}", content);
    }
}
