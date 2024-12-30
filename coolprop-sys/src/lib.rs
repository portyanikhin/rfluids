#![doc = include_str!("../README.md")]

pub mod bindings;

/// CoolProp dynamic library absolute path.
#[cfg(target_os = "windows")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/CoolProp.dll");
#[cfg(target_os = "linux")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/libCoolProp.so");
#[cfg(target_os = "macos")]
pub const COOLPROP_PATH: &str = concat!(env!("OUT_DIR"), "/libCoolProp.dylib");
