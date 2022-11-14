use crate::error::Error;

mod error;

/// Load the configuration
///
/// # Errors
/// -
pub fn load<T>() -> Result<T, Error>
where
    T: Desenv,
{
    T::_load()
}

pub trait Desenv {
    /// Load the configuration
    ///
    /// # Errors
    /// -
    fn _load() -> Result<Self, Error>
    where
        Self: Sized;
}
