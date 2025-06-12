//! Implementation of the `CoolProp` native API.
//!
//! This module provides Rust bindings and high-level abstractions
//! for interacting with the `CoolProp` thermophysical and transport properties library.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`CoolProp`] -- high-level API for simplified access to properties.
//! - [`AbstractState`] -- low-level API for direct control and improved performance.

pub use high_level_api::CoolProp;
pub use low_level_api::AbstractState;
use thiserror::Error;

mod common;
mod high_level_api;
mod low_level_api;

/// `CoolProp` internal error.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("{0}")]
pub struct CoolPropError(pub(crate) String);

/// A type alias for results returned by `CoolProp` native API.
pub type Result<T> = std::result::Result<T, CoolPropError>;
