//! Raw FFI bindings to [CoolProp](https://coolprop.github.io/CoolProp/).
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
//! cargo add coolprop-sys
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
