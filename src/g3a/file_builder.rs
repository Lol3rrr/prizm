use super::{eactivity, image, localization, File};

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
    pub fn new(name: String) -> Self {
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
                chinese: name.clone(),
                eactivity: false,
                version: "00.00.0000".to_string(),
                date: "0000.0000.0000".to_string(),
            },
            eactivity: None,
            code: Vec::new(),
        }
    }

    pub fn internal_name<'a>(&'a mut self, n_name: String) -> &'a mut Self {
        self.internal_name = Some(n_name);
        self
    }
    pub fn short_name<'a>(&'a mut self, n_short: String) -> &'a mut Self {
        self.short_name = Some(n_short);
        self
    }
    pub fn selected_image_path<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.selected = image::Image::from_file(path);
        self
    }
    pub fn unselected_image_path<'a>(&'a mut self, path: &str) -> &'a mut Self {
        self.unselected = image::Image::from_file(path);
        self
    }
    pub fn code<'a>(&'a mut self, n_code: Vec<u8>) -> &'a mut Self {
        self.code = n_code;
        self
    }

    pub fn finish(self) -> File {
        let eactivity = self.eactivity.unwrap_or(eactivity::EActivity::empty());
        let selected = self.selected.unwrap_or(image::Image::empty());
        let unselected = self.unselected.unwrap_or(image::Image::empty());

        File {
            internal_name: self.internal_name.unwrap(),
            short_name: self.short_name.unwrap(),
            file_size: 0,
            selected_image: selected,
            unselected_image: unselected,
            executable_code: self.code,
            localized: self.localized,
            eactivity,
        }
    }
}