//! Raw FFI bindings to `CoolProp`.
//!
//! # See also
//!
//! - [`CoolPropLib.h` reference](https://coolprop.github.io/CoolProp/_static/doxygen/html/_cool_prop_lib_8h.html)

#![allow(
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clippy::pedantic
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
