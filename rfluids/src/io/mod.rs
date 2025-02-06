//! `CoolProp` inputs/outputs.
//!
//! This module provides functionality for handling `CoolProp`-related
//! input/output operations. It includes types and utilities for working
//! with fluid parameters, humid air parameters, and phase states.
//!
//! # Types
//!
//! The main types provided by this module are:
//!
//! - [`FluidParam`] -- non-trivial fluid parameters
//!   (e.g., pressure, temperature, etc.).
//! - [`FluidTrivialParam`] -- trivial fluid parameters
//!   (e.g., molar mass, critical pressure, etc.).
//! - [`FluidInputPair`] -- valid combinations of two fluid parameters
//!   (e.g., pressure-temperature, pressure-enthalpy, etc.)
//!   used to define a thermodynamic state.
//! - [`FluidInput`] -- fluid input parameter with specified value.
//! - [`HumidAirParam`] -- humid air parameters.
//! - [`Phase`] -- phase state of the fluid.

pub use fluid_param::*;
pub use humid_air_param::*;
pub use input::*;
pub use input_pair::*;
pub use phase::*;

mod fluid_param;
mod humid_air_param;
mod input;
mod input_pair;
mod phase;

pub(crate) fn try_from<T: TryFrom<u8, Error = strum::ParseError>>(
    value: f64,
) -> Result<T, strum::ParseError> {
    let val = value.trunc();
    if val < f64::from(u8::MIN) || val > f64::from(u8::MAX) {
        return Err(strum::ParseError::VariantNotFound);
    }
    T::try_from(val as u8)
}
