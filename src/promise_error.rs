use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    text: String
}

impl Error {
    fn new(text: String) -> Self {
        Self {text }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl std::error::Error for Error {

}
