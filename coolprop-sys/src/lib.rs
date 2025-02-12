//! Raw FFI bindings to [CoolProp](https://coolprop.github.io/CoolProp/).
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
//! coolprop-sys = "0.1.0-alpha"
//! ```
//!
//! **NB.** It comes with native `CoolProp` dynamic libraries for supported platforms.
//! The library required for your platform will be automatically
//! copied to the target directory during build.
//!
//! ## License
//!
//! This project is licensed under [MIT License](https://github.com/portyanikhin/rfluids/blob/main/LICENSE).

pub mod bindings;

/// `CoolProp` dynamic library absolute path.
#[cfg(target_os = "windows")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/CoolProp.dll");
#[cfg(target_os = "linux")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/libCoolProp.so");
#[cfg(target_os = "macos")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/libCoolProp.dylib");
