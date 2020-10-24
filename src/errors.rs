use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GmockSedError {
    ParseSignatureError,
    UnmatchedParenthesisError,
}

impl fmt::Display for GmockSedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for GmockSedError {}
