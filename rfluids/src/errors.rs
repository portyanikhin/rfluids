//! Error handling.

use std::any::type_name;
use thiserror::Error;

/// Enum parsing error.
#[derive(Error, Debug, Clone)]
#[error("'{key}' has no matching '{target}'!")]
pub struct EnumParseError {
    key: String,
    target: &'static str,
}

impl EnumParseError {
    pub(crate) fn new<T>(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            target: type_name::<T>().split("::").last().unwrap(),
        }
    }
}

/// CoolProp internal error.
#[derive(Error, Debug, Clone)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);
