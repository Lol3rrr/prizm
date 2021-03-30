use crate::util::write_string;

#[derive(Debug)]
pub struct Localized {
    english: String,
    spanish: String,
    german: String,
    french: String,
    portuguese: String,
    chinese: String,
    eactivity: bool,
    version: String,
    date: String,
}

impl Localized {
    pub fn parse(content: &[u8]) -> Option<Self> {
        let raw_english = &content[0x006b..0x0083];
        let raw_spanish = &content[0x0083..0x009b];
        let raw_german = &content[0x009b..0x00b3];
        let raw_french = &content[0x00b3..0x00cb];
        let raw_portuguese = &content[0x00cb..0x00e3];
        let raw_chinese = &content[0x00e3..0x00fb];

        let english = String::from_utf8(raw_english.to_vec()).unwrap();
        let spanish = String::from_utf8(raw_spanish.to_vec()).unwrap();
        let german = String::from_utf8(raw_german.to_vec()).unwrap();
        let french = String::from_utf8(raw_french.to_vec()).unwrap();
        let portuguese = String::from_utf8(raw_portuguese.to_vec()).unwrap();
        let chinese = String::from_utf8(raw_chinese.to_vec()).unwrap();

        let raw_eactivity = content[0x012b];
        let eactivity = if raw_eactivity == 0 { false } else { true };

        let raw_version = &content[0x0130..0x013c];
        let raw_date = &content[0x013c..0x014a];

        let version = String::from_utf8(raw_version.to_vec()).unwrap();
        let date = String::from_utf8(raw_date.to_vec()).unwrap();

        Some(Self {
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

    pub fn serialize(&self, buf: &mut [u8]) {
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

        let mut outbuf = [0; 0xdf];

        localized.serialize(&mut outbuf);
        assert_eq!(expected, outbuf);
    }
}
