use crate::util::write_string;

// These are just some predefined offsets for the
// different Languages
const ENGLISH_OFFSET: usize = 0x6b;
const SPANISH_OFFSET: usize = 0x83;
const GERMAN_OFFSET: usize = 0x9b;
const FRENCH_OFFSET: usize = 0xb3;
const PORTUGUESE_OFFSET: usize = 0xcb;
const CHINESE_OFFSET: usize = 0xe3;
const TEXT_SIZE: usize = 0x18;

const EACTIVITY_OFFSET: usize = 0x12b;
const VERSION_OFFSET: usize = 0x130;
const DATE_OFFSET: usize = 0x13c;

#[derive(Debug)]
pub enum LocalizationError {
    MalformedInput,
}

#[derive(Debug)]
pub struct Localized {
    pub english: String,
    pub spanish: String,
    pub german: String,
    pub french: String,
    pub portuguese: String,
    pub chinese: String,
    pub eactivity: bool,
    pub version: String,
    pub date: String,
}

fn parse_string(input: Vec<u8>) -> Result<String, LocalizationError> {
    match String::from_utf8(input) {
        Ok(s) => Ok(s),
        Err(_) => Err(LocalizationError::MalformedInput),
    }
}

impl Localized {
    pub fn parse(content: &[u8]) -> Result<Self, LocalizationError> {
        let raw_english = &content[ENGLISH_OFFSET..ENGLISH_OFFSET + TEXT_SIZE];
        let raw_spanish = &content[SPANISH_OFFSET..SPANISH_OFFSET + TEXT_SIZE];
        let raw_german = &content[GERMAN_OFFSET..GERMAN_OFFSET + TEXT_SIZE];
        let raw_french = &content[FRENCH_OFFSET..FRENCH_OFFSET + TEXT_SIZE];
        let raw_portuguese = &content[PORTUGUESE_OFFSET..PORTUGUESE_OFFSET + TEXT_SIZE];
        let raw_chinese = &content[CHINESE_OFFSET..CHINESE_OFFSET + TEXT_SIZE];

        let english = parse_string(raw_english.to_vec())?;
        let spanish = parse_string(raw_spanish.to_vec())?;
        let german = parse_string(raw_german.to_vec())?;
        let french = parse_string(raw_french.to_vec())?;
        let portuguese = parse_string(raw_portuguese.to_vec())?;
        let chinese = parse_string(raw_chinese.to_vec())?;

        let raw_eactivity = content[EACTIVITY_OFFSET];
        let eactivity = raw_eactivity != 0;

        let raw_version = &content[VERSION_OFFSET..VERSION_OFFSET + 0xc];
        let raw_date = &content[DATE_OFFSET..DATE_OFFSET + 0xe];

        let version = parse_string(raw_version.to_vec())?;
        let date = parse_string(raw_date.to_vec())?;

        Ok(Self {
            english,
            spanish,
            german,
            french,
            portuguese,
            chinese,
            eactivity,
            version,
            date,
        })
    }

    /// Serializes the Localization related stuff
    pub fn serialize(&self) -> [u8; 0xdf] {
        let mut buf = [0; 0xdf];

        write_string(&mut buf[0x0..0x18], &self.english);
        write_string(&mut buf[0x18..0x30], &self.spanish);
        write_string(&mut buf[0x30..0x48], &self.german);
        write_string(&mut buf[0x48..0x60], &self.french);
        write_string(&mut buf[0x60..0x78], &self.portuguese);
        write_string(&mut buf[0x78..0x90], &self.chinese);

        // Reserved but set to english by default
        write_string(&mut buf[0x90..0xa8], &self.english);
        write_string(&mut buf[0xa8..0xc0], &self.english);

        buf[0xc0] = if self.eactivity { 0x1 } else { 0x0 };

        write_string(&mut buf[0xc5..0xd1], &self.version);
        write_string(&mut buf[0xd1..0xdf], &self.date);

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let localized = Localized {
            english: "english".to_owned(),
            spanish: "spanish".to_owned(),
            german: "german".to_owned(),
            french: "french".to_owned(),
            portuguese: "portuguese".to_owned(),
            chinese: "chinese".to_owned(),
            eactivity: true,
            version: "12.12.1234".to_owned(),
            date: "2021.0330.1250".to_owned(),
        };

        let expected: &[u8] = &[
            101, 110, 103, 108, 105, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            115, 112, 97, 110, 105, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            103, 101, 114, 109, 97, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102,
            114, 101, 110, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 111,
            114, 116, 117, 103, 117, 101, 115, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99,
            104, 105, 110, 101, 115, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101,
            110, 103, 108, 105, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101,
            110, 103, 108, 105, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 49, 50, 46, 49, 50, 46, 49, 50, 51, 52, 0, 0, 50, 48, 50, 49, 46, 48, 51, 51,
            48, 46, 49, 50, 53, 48,
        ];

        let output = localized.serialize();
        assert_eq!(expected, output);
    }
}
