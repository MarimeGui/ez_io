use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MagicNumberCheckError {
    IoError(io::Error),
    MagicNumber(WrongMagicNumber),
}

impl Error for MagicNumberCheckError {
    fn description(&self) -> &str {
        match *self {
            MagicNumberCheckError::IoError(ref e) => e.description(),
            MagicNumberCheckError::MagicNumber(ref e) => e.description(),
        }
    }
}

impl fmt::Display for MagicNumberCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MagicNumberCheckError::IoError(ref e) => e.fmt(f),
            MagicNumberCheckError::MagicNumber(ref e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for MagicNumberCheckError {
    fn from(e: io::Error) -> MagicNumberCheckError {
        MagicNumberCheckError::IoError(e)
    }
}

impl From<WrongMagicNumber> for MagicNumberCheckError {
    fn from(e: WrongMagicNumber) -> MagicNumberCheckError {
        MagicNumberCheckError::MagicNumber(e)
    }
}

#[derive(Debug)]
pub struct WrongMagicNumber {
    pub expected: Vec<u8>,
    pub read: Vec<u8>,
}

impl Error for WrongMagicNumber {
    fn description(&self) -> &str {
        "A Magic Number check Failed"
    }
}

impl fmt::Display for WrongMagicNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expected_string =
            String::from_utf8(self.expected.clone()).unwrap_or(format!("{:X?}", self.expected));
        let read_string =
            String::from_utf8(self.read.clone()).unwrap_or(format!("{:X?}", self.read));
        write!(
            f,
            "Incorrect Magic Number: Expected '{}', Read '{}'",
            expected_string, read_string
        )
    }
}
