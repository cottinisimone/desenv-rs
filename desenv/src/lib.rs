//! Desenv-rs is a library used to deserialize the environment variables into a given struct deriving
//! `Desenv` macro.

pub use desenv_macros::Desenv;
pub use error::Error;

mod error;

/// Load all the environment variables into a given `Desenv` struct.
///
/// # Errors
///
/// Will return `Err` if:
/// - One environment variable for non-optional field is missing (and no default is set).
/// - Deserialization from string of the resulting type fails.
/// - Deserialization of default value from string to resulting type fails.
/// - Both environment variable for non-optional field and default environment variable
///   is missing.
pub fn load<T>() -> Result<T, Error>
where
    T: Desenv,
{
    T::_load(None)
}

pub trait Desenv {
    /// Load the configuration with the given optional `parent_prefix`.
    /// DO NOT USE THIS FUNCTION! Use [`desenv::load`] instead!
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - One environment variable for non-optional field is missing (and no default is set).
    /// - Deserialization from string of the resulting type fails.
    /// - Deserialization of default value from string to resulting type fails.
    /// - Both environment variable for non-optional field and default environment variable
    ///   is missing.
    fn _load(parent_prefix: Option<String>) -> Result<Self, Error>
    where
        Self: Sized;
}
