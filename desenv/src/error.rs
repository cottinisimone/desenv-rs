use std::fmt::{Debug, Formatter, Result};

pub enum Error {}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error")
    }
}
