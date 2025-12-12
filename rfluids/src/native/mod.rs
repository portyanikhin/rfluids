//! Implementation of the `CoolProp` native API.
//!
//! This module provides Rust bindings and high-level abstractions for interacting
//! with the `CoolProp` thermophysical properties library.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`CoolProp`] -- high-level API for simplified access to properties
//! - [`AbstractState`] -- low-level API for direct control and improved performance

mod common;
mod high_level_api;
mod low_level_api;
mod utils;

pub use high_level_api::CoolProp;
pub use low_level_api::AbstractState;
pub use utils::ConfigValue;

/// `CoolProp` internal error.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);

impl Default for CoolPropError {
    fn default() -> Self {
        Self("Unknown CoolProp error".into())
    }
}

/// A type alias for results returned by `CoolProp` native API.
pub type Result<T> = std::result::Result<T, CoolPropError>;

#[cfg(test)]
mod tests {
    use super::*;

    mod coolprop_error {
        use super::*;

        #[test]
        fn default() {
            // Given
            let sut = CoolPropError::default();

            // When
            let res = sut.to_string();

            // Then
            assert_eq!(res, "Unknown CoolProp error");
        }
    }
}
