//! Simple, full-featured, lightweight, cross-platform
//! [CoolProp](https://coolprop.github.io/CoolProp/) wrapper for Rust.
//!
//! ## Supported platforms
//!
//! - Windows x64
//! - Linux x64
//! - macOS x64
//! - macOS ARM64
//!
//! ## How to install
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rfluids = "0.1.0-alpha"
//! ```
//!
//! **NB.** It comes with native `CoolProp` dynamic libraries for supported platforms.
//! The library required for your platform will be automatically
//! copied to the target directory during build.
//!
//! ## License
//!
//! This project is licensed under [MIT License](https://github.com/portyanikhin/rfluids/blob/main/LICENSE).

#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::missing_panics_doc
)]

pub extern crate uom;

pub mod error;
pub mod fluid;
pub mod io;
pub mod native;
pub mod prelude;
pub mod state_variant;
pub mod substance;
