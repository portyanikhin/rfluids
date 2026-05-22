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

/// `CoolProp` error.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum CoolPropError {
    /// Error reported by `CoolProp`.
    #[error("{0}")]
    Native(String),

    /// Non-finite result returned by `CoolProp` without an error message.
    #[error("CoolProp returned a non-finite output without an error message!")]
    NonFiniteOutput,

    /// Non-finite keyed output returned by `CoolProp` without an error message.
    #[error("CoolProp returned a non-finite output for key `{key}` without an error message!")]
    NonFiniteKeyedOutput {
        /// Requested output key.
        key: u8,
    },

    /// Input string cannot be represented as a C string because it contains an interior NUL byte.
    #[error("Input `{arg}` contains an interior NUL byte at byte position {pos}!")]
    InteriorNul {
        /// Name of the invalid input argument.
        arg: &'static str,
        /// Byte position of the interior NUL byte.
        pos: usize,
    },
}

/// A type alias for results returned by `CoolProp` native API.
pub type Result<T> = std::result::Result<T, CoolPropError>;
