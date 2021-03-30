use crate::util;

use std::convert::TryInto;

const HEADER_IDENTIFIER: [u8; 14] = [
    0xAA, 0xAC, 0xBD, 0xAF, 0x90, 0x88, 0x9A, 0x8D, 0xD3, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF,
];

mod error;
pub use error::ParseError;
pub mod eactivity;
pub mod image;
pub mod localization;

#[derive(Debug)]
pub struct File {
    internal_name: String,
    short_name: String,
    file_name: String,
    file_size: u32,
    pub selected_image: image::Image,
    pub unselected_image: image::Image,
    pub executable_code: Vec<u8>,
    pub localized: localization::Localized,
    pub eactivity: eactivity::EActivity,
}

// References
// https://prizm.cemetech.net/index.php/G3A_File_Format
// https://www.omnimaga.org/casio-calculator-programming-news-and-support/casio-prizm-already-for-sale/msg157437/#msg157437
impl File {
    pub fn parse(content: &[u8]) -> Result<File, ParseError> {
        let identifier = &content[0..14];
        if identifier != &HEADER_IDENTIFIER {
            return Err(ParseError::WrongIdentifier);
        }

        if content[0x000f] != 0xfe {
            return Err(ParseError::WrongFormat);
        }

        let raw_file_size = [
            content[0x0010] ^ 0xff,
            content[0x0011] ^ 0xff,
            content[0x0012] ^ 0xff,
            content[0x0013] ^ 0xff,
        ];

        let file_size = u32::from_be_bytes(raw_file_size);

        let raw_checksum = &content[0x0020..0x0024];
        let checksum = u32::from_be_bytes(raw_checksum.try_into().unwrap());

        if &content[0x0024..0x0026] != &[0x01, 0x01] {
            return Err(ParseError::WrongFormat);
        }

        let raw_executable_size = &content[0x002E..0x0032];
        let executable_size = u32::from_be_bytes(raw_executable_size.try_into().unwrap());

        let raw_short_name = &content[0x0040..0x005c];
        let short_name = match String::from_utf8(raw_short_name.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ParseError::WrongFormat),
        };

        let raw_internal_name = &content[0x0060..0x006b];
        let internal_name = match String::from_utf8(raw_internal_name.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ParseError::WrongFormat),
        };

        let raw_file_name = &content[0x0ebc..0x1000];
        let file_name = match String::from_utf8(raw_file_name.to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(ParseError::WrongFormat),
        };

        let raw_icon_unselected = &content[0x1000..=0x3dff];
        let icon_unselected = image::Image::parse(raw_icon_unselected);

        let raw_icon_selected = &content[0x4000..=0x6dff];
        let icon_selected = image::Image::parse(raw_icon_selected);

        let executable_end: usize = 0x7000 + executable_size as usize;
        let raw_executable_section = &content[0x7000..executable_end];

        let raw_checksum_copy = &content[executable_end..executable_end + 4];
        let checksum_copy = u32::from_be_bytes(raw_checksum_copy.try_into().unwrap());

        if checksum != checksum_copy {
            return Err(ParseError::MismatchedChecksums);
        }

        let localized = localization::Localized::parse(content).unwrap();
        let parsed_eactivity = eactivity::EActivity::parse(content).unwrap();

        Ok(File {
            internal_name,
            short_name,
            file_name,
            file_size,
            selected_image: icon_selected,
            unselected_image: icon_unselected,
            executable_code: raw_executable_section.to_vec(),
            localized,
            eactivity: parsed_eactivity,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = vec![0; 0x7000];

        &result[0..0x000e].copy_from_slice(&HEADER_IDENTIFIER);

        // **
        // General Header stuff
        // **
        let file_size = self.file_size.to_be_bytes();
        // 0x000E
        result[0x000e] = (file_size[3] ^ 0xff) - 0x41;
        // 0x000F
        result[0x000f] = 0xfe;
        // 0x0010
        result[0x0010] = file_size[0] ^ 0xff;
        result[0x0011] = file_size[1] ^ 0xff;
        result[0x0012] = file_size[2] ^ 0xff;
        result[0x0013] = file_size[3] ^ 0xff;
        // 0x0014
        result[0x0014] = (file_size[3] ^ 0xff).wrapping_sub(0xb8);
        // 0x0016
        // TODO
        &result[0x0016..0x0016 + 4].copy_from_slice(&(0 as u32).to_be_bytes());
        // 0x0024
        result[0x0024] = 0x01;
        result[0x0025] = 0x01;
        // 0x002e
        &result[0x002e..0x002e + 4]
            .copy_from_slice(&(self.executable_code.len() as u32).to_be_bytes());
        // 0x0040
        util::write_string(&mut result[0x0040..0x005c], &self.short_name);
        // 0x005c
        &result[0x005c..0x005c + 4].copy_from_slice(&self.file_size.to_be_bytes());
        // 0x0060
        util::write_string(&mut result[0x0060..0x006b], &self.internal_name);

        // **
        // Unselected image
        // **
        &result[0x1000..=0x3dff].copy_from_slice(&self.unselected_image.serialize());

        // **
        // Selected image
        // **
        &result[0x4000..=0x6dff].copy_from_slice(&self.selected_image.serialize());

        // **
        // Localization Stuff
        // **
        self.localized.serialize(&mut result[0x006b..0x0170]);

        // **
        // EActivity
        // **
        &result[0x0170..0x0590].copy_from_slice(&self.eactivity.serialize());

        util::write_string(&mut result[0x0ebc..0x1000], &self.file_name);

        // **
        // Code Block
        // **
        result.extend_from_slice(&self.executable_code);

        // Checksum at the end
        let checksum = util::checksum(&result);
        let chechsum_bytes = checksum.to_be_bytes();
        result[0x0020..0x0024].copy_from_slice(&chechsum_bytes);
        result.extend_from_slice(&chechsum_bytes);

        result
    }
}
