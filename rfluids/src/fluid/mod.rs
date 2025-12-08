//! Thermophysical properties of substances.
//!
//! This module provides a comprehensive interface for calculating thermodynamic
//! and transport properties of various substances, including pure fluids and different mixtures
//! (i.e., [`Substance`] or any of its subsets).
//!
//! # Types
//!
//! The core type for managing thermophysical properties of substances is [`Fluid`].
//! It encompasses various state management features and property calculations.
//! The [`Fluid`] struct is generic over the state variant ([`Defined`] or [`Undefined`]).
//! Depending on the state variant, the [`Fluid`] instance has different functionality
//! (e.g., with [`Undefined`] state variant it has only trivial properties available,
//! but with [`Defined`] state variant it supports a full set of property calculations).
//! The [`Fluid`] struct can be created from any [`Pure`], [`IncompPure`], [`PredefinedMix`]
//! or [`BinaryMix`] using the [`From`]/[`Into`] traits; and from any [`Substance`]
//! or [`CustomMix`] using the [`TryFrom`]/[`TryInto`] traits. This is due to the fact
//! that [`CustomMix`] can potentially be unsupported by `CoolProp`.
//! For advanced control over backend selection, use [`Fluid::builder`].

pub mod backend;
mod common;
mod defined;
mod invariant;
mod request;
mod undefined;

use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use backend::Backend;
use request::FluidUpdateRequest;

use crate::{
    io::{FluidParam, FluidTrivialParam},
    native::{AbstractState, CoolPropError},
    state_variant::{Defined, StateVariant, Undefined},
    substance::{BinaryMix, CustomMix, IncompPure, PredefinedMix, Pure, Substance},
};

/// Result type for operations that could fail while updating fluid state.
pub type StateResult<T> = Result<T, FluidStateError>;

/// Result type for operations that could fail while retrieving fluid properties.
pub type OutputResult<T> = Result<T, FluidOutputError>;

/// Provider of thermophysical properties of substances.
///
/// It implements the [Typestate Pattern](https://en.wikipedia.org/wiki/Typestate_analysis)
/// and has one generic type parameter: `S` -- state variant _([`Defined`] or [`Undefined`])_.
///
/// Depending on `S`, the `Fluid` instance has different functionality
/// (e.g., with [`Undefined`] state variant it has only trivial properties available,
/// but with [`Defined`] state variant it supports a full set of property calculations).
///
/// # See Also
///
/// - [Fluid Properties](https://coolprop.org/fluid_properties/index.html)
#[derive(Debug)]
pub struct Fluid<S: StateVariant = Defined> {
    substance: Substance,
    backend: AbstractState,
    requested_backend: Option<Backend>,
    update_request: Option<FluidUpdateRequest>,
    outputs: HashMap<FluidParam, OutputResult<f64>>,
    trivial_outputs: HashMap<FluidTrivialParam, OutputResult<f64>>,
    state: PhantomData<S>,
}

impl TryFrom<Substance> for Fluid<Undefined> {
    type Error = FluidBuildError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from any [`Substance`].
    ///
    /// # Errors
    ///
    /// Returns a [`FluidBuildError`] for unsupported custom mixtures.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = Substance::from(Pure::Water);
    /// assert!(Fluid::try_from(water).is_ok());
    ///
    /// let supported_mix =
    ///     Substance::from(CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])?);
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix =
    ///     Substance::from(CustomMix::mass_based([(Pure::Orthohydrogen, 0.6), (Pure::R32, 0.4)])?);
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
    fn try_from(value: Substance) -> Result<Self, Self::Error> {
        Self::builder().substance(value).build()
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
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
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
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
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
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
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
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
    fn from(value: BinaryMix) -> Self {
        Substance::from(value).try_into().unwrap()
    }
}

impl TryFrom<CustomMix> for Fluid<Undefined> {
    type Error = FluidBuildError;

    /// Creates and returns a new [`Fluid`] instance
    /// with [`Undefined`] state variant from [`CustomMix`].
    ///
    /// # Errors
    ///
    /// Returns a [`FluidBuildError`] for unsupported custom mixtures.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let supported_mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])?;
    /// assert!(Fluid::try_from(supported_mix).is_ok());
    ///
    /// let unsupported_mix = CustomMix::mass_based([(Pure::Orthohydrogen, 0.6), (Pure::R32, 0.4)])?;
    /// assert!(Fluid::try_from(unsupported_mix).is_err());
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Fluid::builder`]
    fn try_from(value: CustomMix) -> Result<Self, Self::Error> {
        Substance::from(value).try_into()
    }
}

/// Error during building of the [`Fluid`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("Unable to build fluid! {0}")]
pub struct FluidBuildError(#[from] CoolPropError);

/// Error during [`Fluid::update`] or [`Fluid::in_state`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
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
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
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
    use strum::IntoEnumIterator;

    use super::*;
    use crate::substance::*;

    #[test]
    fn from_each_pure() {
        for substance in Pure::iter() {
            // Given
            let sut = Fluid::from(substance);

            // When
            let res = size_of_val(&sut);

            // Then
            assert!(res > 0);
        }
    }

    #[test]
    fn from_each_incomp_pure() {
        for substance in IncompPure::iter() {
            // Given
            let sut = Fluid::from(substance);

            // When
            let res = size_of_val(&sut);

            // Then
            assert!(res > 0);
        }
    }

    #[test]
    fn from_each_predefined_mix() {
        for substance in PredefinedMix::iter() {
            // Given
            let sut = Fluid::from(substance);

            // When
            let res = size_of_val(&sut);

            // Then
            assert!(res > 0);
        }
    }

    #[test]
    fn from_each_binary_mix() {
        for kind in BinaryMixKind::iter() {
            // Given
            let sut = Fluid::from(
                kind.with_fraction(0.5 * (kind.min_fraction() + kind.max_fraction())).unwrap(),
            );

            // When
            let res = size_of_val(&sut);

            // Then
            assert!(res > 0);
        }
    }

    #[test]
    fn try_from_supported_custom_mix() {
        // Given
        let supported_mix = CustomMix::mass_based([(Pure::R32, 0.7), (Pure::R134a, 0.3)]).unwrap();

        // When
        let res = Fluid::try_from(supported_mix);

        // Then
        assert!(res.is_ok());
    }

    #[test]
    fn try_from_unsupported_custom_mix() {
        // Given
        let unsupported_mix =
            CustomMix::mole_based([(Pure::Xenon, 0.1), (Pure::R22, 0.9)]).unwrap();

        // When
        let res = Fluid::try_from(unsupported_mix);

        // Then
        assert!(res.is_err());
    }
}
