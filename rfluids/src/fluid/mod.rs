//! Thermophysical and transport properties of substances.
//!
//! This module provides a comprehensive interface for calculating
//! thermophysical and transport properties of various substances,
//! including pure fluids and different mixtures (i.e., [`Substance`] or any of its subset).
//!
//! # Types
//!
//! The core type for managing thermophysical and transport properties of substances is [`Fluid`].
//! It encompasses various state management features and property calculations.
//! The [`Fluid`] struct is generic over the state variant ([`Defined`] or [`Undefined`]).
//! Depending on the state variant, the [`Fluid`] instance has different functionality
//! (e.g., with [`Undefined`] state variant it has only trivial properties available,
//! but with [`Defined`] state variant it supports a full set of property calculations).
//! The [`Fluid`] struct can be created from any [`Pure`], [`IncompPure`], [`PredefinedMix`]
//! or [`BinaryMix`] using the [`From`]/[`Into`] traits; and from any [`Substance`]
//! or [`CustomMix`] using the [`TryFrom`]/[`TryInto`] traits. This is due to the fact that
//! [`CustomMix`] potentially can be unsupported by the `CoolProp`.

mod common;
mod defined;
mod invariant;
mod requests;
mod undefined;

use crate::{
    io::{FluidParam, FluidTrivialParam},
    native::{AbstractState, CoolPropError},
    state_variant::{Defined, StateVariant, Undefined},
    substance::{BinaryMix, CustomMix, IncompPure, PredefinedMix, Pure, Substance},
};
use requests::{FluidCreateRequest, FluidUpdateRequest};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};
use thiserror::Error;

/// Result type for operations that could fail while updating fluid state.
pub type StateResult<T> = Result<T, FluidStateError>;

/// Result type for operations that could fail while retrieving fluid properties.
pub type OutputResult<T> = Result<T, FluidOutputError>;

/// Provider of thermophysical and transport properties of substances.
///
/// It implements the [typestate pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter: `S` -- state variant _([`Defined`] or [`Undefined`])_.
///
/// Depending on `S`, the `Fluid` instance has different functionality
/// (e.g., with [`Undefined`] state variant it has only trivial properties available,
/// but with [`Defined`] state variant it supports a full set of property calculations).
#[derive(Debug)]
pub struct Fluid<S: StateVariant = Defined> {
    substance: Substance,
    backend: AbstractState,
    update_request: Option<FluidUpdateRequest>,
    outputs: HashMap<FluidParam, OutputResult<f64>>,
    trivial_outputs: HashMap<FluidTrivialParam, OutputResult<f64>>,
    state: PhantomData<S>,
}

impl TryFrom<Substance> for Fluid<Undefined> {
    type Error = FluidFromCustomMixError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from any [`Substance`].
    ///
    /// # Errors
    ///
    /// For unsupported custom mixtures, a [`FluidFromCustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let water = Substance::from(Pure::Water);
    /// assert!(Fluid::try_from(water).is_ok());
    ///
    /// let supported_mix = Substance::from(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Water, 0.6),
    ///         (Pure::Ethanol, 0.4),
    ///     ]))?,
    /// );
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = Substance::from(
    ///     CustomMix::mass_based(HashMap::from([
    ///         (Pure::Orthohydrogen, 0.6),
    ///         (Pure::R32, 0.4),
    ///     ]))?,
    /// );
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn try_from(value: Substance) -> Result<Self, Self::Error> {
        let request = FluidCreateRequest::from(&value);
        let mut backend = AbstractState::new(request.backend_name, request.name)?;
        if let Some(fractions) = request.fractions {
            backend.set_fractions(&fractions).unwrap();
        }
        Ok(Self {
            substance: value,
            backend,
            update_request: None,
            outputs: HashMap::new(),
            trivial_outputs: HashMap::new(),
            state: PhantomData,
        })
    }
}

impl From<Pure> for Fluid<Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`Pure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = Fluid::from(Pure::Water);
    /// ```
    fn from(value: Pure) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl From<IncompPure> for Fluid<Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`IncompPure`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let incomp_water = Fluid::from(IncompPure::Water);
    /// ```
    fn from(value: IncompPure) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl From<PredefinedMix> for Fluid<Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`PredefinedMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let r444a = Fluid::from(PredefinedMix::R444A);
    /// ```
    fn from(value: PredefinedMix) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl From<BinaryMix> for Fluid<Undefined> {
    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`BinaryMix`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let propylene_glycol = Fluid::from(BinaryMixKind::MPG.with_fraction(0.4)?);
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn from(value: BinaryMix) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl TryFrom<CustomMix> for Fluid<Undefined> {
    type Error = FluidFromCustomMixError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`CustomMix`].
    ///
    /// # Errors
    ///
    /// For unsupported custom mixtures, a [`FluidFromCustomMixError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let supported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Water, 0.6),
    ///     (Pure::Ethanol, 0.4),
    /// ]))?;
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = CustomMix::mass_based(HashMap::from([
    ///     (Pure::Orthohydrogen, 0.6),
    ///     (Pure::R32, 0.4),
    /// ]))?;
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn try_from(value: CustomMix) -> Result<Self, Self::Error> {
        Substance::from(value).try_into()
    }
}

/// Error during creation of [`Fluid`] from [`CustomMix`].
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidFromCustomMixError {
    /// Specified custom mixture is not supported.
    #[error("Specified custom mixture is not supported! {0}")]
    Unsupported(#[from] CoolPropError),
}

/// Error during [`Fluid::update`] or [`Fluid::in_state`].
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidStateError {
    /// Specified inputs are invalid.
    #[error("Specified inputs (`{0:?}`, `{1:?}`) are invalid!")]
    InvalidInputPair(FluidParam, FluidParam),

    /// Some of the specified input value is infinite or NaN.
    #[error("Input values must be finite!")]
    InvalidInputValue,

    /// Failed to update the fluid state due to unsupported inputs or invalid state.
    #[error("Failed to update the fluid state! {0}")]
    UpdateFailed(#[from] CoolPropError),
}

/// Error during calculation of the [`Fluid`] output parameter value.
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum FluidOutputError {
    /// Specified trivial output parameter is not available.
    #[error("Specified trivial output parameter `{0:?}` is not available!")]
    UnavailableTrivialOutput(FluidTrivialParam),

    /// Specified output parameter is not available.
    #[error("Specified output parameter `{0:?}` is not available!")]
    UnavailableOutput(FluidParam),

    /// Failed to calculate the output parameter value.
    #[error("Failed to calculate the output value of `{0:?}`! {1}")]
    CalculationFailed(FluidParam, CoolPropError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substance::*;
    use strum::IntoEnumIterator;

    #[test]
    fn from_each_pure_does_not_panic() {
        for substance in Pure::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_incomp_pure_does_not_panic() {
        for substance in IncompPure::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_predefined_mix_does_not_panic() {
        for substance in PredefinedMix::iter() {
            let _fluid = Fluid::from(substance);
        }
    }

    #[test]
    fn from_each_binary_mix_does_not_panic() {
        for kind in BinaryMixKind::iter() {
            let _fluid = Fluid::from(
                kind.with_fraction(0.5 * (kind.min_fraction() + kind.max_fraction()))
                    .unwrap(),
            );
        }
    }

    #[test]
    fn try_from_supported_custom_mix_returns_ok() {
        let supported_mix =
            CustomMix::mass_based(HashMap::from([(Pure::R32, 0.7), (Pure::R134a, 0.3)])).unwrap();
        assert!(Fluid::try_from(supported_mix).is_ok());
    }

    #[test]
    fn try_from_unsupported_custom_mix_returns_err() {
        let unsupported_mix =
            CustomMix::mole_based(HashMap::from([(Pure::Xenon, 0.1), (Pure::R22, 0.9)])).unwrap();
        assert!(Fluid::try_from(unsupported_mix).is_err());
    }
}
