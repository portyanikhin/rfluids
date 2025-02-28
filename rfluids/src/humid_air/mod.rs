//! Thermophysical and transport properties of humid air.
//!
//! This module provides a comprehensive interface for calculating
//! thermophysical and transport properties of humid air.

mod common;
mod defined;
mod invariant;
mod requests;
mod undefined;

use crate::error::{HumidAirOutputError, HumidAirStateError};
use crate::io::HumidAirParam;
use crate::state_variant::{Defined, StateVariant};
use requests::HumidAirUpdateRequest;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Result type for operations that could fail while updating humid air state.
pub type StateResult<T> = Result<T, HumidAirStateError>;

/// Result type for operations that could fail while retrieving humid air properties.
pub type OutputResult<T> = Result<T, HumidAirOutputError>;

/// Provider of thermophysical and transport properties of humid air.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter: `S` -- state variant
/// _([`Defined`] or [`Undefined`](crate::state_variant::Undefined))_.
///
/// Depending on `S`, the `HumidAir` instance has different functionality.
#[derive(Debug)]
pub struct HumidAir<S: StateVariant = Defined> {
    update_request: Option<HumidAirUpdateRequest>,
    outputs: HashMap<HumidAirParam, OutputResult<f64>>,
    state: PhantomData<S>,
}
