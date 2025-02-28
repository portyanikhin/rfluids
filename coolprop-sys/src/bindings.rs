//! Raw FFI bindings to `CoolProp`.

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
