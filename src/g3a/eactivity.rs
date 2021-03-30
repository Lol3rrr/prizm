use crate::util::write_string;

#[derive(Debug)]
pub struct EActivity {
    english: String,
    spanish: String,
    german: String,
    french: String,
    portuguese: String,
    chinese: String,
    icon: Vec<u8>,
}

impl EActivity {
    pub fn parse(content: &[u8]) -> Option<Self> {
        let raw_english = &content[0x0170..0x0194];
        let raw_spanish = &content[0x0194..0x01b8];
        let raw_german = &content[0x01b8..0x01dc];
        let raw_french = &content[0x01dc..0x0200];
        let raw_portuguese = &content[0x0200..0x0224];
        let raw_chinese = &content[0x0224..0x0248];

        let english = String::from_utf8(raw_english.to_vec()).unwrap();
        let spanish = String::from_utf8(raw_spanish.to_vec()).unwrap();
        let german = String::from_utf8(raw_german.to_vec()).unwrap();
        let french = String::from_utf8(raw_french.to_vec()).unwrap();
        let portuguese = String::from_utf8(raw_portuguese.to_vec()).unwrap();
        let chinese = String::from_utf8(raw_chinese.to_vec()).unwrap();

        let raw_icon = &content[0x0290..0x0590];

        Some(Self {
            english,
            spanish,
            german,
            french,
            portuguese,
            chinese,
            icon: raw_icon.to_vec(),
        })
    }

    pub fn serialize(&self) -> [u8; 0x420] {
        let mut result = [0; 0x420];

        write_string(&mut result[0x0..], &self.english);
        write_string(&mut result[0x24..], &self.spanish);
        write_string(&mut result[0x48..], &self.german);
        write_string(&mut result[0x6c..], &self.french);
        write_string(&mut result[0x90..], &self.portuguese);
        write_string(&mut result[0xb4..], &self.chinese);

        // Reserved, filled with english in meantime
        write_string(&mut result[0xd8..], &self.english);
        write_string(&mut result[0xfc..], &self.english);

        &result[0x120..].copy_from_slice(&self.icon);

        result
    }
}
