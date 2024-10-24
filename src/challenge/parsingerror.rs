use std::fmt::Display;

#[derive(Debug)]
pub enum ParsingError {
    MissmatchedMagicBytes { expected: [u8; 3], found: [u8; 3] },
    UnexpectedEnd,
    UnexpectedError(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::MissmatchedMagicBytes { expected, found } => {
                write!(
                    f,
                    "Missmatched magic bytes, expected: {:X?}, found: {:X?}",
                    expected, found
                )
            }
            ParsingError::UnexpectedEnd => {
                write!(f, "Unexpected end of data")
            }
            ParsingError::UnexpectedError(data) => {
                write!(f, "Unexpected error: {}", data)
            }
        }
    }
}
