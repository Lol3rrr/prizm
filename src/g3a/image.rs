use image::{io::Reader as ImageReader, DynamicImage, ImageBuffer, Rgb};

mod pixel;
pub use pixel::Pixel;

#[derive(Debug)]
pub struct Image {
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn parse(raw: &[u8]) -> Self {
        let mut rows = Vec::with_capacity(64);
        for y in 0..64 {
            let mut current_row = Vec::with_capacity(92);
            for x in 0..92 {
                let index = (y * 92 + x) * 2;
                let data = [raw[index], raw[index + 1]];

                current_row.push(Pixel::parse(&data));
            }

            rows.push(current_row);
        }

        Self { pixels: rows }
    }

    pub fn serialize(&self) -> [u8; 64 * 92 * 2] {
        let mut result = [0; 64 * 92 * 2];

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, pix) in row.iter().enumerate() {
                let first = (y * 92 + x) * 2;
                let second = first + 1;

                let pix_data = pix.serialize();
                result[first] = pix_data[0];
                result[second] = pix_data[1];
            }
        }

        result
    }

    pub fn from_file(path: &str) -> Option<Self> {
        let raw_img = match ImageReader::open(path) {
            Ok(s) => match s.decode() {
                Ok(i) => i,
                Err(_) => return None,
            },
            Err(_) => return None,
        };

        let img = match raw_img {
            DynamicImage::ImageRgb8(i) => i,
            _ => return None,
        };

        if img.width() != 92 || img.height() != 64 {
            return None;
        }

        let mut result_rows = Vec::with_capacity(64);
        for y in 0..64 {
            let mut current_row = Vec::with_capacity(92);
            for x in 0..92 {
                let pixel = img.get_pixel(x, y);

                current_row.push(Pixel::new(pixel[0], pixel[1], pixel[2]));
            }

            result_rows.push(current_row);
        }

        Some(Self {
            pixels: result_rows,
        })
    }

    pub fn save_to_file(&self, path: &str) {
        let width = 92;
        let height = 64;

        let mut imgbuf = ImageBuffer::new(width, height);

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, pix) in row.iter().enumerate() {
                *(imgbuf.get_pixel_mut(x as u32, y as u32)) = Rgb([pix.red, pix.green, pix.blue]);
            }
        }

        imgbuf.save(path).unwrap();
    }
}
