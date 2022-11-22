use std::fmt::{Debug, Display, Formatter, Result};

/// All possible errors returned by this library after the [`desenv::load`] call. The errors are:
///
/// - `MissingVar`: the environment variable could not be found
/// - `NotUnicodeVar`: the environment variable exists but is not UTF-8 encoded.
/// - `ParseFromStr`: the environment variable contains a value that cannot be parsed to original value.
/// - `Custom`: custom error defined by the user of this library. Is mainly used to handle the errors
///             of a custom deserializable type (mixing in the [`std::str::FromStr`] trait).
pub enum Error {
    MissingVar(String),
    NotUnicodeVar(String),
    ParseFromStr(String),
    Custom(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MissingVar(error) => write!(f, "Missing env var `{}`", error),
            Self::NotUnicodeVar(error) => write!(f, "Env var is not unicode `{}`", error),
            Self::ParseFromStr(error) => write!(f, "Cannot parse env var: {}", error),
            Self::Custom(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
