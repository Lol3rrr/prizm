use crate::{eactivity::EActivityError, localization::LocalizationError};

#[derive(Debug)]
pub enum ParseError {
    WrongIdentifier,
    WrongFormat,
    MismatchedChecksums,
    EActivity(EActivityError),
    Localization(LocalizationError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongIdentifier => write!(f, "Wrong-Identifier"),
            Self::WrongFormat => write!(f, "Wrong-Format"),
            Self::MismatchedChecksums => write!(f, "Checksums are not matching"),
            Self::EActivity(err) => write!(f, "{:?}", err),
            Self::Localization(err) => write!(f, "{:?}", err),
        }
    }
}

impl From<EActivityError> for ParseError {
    fn from(other: EActivityError) -> Self {
        ParseError::EActivity(other)
    }
}
impl From<LocalizationError> for ParseError {
    fn from(other: LocalizationError) -> Self {
        ParseError::Localization(other)
    }
}
