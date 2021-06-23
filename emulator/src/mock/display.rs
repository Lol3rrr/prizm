use crate::{Display, Memory};

pub struct MockDisplay {}

impl MockDisplay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for MockDisplay {
    fn display_vram(&mut self, _mem: &mut Memory) {}
}
