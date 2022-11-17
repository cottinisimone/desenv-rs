use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    MissingVar(String),
    NotUnicodeVar(String),
    ParseFromStr(String),
    Custom(String),
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
