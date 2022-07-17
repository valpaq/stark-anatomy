// use crate::string::String;
use core::fmt;

// DESERIALIZATION ERROR
// ================================================================================================
pub use std::string::{String, ToString};

/// Defines errors which can occur during deserialization.
#[derive(Debug, PartialEq, Eq)]
pub enum DeserializationError {
    /// Bytes in the input do not represent a valid value.
    InvalidValue(String),
    /// An end of input was reached before a valid value could be deserialized.
    UnexpectedEOF,
    /// Deserialization has finished but not all bytes have been consumed.
    UnconsumedBytes,
    /// An unknown error has occurred.
    UnknownError(String),
}

impl fmt::Display for DeserializationError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(err_msg) => {
                write!(f, "{}", err_msg)
            }
            Self::UnexpectedEOF => {
                write!(f, "unexpected EOF")
            }
            Self::UnconsumedBytes => {
                write!(f, "not all bytes were consumed")
            }
            Self::UnknownError(err_msg) => {
                write!(f, "unknown error: {}", err_msg)
            }
        }
    }
}
