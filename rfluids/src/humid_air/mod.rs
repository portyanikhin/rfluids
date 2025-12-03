//! Thermophysical and transport properties of humid air.
//!
//! This module provides a comprehensive interface for calculating
//! thermophysical and transport properties of humid air
//! through the [`HumidAir`] struct.

mod common;
mod defined;
mod invariant;
mod request;
mod undefined;

use std::{collections::HashMap, marker::PhantomData};

use request::HumidAirUpdateRequest;

use crate::{
    io::HumidAirParam,
    native::CoolPropError,
    state_variant::{Defined, StateVariant},
};

/// Result type for operations that could fail
/// while updating humid air state.
pub type StateResult<T> = Result<T, HumidAirStateError>;

/// Result type for operations that could fail
/// while retrieving humid air properties.
pub type OutputResult<T> = Result<T, HumidAirOutputError>;

/// Provider of thermophysical and transport properties of humid air.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter: `S` -- state variant
/// _([`Defined`] or [`Undefined`](crate::state_variant::Undefined))_.
///
/// Depending on `S`, the `HumidAir`
/// instance has different functionality.
#[derive(Debug)]
pub struct HumidAir<S: StateVariant = Defined> {
    update_request: Option<HumidAirUpdateRequest>,
    outputs: HashMap<HumidAirParam, OutputResult<f64>>,
    state: PhantomData<S>,
}

/// Error during [`HumidAir::update`] or [`HumidAir::in_state`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HumidAirStateError {
    /// Specified inputs are invalid.
    #[error("Specified inputs (`{0:?}`, `{1:?}`, `{2:?}`) are invalid!")]
    InvalidInputs(HumidAirParam, HumidAirParam, HumidAirParam),

    /// Some of the specified input value is infinite or NaN.
    #[error("Input values must be finite!")]
    InvalidInputValue,
}

/// Error during calculation
/// of the [`HumidAir`] output parameter value.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HumidAirOutputError {
    /// Specified output parameter is not available.
    #[error("Specified output parameter `{0:?}` is not available!")]
    UnavailableOutput(HumidAirParam),

    /// Failed to calculate the output parameter value.
    #[error("Failed to calculate the output value of `{0:?}`! {1}")]
    CalculationFailed(HumidAirParam, CoolPropError),
}
