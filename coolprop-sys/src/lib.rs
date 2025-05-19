//! [<img alt="GitHub" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/portyanikhin/rfluids)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/coolprop-sys)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/coolprop-sys?style=for-the-badge&logo=rust&labelColor=555555&color=fc8d62" height="22">](https://crates.io/crates/coolprop-sys)
//! [<img alt="CI" src="https://img.shields.io/github/actions/workflow/status/portyanikhin/rfluids/ci.yml?style=for-the-badge&logo=githubactions&logoColor=ffffff&label=ci&labelColor=555555" height="22">](https://github.com/portyanikhin/rfluids/actions/workflows/ci.yml)
//!
//! Raw FFI bindings to [CoolProp](https://coolprop.github.io/CoolProp/).
//!
//! ## Supported platforms
//!
//! - `Linux x86-64`
//! - `macOS AArch64`
//! - `macOS x86-64`
//! - `Windows AArch64`
//! - `Windows x86-64`
//!
//! ## MSRV
//!
//! `coolprop-sys` requires `rustc` 1.85.0 or later.
//!
//! ## How to install
//!
//! Run the following command in your project directory:
//!
//! ```shell
//! cargo add coolprop-sys
//! ```
//!
//! 🎁 It comes with native `CoolProp` dynamic libraries for supported platforms.
//! The library required for your platform will be automatically copied
//! to the target directory during build.
//!
//! #### License
//!
//! <sup>
//! This project is licensed under
//! <a href="https://github.com/portyanikhin/rfluids/blob/main/LICENSE">MIT License</a>.
//! </sup>

pub mod bindings;

/// `CoolProp` dynamic library absolute path.
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const COOLPROP_PATH: &str = coolprop_sys_windows_x86_64::COOLPROP_PATH;
#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const COOLPROP_PATH: &str = coolprop_sys_windows_aarch64::COOLPROP_PATH;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const COOLPROP_PATH: &str = coolprop_sys_linux_x86_64::COOLPROP_PATH;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const COOLPROP_PATH: &str = coolprop_sys_macos_x86_64::COOLPROP_PATH;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const COOLPROP_PATH: &str = coolprop_sys_macos_aarch64::COOLPROP_PATH;
