//! 🦀 Rusty [CoolProp](https://coolprop.github.io/CoolProp/) wrapper.
//!
//! ## Supported platforms
//!
//! - `Windows x86-64`
//! - `Windows AArch64`
//! - `Linux x86-64`
//! - `macOS x86-64`
//! - `macOS AArch64`
//!
//! ## How to install
//!
//! Run the following command in your project directory:
//!
//! ```shell
//! cargo add rfluids
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
pub mod humid_air;
pub mod io;
pub mod native;
mod ops;
pub mod prelude;
pub mod state_variant;
pub mod substance;
#[cfg(test)]
mod test;
pub mod uom_ext;
