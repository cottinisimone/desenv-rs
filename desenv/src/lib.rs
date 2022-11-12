use crate::error::Error;

mod error;

pub trait Desenv {
    /// Load the configuration
    ///
    /// # Errors
    /// -
    fn load() -> Result<Self, Error>
    where
        Self: Sized;
}
