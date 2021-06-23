#[derive(Debug, PartialEq, Clone)]
pub enum Modifier {
    Shift,
    Alpha,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Key {
    Menu,
    Exit,
    Exe,
    Del,
    Ac,
    Number(u8),
    Character(char),
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

impl Key {
    pub fn serialize(&self, modifier: &Modifier) -> u16 {
        match modifier {
            Modifier::None => match self {
                Self::Menu => 0x7533,
                Self::Exit => 0x7532,
                Self::Exe => 0x7534,
                Self::Ac => 0x753F,
                Self::Number(val) => 0x0030 | ((val & 0x0f) as u16),
                Self::ArrowUp => 0x7542,
                Self::ArrowDown => 0x7547,
                Self::ArrowLeft => 0x7544,
                Self::ArrowRight => 0x7545,
                _ => unimplemented!("Unknown Input: {:?}", self),
            },
            _ => unimplemented!("Unknown Modifer: {:?}", modifier),
        }
    }
}
