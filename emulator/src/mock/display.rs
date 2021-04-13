use crate::{Display, DisplayBits};

pub struct MockDisplay {
    vram: [[u16; 92]; 64],
}

impl MockDisplay {
    pub fn new() -> Self {
        Self {
            vram: [[0; 92]; 64],
        }
    }
}

impl Display for MockDisplay {
    fn write_vram(&mut self, x: u32, y: u32, color: u16) {
        println!("[VRAM] C({}, {}) = {}", x, y, color);
        self.vram[y as usize][x as usize] = color;
    }
    fn write_vram_u8(&mut self, x: u32, y: u32, part: DisplayBits, color: u8) {
        let prev_value = self.vram[y as usize][x as usize];
        let n_value = match part {
            DisplayBits::HighBits => (prev_value & 0x00ff) | (((color as u16) << 8) & 0xff00),
            DisplayBits::LowBits => (prev_value & 0xff00) | ((color as u16) & 0x00ff),
        };
        self.write_vram(x, y, n_value);
    }

    fn clear_vram(&mut self) {
        self.vram = [[0; 92]; 64];
    }

    fn display_vram(&mut self) {}
}
