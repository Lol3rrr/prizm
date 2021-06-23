use crate::traits::Debugger;

pub struct EmptyDebugger {}

impl EmptyDebugger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debugger for EmptyDebugger {
    fn print(&self, _content: &str) {}
}
