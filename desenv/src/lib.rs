pub use desenv_macros::Desenv;
pub use error::Error;

mod error;

/// Load the configuration
///
/// # Errors
/// -
pub fn load<T>() -> Result<T, Error>
where
    T: Desenv,
{
    T::_load(None)
}

pub trait Desenv {
    /// Load the configuration
    ///
    /// # Errors
    /// -
    fn _load(parent_prefix: Option<String>) -> Result<Self, Error>
    where
        Self: Sized;
}
