use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::{Display, Memory};

const DISPLAY_WIDTH: usize = 384;
const DISPLAY_HEIGHT: usize = 216;

pub struct WasmDisplay {
    context: web_sys::CanvasRenderingContext2d,
}

impl WasmDisplay {
    pub fn new(canvas_id: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self { context }
    }

    fn bytes_to_color(first: u8, second: u8) -> String {
        let r: u8 = first & 0b11111000;
        let g: u8 = ((first & 0b00000111) << 5) | ((second & 0b11100000) >> 3);
        let b: u8 = (second & 0b00011111) << 3;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl Display for WasmDisplay {
    fn display_vram(&mut self, memory: &mut Memory) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let index = (y * DISPLAY_WIDTH + x) * 2;
                let first_part = memory.get_vram(index);
                let second_part = memory.get_vram(index + 1);

                let color = Self::bytes_to_color(first_part, second_part);
                self.context.set_fill_style(&JsValue::from_str(&color));

                self.context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_color_1() {
        let color = 0xFFFFu16.to_be_bytes();
        assert_eq!(
            "#F8FCF8".to_string(),
            WasmDisplay::bytes_to_color(color[0], color[1])
        );
    }
}
