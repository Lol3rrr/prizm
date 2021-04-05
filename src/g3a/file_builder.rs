use super::{eactivity, image, localization, File};

use chrono::NaiveDateTime;

pub struct FileBuilder {
    internal_name: Option<String>,
    short_name: Option<String>,
    selected: Option<image::Image>,
    unselected: Option<image::Image>,
    localized: localization::Localized,
    eactivity: Option<eactivity::EActivity>,
    code: Vec<u8>,
}

// TODO
// Add options for localization stuff
impl FileBuilder {
    pub fn new(name: String, creation_date: NaiveDateTime) -> Self {
        let date_string = creation_date.format("%Y.%m%d.%H%M").to_string();

        Self {
            internal_name: None,
            short_name: None,
            selected: None,
            unselected: None,
            localized: localization::Localized {
                english: name.clone(),
                spanish: name.clone(),
                german: name.clone(),
                french: name.clone(),
                portuguese: name.clone(),
                chinese: name,
                eactivity: false,
                version: "01.00.0000".to_string(),
                date: date_string,
            },
            eactivity: None,
            code: Vec::new(),
        }
    }

    pub fn internal_name(&mut self, n_name: String) -> &mut Self {
        self.internal_name = Some(n_name);
        self
    }
    pub fn short_name(&mut self, n_short: String) -> &mut Self {
        self.short_name = Some(n_short);
        self
    }
    pub fn selected_image(&mut self, n_image: image::Image) -> &mut Self {
        self.selected = Some(n_image);
        self
    }
    pub fn unselected_image(&mut self, n_image: image::Image) -> &mut Self {
        self.unselected = Some(n_image);
        self
    }
    pub fn selected_image_path(&mut self, path: &str) -> &mut Self {
        self.selected = image::Image::from_file(path);
        self
    }
    pub fn unselected_image_path(&mut self, path: &str) -> &mut Self {
        self.unselected = image::Image::from_file(path);
        self
    }
    pub fn code(&mut self, n_code: Vec<u8>) -> &mut Self {
        self.code = n_code;
        self
    }

    pub fn finish(self) -> File {
        let eactivity = self.eactivity.unwrap_or_else(eactivity::EActivity::empty);
        let selected = self.selected.unwrap_or_else(image::Image::empty);
        let unselected = self.unselected.unwrap_or_else(image::Image::empty);

        let file_size = 0x7000 + 0x4 + self.code.len() as u32;

        File {
            internal_name: self.internal_name.unwrap(),
            short_name: self.short_name.unwrap(),
            file_size,
            selected_image: selected,
            unselected_image: unselected,
            executable_code: self.code,
            localized: self.localized,
            eactivity,
        }
    }
}
