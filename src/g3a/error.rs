#[derive(Debug)]
pub enum ParseError {
    WrongIdentifier,
    WrongFormat,
    MismatchedChecksums,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongIdentifier => write!(f, "Wrong-Identifier"),
            Self::WrongFormat => write!(f, "Wrong-Format"),
            Self::MismatchedChecksums => write!(f, "Checksums are not matching"),
        }
    }
}
